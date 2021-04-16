use crate::render::texture;

struct Sprite {
    position: (u32, u32),
    texture_bind_group: wgpu::BindGroup,
    texture: texture::Texture,
}