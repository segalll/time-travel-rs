#version 450

layout(location = 0) in vec2 v_tex_coords;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 1) uniform texture2D u_textures[32];
layout(set = 0, binding = 2) uniform sampler u_sampler;
layout(push_constant) uniform Uniforms {
    int u_index;
};

void main() {
    f_color = texture(sampler2D(u_textures[u_index], u_sampler), v_tex_coords);
}