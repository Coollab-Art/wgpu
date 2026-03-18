#version 450

layout(location = 0) out vec4 out_color;

// Non-void function with missing return on some paths
float missing_return_float(float x) {
    if (x < 1.0) return 1.0;
    if (x < 2.0) return 2.0;
    // No return here — undefined behavior per GLSL spec 6.1, but not an error
}

// Non-void function returning vec3
vec3 missing_return_vec3(int x) {
    if (x == 0) {
        return vec3(1.0, 0.0, 0.0);
    } else if (x == 1) {
        return vec3(0.0, 1.0, 0.0);
    }
    // Missing else — no return on this path
}

// Non-void function returning int
int missing_return_int(bool b) {
    if (b) return 42;
}

// Non-void function returning mat4
mat4 missing_return_mat4(float x) {
    if (x > 0.0) return mat4(1.0);
}

// Void function — should be unaffected
void void_function() {
    if (true) return;
}

// Function where all paths already return — should be unaffected
float complete_return(float x) {
    if (x > 0.0) {
        return 1.0;
    } else {
        return -1.0;
    }
}

void main() {
    float a = missing_return_float(0.5);
    vec3 b = missing_return_vec3(2);
    int c = missing_return_int(false);
    mat4 d = missing_return_mat4(-1.0);
    void_function();
    float e = complete_return(1.0);
    out_color = vec4(a, b.x, float(c), e);
}
