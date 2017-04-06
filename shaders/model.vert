#version 140

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 view_perspective;
uniform mat4 model;

void main() {
    v_normal = transpose(inverse(mat3(model))) * normal;
    gl_Position = view_perspective * model * vec4(position, 1.0);
}

