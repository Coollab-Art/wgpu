#version 450

layout(set = 0, binding = 0) uniform Uniforms {
    float time;
};

// Non-const global initializer referencing a uniform
float speed = time * 2.0;

// Const global initializer (should still work as before)
const float PI = 3.14159;

// Non-const global referencing another non-const global
float double_speed = speed + speed;

layout(location = 0) out vec4 o_color;

void main() {
    o_color = vec4(speed, double_speed, PI, 1.0);
}
