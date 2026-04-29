#version 100
precision highp float;

attribute vec2 position;
attribute vec2 texcoord;
uniform mat4 Model;
uniform mat4 Projection;
varying vec2 v_uv;

void main() {
    v_uv = texcoord;
    gl_Position = Projection * Model * vec4(position, 0.0, 1.0);
}
