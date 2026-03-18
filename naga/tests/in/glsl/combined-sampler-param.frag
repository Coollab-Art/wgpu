#version 440
precision mediump float;

layout(set = 0, binding = 0) uniform texture2D tex2D;
layout(set = 0, binding = 1) uniform sampler samp;
layout(set = 0, binding = 2) uniform texture2D texDepth;
layout(set = 0, binding = 3) uniform samplerShadow sampShadow;

layout(location = 0) out vec4 fragColor;

// Basic: combined sampler as function parameter
vec4 sampleBasic(sampler2D tex, vec2 uv) {
    return texture(tex, uv);
}

// Shadow variant
float sampleShadow(sampler2DShadow tex, vec3 coord) {
    return texture(tex, coord);
}

// Pass-through: combined param forwarded to another combined-param function
vec4 sampleIndirect(sampler2D tex, vec2 uv) {
    return sampleBasic(tex, uv);
}

void main() {
    vec4 a = sampleBasic(sampler2D(tex2D, samp), vec2(0.5));
    float b = sampleShadow(sampler2DShadow(texDepth, sampShadow), vec3(0.5));
    vec4 c = sampleIndirect(sampler2D(tex2D, samp), vec2(0.5));
    fragColor = a + vec4(b) + c;
}
