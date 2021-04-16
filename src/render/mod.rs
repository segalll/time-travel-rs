mod buffer;
mod sprite;
mod texture;

use std::iter;

use winit::{
    monitor::VideoMode,
    window::Window,
};

use buffer::*;

use crate::state;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    proj: [[f32; 4]; 4],
}

impl Uniforms {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_proj(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        let ratio: f32 = (size.width as f32) / (size.height as f32);
        self.proj = (OPENGL_TO_WGPU_MATRIX * cgmath::ortho(-ratio, ratio, -1.0, 1.0, -5.0, 5.0)).into();
    }
}

struct Render {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}