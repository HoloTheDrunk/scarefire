#version 450

struct FrameData {
    mat4 view_proj;

    vec3 sun_dir;
    uint point_light_count;

    vec3 sun_color;
    float padding_1;
};

layout(location = 0) uniform FrameData frame_data;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 color;

out vec3 vNormal;
out vec3 vColor;

void main() {
  gl_Position = vec4(1.);// frame_data.view_proj * vec4(pos, 1.);
  vNormal = normal;
  vColor = color;
}
