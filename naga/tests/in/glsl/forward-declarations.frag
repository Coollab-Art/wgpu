#version 450

// Test 1: Forward declaration (prototype before definition)
float declared_later(float x);

float calls_declared(float x) {
    return declared_later(x * 2.0);
}

float declared_later(float x) {
    return x + 1.0;
}

// Test 2: Call to function defined later (no explicit prototype)
float calls_later_defined(float x) {
    return defined_later(x * 3.0);
}

float defined_later(float x) {
    return x - 1.0;
}

// Test 3: Overloaded functions with forward declarations
float overloaded(float x);
float overloaded(float x, float y);

float calls_overloaded() {
    return overloaded(1.0) + overloaded(2.0, 3.0);
}

float overloaded(float x) {
    return x;
}

float overloaded(float x, float y) {
    return x + y;
}

void main() {
    float a = calls_declared(1.0);
    float b = calls_later_defined(2.0);
    float c = calls_overloaded();
}
