fn constIn(x: f32) -> f32 {
    return x;
}

fn constAlone(x_1: f32) -> f32 {
    return x_1;
}

fn main_1() {
    var a: f32;

    let _e1 = constIn(1f);
    a = _e1;
    let _e3 = a;
    let _e4 = constAlone(_e3);
    a = _e4;
    return;
}

@fragment 
fn main() {
    main_1();
    return;
}
