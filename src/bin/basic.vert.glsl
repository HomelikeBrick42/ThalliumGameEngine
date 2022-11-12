#version 440 core

layout(location = 0) in vec4 a_Position;
layout(location = 1) in vec3 a_Normal;

layout(location = 0) out vec3 v_Normal;

layout(location = 0) uniform mat4 u_ProjectionMatrix = mat4(1.0);
layout(location = 1) uniform mat4 u_ViewMatrix = mat4(1.0);
layout(location = 2) uniform mat4 u_ModelMatrix = mat4(1.0);

void main() {
  v_Normal = (u_ModelMatrix * vec4(a_Normal, 0.0)).xyz;
  gl_Position =
      u_ProjectionMatrix * inverse(u_ViewMatrix) * u_ModelMatrix * a_Position;
}
