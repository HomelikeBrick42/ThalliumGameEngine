#version 440 core

layout(location = 0) in vec4 a_Position;
layout(location = 1) in vec3 a_Color;

layout(location = 0) out vec3 v_Color;

void main() {
    v_Color = a_Color;
    gl_Position = a_Position;
}
