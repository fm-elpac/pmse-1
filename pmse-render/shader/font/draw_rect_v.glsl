#version 460
// 绘制矩形 (vertex shader)

layout(location = 0) in vec3 p;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec3 color;

layout(location = 0) out vec2 out_uv;
layout(location = 1) out vec3 out_color;

void main() {
  gl_Position = vec4(p, 1.0);
  out_uv = uv;
  out_color = color;
}
