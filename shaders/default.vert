#version 450

layout(push_constant) uniform PushConstants {
    mat4 view_projection;
    mat4 model;
} constants;

layout(location = 0) in vec3 position;

void main() {
    gl_Position = constants.view_projection * (constants.model * vec4(position, 1.0));
}