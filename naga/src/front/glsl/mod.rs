/*!
Frontend for [GLSL][glsl] (OpenGL Shading Language).

To begin, take a look at the documentation for the [`Frontend`].

# Supported versions
## Vulkan
- 440 (partial)
- 450
- 460

[glsl]: https://www.khronos.org/registry/OpenGL/index_gl.php
*/

pub use ast::{Precision, Profile};
pub use error::{Error, ErrorKind, ExpectedToken, ParseErrors};
pub use token::TokenValue;

use alloc::{string::String, vec, vec::Vec};

use crate::{proc::Layouter, FastHashMap, FastHashSet, Handle, Module, ShaderStage, Span, Type};
use ast::{EntryArg, FunctionDeclaration, GlobalLookup};
use parser::{ParsePhase, ParsingContext};

mod ast;
mod builtins;
mod context;
mod error;
mod functions;
mod lex;
mod offset;
mod parser;
#[cfg(test)]
mod parser_tests;
mod token;
mod types;
mod variables;

type Result<T> = core::result::Result<T, Error>;

/// Per-shader options passed to [`parse`](Frontend::parse).
///
/// The [`From`] trait is implemented for [`ShaderStage`] to provide a quick way
/// to create an `Options` instance.
///
/// ```rust
/// # use naga::ShaderStage;
/// # use naga::front::glsl::Options;
/// Options::from(ShaderStage::Vertex);
/// ```
#[derive(Debug)]
pub struct Options {
    /// The shader stage in the pipeline.
    pub stage: ShaderStage,
    /// Preprocessor definitions to be used, akin to having
    /// ```glsl
    /// #define key value
    /// ```
    /// for each key value pair in the map.
    pub defines: FastHashMap<String, String>,
}

impl From<ShaderStage> for Options {
    fn from(stage: ShaderStage) -> Self {
        Options {
            stage,
            defines: FastHashMap::default(),
        }
    }
}

/// Additional information about the GLSL shader.
///
/// Stores additional information about the GLSL shader which might not be
/// stored in the shader [`Module`].
#[derive(Debug)]
pub struct ShaderMetadata {
    /// The GLSL version specified in the shader through the use of the
    /// `#version` preprocessor directive.
    pub version: u16,
    /// The GLSL profile specified in the shader through the use of the
    /// `#version` preprocessor directive.
    pub profile: Profile,
    /// The shader stage in the pipeline, passed to the [`parse`](Frontend::parse)
    /// method via the [`Options`] struct.
    pub stage: ShaderStage,

    /// The workgroup size for compute shaders, defaults to `[1; 3]` for
    /// compute shaders and `[0; 3]` for non compute shaders.
    pub workgroup_size: [u32; 3],
    /// Whether or not early fragment tests where requested by the shader.
    /// Defaults to `false`.
    pub early_fragment_tests: bool,

    /// The shader can request extensions via the
    /// `#extension` preprocessor directive, in the directive a behavior
    /// parameter is used to control whether the extension should be disabled,
    /// warn on usage, enabled if possible or required.
    ///
    /// This field only stores extensions which were required or requested to
    /// be enabled if possible and they are supported.
    pub extensions: FastHashSet<String>,
}

impl ShaderMetadata {
    fn reset(&mut self, stage: ShaderStage) {
        self.version = 0;
        self.profile = Profile::Core;
        self.stage = stage;
        self.workgroup_size = [u32::from(stage.compute_like()); 3];
        self.early_fragment_tests = false;
        self.extensions.clear();
    }
}

impl Default for ShaderMetadata {
    fn default() -> Self {
        ShaderMetadata {
            version: 0,
            profile: Profile::Core,
            stage: ShaderStage::Vertex,
            workgroup_size: [0; 3],
            early_fragment_tests: false,
            extensions: FastHashSet::default(),
        }
    }
}

/// The `Frontend` is the central structure of the GLSL frontend.
///
/// To instantiate a new `Frontend` the [`Default`] trait is used, so a
/// call to the associated function [`Frontend::default`](Frontend::default) will
/// return a new `Frontend` instance.
///
/// To parse a shader simply call the [`parse`](Frontend::parse) method with a
/// [`Options`] struct and a [`&str`](str) holding the glsl code.
///
/// The `Frontend` also provides the [`metadata`](Frontend::metadata) to get some
/// further information about the previously parsed shader, like version and
/// extensions used (see the documentation for
/// [`ShaderMetadata`] to see all the returned information)
///
/// # Example usage
/// ```rust
/// use naga::ShaderStage;
/// use naga::front::glsl::{Frontend, Options};
///
/// let glsl = r#"
///     #version 450 core
///
///     void main() {}
/// "#;
///
/// let mut frontend = Frontend::default();
/// let options = Options::from(ShaderStage::Vertex);
/// frontend.parse(&options, glsl);
/// ```
///
/// # Reusability
///
/// If there's a need to parse more than one shader reusing the same `Frontend`
/// instance may be beneficial since internal allocations will be reused.
///
/// Calling the [`parse`](Frontend::parse) method multiple times will reset the
/// `Frontend` so no extra care is needed when reusing.
#[derive(Debug, Default)]
pub struct Frontend {
    meta: ShaderMetadata,

    lookup_function: FastHashMap<String, FunctionDeclaration>,
    lookup_type: FastHashMap<String, Handle<Type>>,

    global_variables: Vec<(String, GlobalLookup)>,

    entry_args: Vec<EntryArg>,

    layouter: Layouter,

    errors: Vec<Error>,
}

impl Frontend {
    fn reset(&mut self, stage: ShaderStage) {
        self.meta.reset(stage);

        self.lookup_function.clear();
        self.lookup_type.clear();
        self.global_variables.clear();
        self.entry_args.clear();
        self.layouter.clear();
    }

    /// Parses a shader either outputting a shader [`Module`] or a list of
    /// [`Error`]s.
    ///
    /// Uses a two-phase parse to support forward declarations (GLSL spec 6.1):
    /// - Phase 1 collects all function signatures, registering definitions as prototypes
    /// - Phase 2 re-lexes and parses function bodies, with all signatures already known
    ///
    /// Multiple calls using the same `Frontend` and different shaders are supported.
    pub fn parse(
        &mut self,
        options: &Options,
        source: &str,
    ) -> core::result::Result<Module, ParseErrors> {
        self.reset(options.stage);

        match self.parse_inner(options, source) {
            Ok(module) => {
                if self.errors.is_empty() {
                    Ok(module)
                } else {
                    Err(core::mem::take(&mut self.errors).into())
                }
            }
            Err(e) => {
                self.errors.push(e);
                Err(core::mem::take(&mut self.errors).into())
            }
        }
    }

    fn parse_inner(
        &mut self,
        options: &Options,
        source: &str,
    ) -> Result<Module> {
        use ast::FunctionKind;

        let mut module = Module::default();
        let mut global_expression_kind_tracker = crate::proc::ExpressionKindTracker::new();

        // Phase 1: Collect all function signatures (skip function bodies).
        // Also processes globals, types, structs, and their initializers.
        // We save the global init state (body, expressions, tracker) because
        // add_entry_point needs the global init body to be spliced into the
        // entry point before calling main.
        let (global_init_body, global_init_expressions, global_init_local_tracker) = {
            let lexer = lex::Lexer::new(source, &options.defines);
            let mut parser = ParsingContext::new(lexer, ParsePhase::CollectSignatures);
            let ctx = parser.parse(self, &mut module, &mut global_expression_kind_tracker)?;
            (ctx.body, ctx.expressions, ctx.local_expression_kind_tracker)
        };

        // Phase 2: Parse function bodies (all signatures now known).
        // Skips non-function declarations since they were processed in phase 1.
        // The global Context from phase 2 is not needed — function definitions
        // create their own Contexts, and the global init state is from phase 1.
        {
            let lexer = lex::Lexer::new(source, &options.defines);
            let mut parser = ParsingContext::new(lexer, ParsePhase::ParseBodies);
            parser.parse(self, &mut module, &mut global_expression_kind_tracker)?;
        }

        // Reorder functions in topological order (callees before callers) so that
        // the naga IR validator's forward-dependency check is satisfied.
        sort_functions_topologically(&mut module, &mut self.lookup_function);

        // Build a fresh Context for add_entry_point, restoring phase 1's global
        // init state (body, expressions, tracker) which gets spliced into the
        // entry point before the call to main.
        let mut ctx = context::Context::new(
            self,
            &mut module,
            false,
            &mut global_expression_kind_tracker,
        )?;
        ctx.body = global_init_body;
        ctx.expressions = global_init_expressions;
        ctx.local_expression_kind_tracker = global_init_local_tracker;

        // Add an `EntryPoint` to the module for `main`, if a
        // suitable overload exists. Error out if we can't find one.
        if let Some(declaration) = self.lookup_function.get("main") {
            for decl in declaration.overloads.iter() {
                if let FunctionKind::Call(handle) = decl.kind {
                    if decl.defined && decl.parameters.is_empty() {
                        self.add_entry_point(handle, ctx)?;
                        return Ok(module);
                    }
                }
            }
        }

        Err(Error {
            kind: ErrorKind::SemanticError("Missing entry point".into()),
            meta: Span::default(),
        })
    }

    /// Returns additional information about the parsed shader which might not
    /// be stored in the [`Module`], see the documentation for
    /// [`ShaderMetadata`] for more information about the returned data.
    ///
    /// # Notes
    ///
    /// Following an unsuccessful parsing the state of the returned information
    /// is undefined, it might contain only partial information about the
    /// current shader, the previous shader or both.
    pub const fn metadata(&self) -> &ShaderMetadata {
        &self.meta
    }
}

/// Reorder functions in the module's arena so that callees appear before callers
/// (i.e., a function's handle index is always less than that of any function that
/// calls it). This is required by naga's IR validator.
///
/// Also updates the `Handle<Function>` references in `lookup_function` so that
/// entry point resolution finds the correct handles after reordering.
fn sort_functions_topologically(
    module: &mut Module,
    lookup_function: &mut FastHashMap<String, FunctionDeclaration>,
) {
    let count = module.functions.len();
    if count <= 1 {
        return;
    }

    // Build call graph: adj[caller_index] = list of callee indices
    let mut adj: Vec<Vec<usize>> = Vec::with_capacity(count);
    for (_, function) in module.functions.iter() {
        let mut callees = Vec::new();
        collect_call_handles(&function.body, &mut callees);
        adj.push(callees);
    }

    // Topological sort via DFS post-order.
    // Post-order on the call graph naturally puts callees before callers.
    let mut visited = vec![0u8; count]; // 0=unvisited, 1=in-progress, 2=done
    let mut sorted_order = Vec::with_capacity(count);
    let mut has_cycle = false;

    for i in 0..count {
        if visited[i] == 0 {
            topological_dfs(i, &adj, &mut visited, &mut sorted_order, &mut has_cycle);
            if has_cycle {
                // Cycle detected (mutual recursion) — leave functions in original order.
                // The validator will report the cycle as an error.
                return;
            }
        }
    }

    // Check if already in correct order (common case — no forward references)
    let already_sorted = sorted_order.iter().enumerate().all(|(i, &v)| i == v);
    if already_sorted {
        return;
    }

    // Build the old→new handle index remap
    let mut remap = vec![0usize; count];
    for (new_idx, &old_idx) in sorted_order.iter().enumerate() {
        remap[old_idx] = new_idx;
    }

    // Collect spans first (immutable borrow), then extract functions (mutable borrow)
    let spans: Vec<Span> = (0..count)
        .map(|i| module.functions.get_span(Handle::from_usize(i)))
        .collect();
    let mut functions_with_spans: Vec<(crate::Function, Span)> = Vec::with_capacity(count);
    for i in 0..count {
        let handle = Handle::from_usize(i);
        functions_with_spans.push((core::mem::take(&mut module.functions[handle]), spans[i]));
    }

    module.functions.clear();
    for &old_idx in &sorted_order {
        let (function, span) = core::mem::take(&mut functions_with_spans[old_idx]);
        module.functions.append(function, span);
    }

    // Remap Handle<Function> references in all function bodies and expressions
    for (_, function) in module.functions.iter_mut() {
        remap_call_handles(&mut function.body, &remap);
        remap_call_result_handles(&mut function.expressions, &remap);
    }

    // Remap handles in lookup_function so entry point resolution works
    for (_, decl) in lookup_function.iter_mut() {
        for overload in &mut decl.overloads {
            if let ast::FunctionKind::Call(ref mut handle) = overload.kind {
                let old_idx = handle.index();
                let new_handle = Handle::from_usize(remap[old_idx]);
                *handle = new_handle;
            }
        }
    }
}

fn topological_dfs(
    node: usize,
    adj: &[Vec<usize>],
    visited: &mut [u8],
    order: &mut Vec<usize>,
    has_cycle: &mut bool,
) {
    visited[node] = 1; // in-progress
    for &next in &adj[node] {
        if visited[next] == 1 {
            *has_cycle = true;
            return;
        }
        if visited[next] == 0 {
            topological_dfs(next, adj, visited, order, has_cycle);
            if *has_cycle {
                return;
            }
        }
    }
    visited[node] = 2; // done
    order.push(node);
}

/// Collect all callee function indices from Statement::Call in a block (recursively).
fn collect_call_handles(block: &crate::Block, callees: &mut Vec<usize>) {
    for statement in block.iter() {
        match *statement {
            crate::Statement::Call { function, .. } => {
                callees.push(function.index());
            }
            crate::Statement::Block(ref inner) => collect_call_handles(inner, callees),
            crate::Statement::If {
                ref accept,
                ref reject,
                ..
            } => {
                collect_call_handles(accept, callees);
                collect_call_handles(reject, callees);
            }
            crate::Statement::Switch { ref cases, .. } => {
                for case in cases {
                    collect_call_handles(&case.body, callees);
                }
            }
            crate::Statement::Loop {
                ref body,
                ref continuing,
                ..
            } => {
                collect_call_handles(body, callees);
                collect_call_handles(continuing, callees);
            }
            _ => {}
        }
    }
}

/// Remap Handle<Function> in all Statement::Call in a block (recursively).
fn remap_call_handles(block: &mut crate::Block, remap: &[usize]) {
    for statement in block.iter_mut() {
        match *statement {
            crate::Statement::Call {
                ref mut function, ..
            } => {
                *function = Handle::from_usize(remap[function.index()]);
            }
            crate::Statement::Block(ref mut inner) => remap_call_handles(inner, remap),
            crate::Statement::If {
                ref mut accept,
                ref mut reject,
                ..
            } => {
                remap_call_handles(accept, remap);
                remap_call_handles(reject, remap);
            }
            crate::Statement::Switch { ref mut cases, .. } => {
                for case in cases {
                    remap_call_handles(&mut case.body, remap);
                }
            }
            crate::Statement::Loop {
                ref mut body,
                ref mut continuing,
                ..
            } => {
                remap_call_handles(body, remap);
                remap_call_handles(continuing, remap);
            }
            _ => {}
        }
    }
}

/// Remap Handle<Function> in all Expression::CallResult in an expression arena.
fn remap_call_result_handles(
    expressions: &mut crate::Arena<crate::Expression>,
    remap: &[usize],
) {
    for (_, expr) in expressions.iter_mut() {
        if let crate::Expression::CallResult(ref mut function) = *expr {
            *function = Handle::from_usize(remap[function.index()]);
        }
    }
}
