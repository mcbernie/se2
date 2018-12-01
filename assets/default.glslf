#version 120
varying vec2 _uv;
uniform sampler2D from;
uniform sampler2D to;
uniform float progress;
uniform float ratio;
uniform float _fromR;
uniform float _toR;

vec4 getFromColor(vec2 uv) {
  return texture2D(from, uv);
}

vec4 getToColor(vec2 uv) {
  return texture2D(to, uv);
}

// REPLACE

void main() {
  gl_FragColor = transition(_uv);
}