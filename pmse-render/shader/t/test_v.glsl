#version 460
// 顶点着色器 (vertex shader)

layout(location = 0) in vec3 p;
layout(location = 1) in vec3 color;

layout(location = 0) out vec3 out_color;

void main() {
  gl_Position = vec4(p, 1.0);
  out_color = color;
}
