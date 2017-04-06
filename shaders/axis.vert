#version 140

in vec3 position;

uniform mat4 view_perspective;

void main() {
    gl_Position = view_perspective * vec4(position, 1.0);
}
