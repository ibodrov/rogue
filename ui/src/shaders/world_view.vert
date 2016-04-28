#version 150

in ivec2 position;
in ivec2 screen_position;
in vec3 color;

uniform mat4 matrix;
uniform ivec2 tile_size;
uniform vec2 atlas_ratio;

out vec3 v_Color;
out vec2 v_TexCoords;

void main() {
    gl_Position = matrix * vec4(position * tile_size + screen_position, 0.0, 1.0);

    float tile_offset_x = atlas_ratio.x * 2;
    float tile_offset_y = 0.0;
    float u = position.x * atlas_ratio.x + tile_offset_x;
    float v = 1.0 - (position.y * atlas_ratio.y + tile_offset_y);
    v_TexCoords = vec2(u, v);

    v_Color = color;
}