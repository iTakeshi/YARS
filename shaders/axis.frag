#version 140
uniform vec3 rgb;
out vec4 color;
void main() {
    color = vec4(rgb, 1.0);
}
