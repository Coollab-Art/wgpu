fn declared_later(x: f32) -> f32 {
    var x_1: f32;

    x_1 = x;
    let _e2 = x_1;
    return (_e2 + 1f);
}

fn calls_declared(x_2: f32) -> f32 {
    var x_3: f32;

    x_3 = x_2;
    let _e2 = x_3;
    let _e5 = declared_later((_e2 * 2f));
    return _e5;
}

fn defined_later(x_4: f32) -> f32 {
    var x_5: f32;

    x_5 = x_4;
    let _e2 = x_5;
    return (_e2 - 1f);
}

fn calls_later_defined(x_6: f32) -> f32 {
    var x_7: f32;

    x_7 = x_6;
    let _e2 = x_7;
    let _e5 = defined_later((_e2 * 3f));
    return _e5;
}

fn overloaded(x_8: f32) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e2 = x_9;
    return _e2;
}

fn overloaded_1(x_10: f32, y: f32) -> f32 {
    var x_11: f32;
    var y_1: f32;

    x_11 = x_10;
    y_1 = y;
    let _e4 = x_11;
    let _e5 = y_1;
    return (_e4 + _e5);
}

fn calls_overloaded() -> f32 {
    let _e1 = overloaded(1f);
    let _e4 = overloaded_1(2f, 3f);
    return (_e1 + _e4);
}

fn main_1() {
    var a: f32;
    var b: f32;
    var c: f32;

    let _e1 = calls_declared(1f);
    a = _e1;
    let _e4 = calls_later_defined(2f);
    b = _e4;
    let _e6 = calls_overloaded();
    c = _e6;
    return;
}

@fragment 
fn main() {
    main_1();
    return;
}
