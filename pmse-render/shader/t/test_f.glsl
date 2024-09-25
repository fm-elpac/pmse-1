#version 460
// 片段着色器 (fragment shader)

layout(location = 0) in vec3 in_color;

layout(location = 0) out vec4 f_color;

void main() {
  f_color = vec4(in_color, 1.0);
}
