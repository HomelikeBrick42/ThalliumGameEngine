#version 440 core

layout(location = 0) out vec4 o_Color;

layout(location = 3) uniform vec3 u_Color = vec3(1.0);

void main() { o_Color = vec4(u_Color, 1.0); }
