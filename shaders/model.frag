#version 140

in vec3 v_normal;

out vec4 color;

uniform vec3 light;

void main() {
    float brightness = dot(normalize(v_normal), normalize(light));
    vec3 dark_color = vec3(0.6, 0.2, 0.1);
    vec3 regular_color = vec3(0.8, 0.3, 0.2);
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}

