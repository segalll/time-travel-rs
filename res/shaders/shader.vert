#version 460

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec2 a_tex_coords;

layout(location = 0) out vec2 v_tex_coords;
layout(location = 1) flat out uint v_tex_id;

layout(set = 0, binding = 0)
uniform Uniforms {
    mat4 u_proj;
};

struct Data {
    mat4 model;
    uint tex_id;
};

layout(std430, set = 1, binding = 0)
readonly buffer Storage {
    Data data[];
};

void main() {
    v_tex_coords = a_tex_coords;
    v_tex_id = data[gl_BaseInstance].tex_id;
    gl_Position = u_proj * data[0].model * vec4(a_position, 1.0);
}