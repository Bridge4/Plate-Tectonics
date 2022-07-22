#version 140

    in vec2 position;
    uniform mat4 matrix;
    uniform float t;

    void main() {
        vec2 pos = position;
        pos.x += t;
        gl_Position = matrix * vec4(pos, 0.0, 1.0);
    }