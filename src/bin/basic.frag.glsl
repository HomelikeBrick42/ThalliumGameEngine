#version 440 core

layout(location = 0) in vec3 v_Normal;

layout(location = 0) out vec4 o_Color;

layout(location = 3) uniform vec3 u_Color = vec3(1.0);

void main() {
  vec3 lightDir = normalize(vec3(0.2, -1.0, 0.4));
  float lightIntensity = dot(normalize(v_Normal), -lightDir) * 0.5 + 0.5;
  o_Color = vec4(u_Color * lightIntensity, 1.0);
}
