#version 450

layout(local_size_x = 32, local_size_y = 1, local_size_z = 1) in;

layout(std430, set = 0, binding = 0) buffer Data { float data[]; };

void main() {
  uint i = gl_GlobalInvocationID.x;
  data[i] *= 2.0;
}
