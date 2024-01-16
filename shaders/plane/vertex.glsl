#version 450

#include "../structs.glsl"

layout(location = 0) uniform FrameData frame_data;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 color;

out vec3 vNormal;
out vec3 vColor;

void main() {
  gl_Position = frame_data.view_proj * vec4(pos, 1.);
  vNormal = normal;
  vColor = color;
}
