#version 450

layout(location = 0) out vec4 out_color;

float helper() {
    return 1.0;
}

// Dead code after return: simple case with function call
float dead_code_with_call() {
    return 1.0;
    float x = helper();
    return x;
}

// Dead code after return: simple assignment
float dead_code_with_assignment() {
    float y = 2.0;
    return y;
    y = 3.0;
}

// Dead code after discard
void dead_code_after_discard() {
    discard;
    float z = helper();
}

// Return in nested scope should NOT affect outer scope
float nested_scope_return() {
    if (true) {
        return 1.0;
    }
    // This is NOT dead code — the return above is inside a nested scope
    return 0.0;
}

void main() {
    float a = dead_code_with_call();
    float b = dead_code_with_assignment();
    float c = nested_scope_return();
    out_color = vec4(a, b, c, 1.0);
}
