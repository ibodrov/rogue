#version 150

in highp vec2 v_TexCoords;
in lowp vec4 v_FgColor;
in lowp vec3 v_BgColor;

uniform lowp sampler2D tex;

out lowp vec4 color;

void main() {
  vec4 c1 = texture(tex, v_TexCoords) * v_FgColor;
  vec4 c2 = vec4(v_BgColor, 1.0) * (1.0 - c1.a);
  color = clamp(c1 + c2, 0.0, 1.0);
}
