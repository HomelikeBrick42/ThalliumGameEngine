#version 330 core

in vec4 a_Position;
in vec3 a_Normal;
in vec2 a_TexCoord;

out vec3 v_Normal;
out vec2 v_TexCoord;

uniform mat4 u_ProjectionMatrix = mat4(1.0);
uniform mat4 u_ViewMatrix = mat4(1.0);
uniform mat4 u_ModelMatrix = mat4(1.0);

void main() {
  v_Normal = (u_ModelMatrix * vec4(a_Normal, 0.0)).xyz;
  v_TexCoord = a_TexCoord;
  gl_Position =
      u_ProjectionMatrix * inverse(u_ViewMatrix) * u_ModelMatrix * a_Position;
}
