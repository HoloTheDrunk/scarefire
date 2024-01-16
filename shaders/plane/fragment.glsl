#version 450

in vec3 vNormal;
in vec3 vColor;

out vec4 out_color;

void main() {
  out_color = gl_FragCoord;
}
