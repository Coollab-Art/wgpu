struct FragmentOutput {
    @location(0) fragColor: vec4<f32>,
}

@group(0) @binding(0) 
var tex2D: texture_2d<f32>;
@group(0) @binding(1) 
var samp: sampler;
@group(0) @binding(2) 
var texDepth: texture_depth_2d;
@group(0) @binding(3) 
var sampShadow: sampler_comparison;
var<private> fragColor: vec4<f32>;

fn sampleBasic(tex: texture_2d<f32>, tex_sampler: sampler, uv: vec2<f32>) -> vec4<f32> {
    var uv_1: vec2<f32>;

    uv_1 = uv;
    let _e4 = uv_1;
    let _e5 = textureSample(tex, tex_sampler, _e4);
    return _e5;
}

fn sampleShadow(tex_1: texture_depth_2d, tex_sampler_1: sampler_comparison, coord: vec3<f32>) -> f32 {
    var coord_1: vec3<f32>;

    coord_1 = coord;
    let _e4 = coord_1;
    let _e7 = textureSampleCompare(tex_1, tex_sampler_1, _e4.xy, _e4.z);
    return _e7;
}

fn sampleIndirect(tex_2: texture_2d<f32>, tex_sampler_2: sampler, uv_2: vec2<f32>) -> vec4<f32> {
    var uv_3: vec2<f32>;

    uv_3 = uv_2;
    let _e4 = uv_3;
    let _e5 = sampleBasic(tex_2, tex_sampler_2, _e4);
    return _e5;
}

fn main_1() {
    var a: vec4<f32>;
    var b: f32;
    var c: vec4<f32>;

    let _e7 = sampleBasic(tex2D, samp, vec2(0.5f));
    a = _e7;
    let _e11 = sampleShadow(texDepth, sampShadow, vec3(0.5f));
    b = _e11;
    let _e15 = sampleIndirect(tex2D, samp, vec2(0.5f));
    c = _e15;
    let _e17 = a;
    let _e18 = b;
    let _e21 = c;
    fragColor = ((_e17 + vec4(_e18)) + _e21);
    return;
}

@fragment 
fn main() -> FragmentOutput {
    main_1();
    let _e1 = fragColor;
    return FragmentOutput(_e1);
}
