#version 450

float constIn(const in float x) { return x; }
float constAlone(const float x) { return x; }

void main() {
    float a = constIn(1.0);
    a = constAlone(a);
}
