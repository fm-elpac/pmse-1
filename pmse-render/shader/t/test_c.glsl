#version 460
// 计算着色器 (compute shader)

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
  uint data[];
} b;

void main() {
  uint idx = gl_GlobalInvocationID.x;
  b.data[idx] += 6;
}
