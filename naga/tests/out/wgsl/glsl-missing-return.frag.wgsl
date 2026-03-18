struct FragmentOutput {
    @location(0) out_color: vec4<f32>,
}

var<private> out_color: vec4<f32>;

fn missing_return_float(x: f32) -> f32 {
    var x_1: f32;

    x_1 = x;
    let _e2 = x_1;
    if (_e2 < 1f) {
        return 1f;
    }
    let _e6 = x_1;
    if (_e6 < 2f) {
        return 2f;
    } else {
        return f32();
    }
}

fn missing_return_vec3_(x_2: i32) -> vec3<f32> {
    var x_3: i32;

    x_3 = x_2;
    let _e2 = x_3;
    if (_e2 == 0i) {
        {
            return vec3<f32>(1f, 0f, 0f);
        }
    } else {
        let _e9 = x_3;
        if (_e9 == 1i) {
            {
                return vec3<f32>(0f, 1f, 0f);
            }
        } else {
            return vec3<f32>();
        }
    }
}

fn missing_return_int(b: bool) -> i32 {
    var b_1: bool;

    b_1 = b;
    let _e2 = b_1;
    if _e2 {
        return 42i;
    } else {
        return i32();
    }
}

fn missing_return_mat4_(x_4: f32) -> mat4x4<f32> {
    var x_5: f32;

    x_5 = x_4;
    let _e2 = x_5;
    if (_e2 > 0f) {
        return mat4x4<f32>(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
    } else {
        return mat4x4<f32>();
    }
}

fn void_function() {
    if true {
        return;
    } else {
        return;
    }
}

fn complete_return(x_6: f32) -> f32 {
    var x_7: f32;

    x_7 = x_6;
    let _e2 = x_7;
    if (_e2 > 0f) {
        {
            return 1f;
        }
    } else {
        {
            return -1f;
        }
    }
}

fn main_1() {
    var a: f32;
    var b_2: vec3<f32>;
    var c: i32;
    var d: mat4x4<f32>;
    var e: f32;

    let _e2 = missing_return_float(0.5f);
    a = _e2;
    let _e5 = missing_return_vec3_(2i);
    b_2 = _e5;
    let _e8 = missing_return_int(false);
    c = _e8;
    let _e11 = missing_return_mat4_(-1f);
    d = _e11;
    void_function();
    let _e14 = complete_return(1f);
    e = _e14;
    let _e16 = a;
    let _e17 = b_2;
    let _e19 = c;
    let _e21 = e;
    out_color = vec4<f32>(_e16, _e17.x, f32(_e19), _e21);
    return;
}

@fragment 
fn main() -> FragmentOutput {
    main_1();
    let _e1 = out_color;
    return FragmentOutput(_e1);
}
