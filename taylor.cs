#version 450

#define BRANCH

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(std430, set = 0, binding = 0) buffer ToSin { float tosin[]; };

float mpow(float a, int b) {
  float acc = 1.0;
  for (int i = 0; i < b; ++i) {
    acc *= a;
  }
  return acc;
}

float fact(float x) {
  float acc = 1.0;
  for (float i = 1.0; i <= x; i += 1.0) {
    acc = acc * i;
  }
  return acc;
}

float taylor_sin(float x, int iter) {
  float acc = 0.0;
  for (int i = 0; i < iter; ++i) {
    float n = float(i);
    acc += (mpow(-1.0, i) * pow(x, 2.0 * n + 1.0)) / fact(2.0 * n + 1.0);
  }
  return acc;
}

void main() {
  uint idx = gl_GlobalInvocationID.x;
  float data = tosin[idx];
#ifndef BRANCH
  tosin[idx] = taylor_sin(data, 32);
#else
  if(data < 0.2) {
    tosin[idx] = taylor_sin(data, 8);
  } else if (data < 0.5) {
    tosin[idx] = taylor_sin(data, 16);
  } else {
    tosin[idx] = taylor_sin(data, 32);
  }
#endif
}