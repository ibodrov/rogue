#version 150

in vec3 v_Color;
in vec2 v_TexCoords;

uniform sampler2D tex;

out vec4 color;

void main() {
    color = texture(tex, v_TexCoords) * vec4(v_Color, 1.0);
}