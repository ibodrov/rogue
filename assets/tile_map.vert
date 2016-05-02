#version 150

in uvec2 position;
in uvec2 screen_position;
in vec2 tex_offset;
in vec4 fg_color;
in vec3 bg_color;

uniform mat4 matrix;
uniform uvec2 tile_size;
uniform vec2 tex_ratio;

out vec2 v_TexCoords;
out vec4 v_FgColor;
out vec3 v_BgColor;

void main() {
  gl_Position = matrix * vec4(position * tile_size + screen_position, 0.0, 1.0);

  float u = position.x * tex_ratio.x + tex_offset.x;
  float v = 1.0 - (position.y * tex_ratio.y + tex_offset.y);
  v_TexCoords = vec2(u, v);

  v_FgColor = fg_color;
  v_BgColor = bg_color;
}
