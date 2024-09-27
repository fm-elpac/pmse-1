#version 460
// 绘制矩形 (fragment shader)

layout(location = 0) in vec2 uv;
layout(location = 1) in vec3 color;

layout(location = 0) out vec4 f_color;

void main() {
  // TODO
  f_color = vec4(color, 1.0);
}
