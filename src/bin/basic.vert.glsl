#version 440 core

layout(location = 0) in vec4 a_Position;

layout(location = 0) uniform mat4 u_ProjectionMatrix = mat4(1.0);
layout(location = 1) uniform mat4 u_ViewMatrix = mat4(1.0);
layout(location = 2) uniform mat4 u_ModelMatrix = mat4(1.0);

void main() {
  gl_Position =
      u_ProjectionMatrix * inverse(u_ViewMatrix) * u_ModelMatrix * a_Position;
}
