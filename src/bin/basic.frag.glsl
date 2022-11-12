#version 330 core

in vec3 v_Normal;
in vec2 v_TexCoord;

out vec4 o_Color;

uniform vec3 u_Color = vec3(1.0);
uniform sampler2D u_Texture;

void main() {
  vec3 lightDir = normalize(vec3(0.2, -1.0, 0.4));
  float lightIntensity = dot(normalize(v_Normal), -lightDir) * 0.5 + 0.5;
  o_Color =
      vec4(u_Color, 1.0) * texture(u_Texture, v_TexCoord) * lightIntensity;
}
