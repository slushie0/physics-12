#version 100
#extension GL_OES_standard_derivatives : enable
precision highp float;

uniform vec2  u_resolution;
uniform vec2  u_center;
uniform float u_radius;
uniform vec4  u_color;

void main() {
    // Convert fragment coordinates to normalized device space relative to screen center
    // This reduces coordinate magnitudes and improves precision
    vec2 screen_center = u_resolution * 0.5;
    vec2 pos = vec2(gl_FragCoord.x, u_resolution.y - gl_FragCoord.y);
    vec2 rel = pos - screen_center - u_center;
    
    float dist = length(rel) - u_radius;
    
    // Expanded discard threshold for better edge preservation
    if (dist > 2.0) {
        discard;
    }
    
    // Use fwidth for adaptive anti-aliasing based on derivatives
    float edge_width = fwidth(dist);
    float edge = 1.0 - smoothstep(-edge_width, edge_width, dist);
    gl_FragColor = mix(vec4(0.0, 0.0, 0.0, 1.0), u_color, edge);
}