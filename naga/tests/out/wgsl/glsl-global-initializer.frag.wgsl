struct Uniforms {
    time: f32,
}

struct FragmentOutput {
    @location(0) o_color: vec4<f32>,
}

const PI: f32 = 3.14159f;

@group(0) @binding(0) 
var<uniform> global: Uniforms;
var<private> speed: f32;
var<private> double_speed: f32;
var<private> o_color: vec4<f32>;

fn main_1() {
    let _e4 = speed;
    let _e5 = double_speed;
    o_color = vec4<f32>(_e4, _e5, PI, 1f);
    return;
}

@fragment 
fn main() -> FragmentOutput {
    let _e2 = global.time;
    speed = (_e2 * 2f);
    let _e7 = speed;
    let _e8 = speed;
    double_speed = (_e7 + _e8);
    main_1();
    let _e12 = o_color;
    return FragmentOutput(_e12);
}
