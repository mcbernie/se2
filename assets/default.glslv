#version 120
attribute vec3 position;
attribute vec2 a_uv;
varying vec2 _uv;
uniform mat4 u_model_view_proj;

//gl_Position = project * view * world * vec4(position, 1.0);

void main() {
  gl_Position = vec4(position,1.0);
  //vec2 uv = position * 0.5 + 0.5;
  _uv = a_uv;
}