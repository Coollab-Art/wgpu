struct FragmentOutput {
    @location(0) out_color: vec4<f32>,
}

var<private> out_color: vec4<f32>;

fn dead_code_with_call() -> f32 {
    return 1f;
}

fn dead_code_with_assignment() -> f32 {
    var y: f32 = 2f;

    let _e2 = y;
    return _e2;
}

fn nested_scope_return() -> f32 {
    if true {
        {
            return 1f;
        }
    }
    return 0f;
}

fn main_1() {
    var a: f32;
    var b: f32;
    var c: f32;

    let _e1 = dead_code_with_call();
    a = _e1;
    let _e3 = dead_code_with_assignment();
    b = _e3;
    let _e5 = nested_scope_return();
    c = _e5;
    let _e7 = a;
    let _e8 = b;
    let _e9 = c;
    out_color = vec4<f32>(_e7, _e8, _e9, 1f);
    return;
}

@fragment 
fn main() -> FragmentOutput {
    main_1();
    let _e1 = out_color;
    return FragmentOutput(_e1);
}
