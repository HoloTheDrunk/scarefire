#version 450

#include "../structs.glsl"

layout(location = 0) uniform FrameData frame_data;

in vec3 vNormal;
in vec3 vColor;

void main() {
  gl_FragColor = gl_FragCoord.xyz;
}
