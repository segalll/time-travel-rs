mod render;
mod systems;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use legion::*;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    use futures::executor::block_on;

    let mut render_state: render::Render = block_on(render::Render::new(&window));

    let mut world = World::default();

    world.push((systems::Drawable::new(0f32, 0f32, 5), false));
    world.push((systems::Drawable::new(0f32, 0f32, 5), false));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                   ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            WindowEvent::Resized(physical_size) => {
                render_state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                render_state.resize(**new_inner_size);
            }
            _ => {}
       }
       Event::RedrawRequested(_) => {
            let mut query = <&systems::Drawable>::query();

            for item in query.iter(&world) {
                render_state.add_sprite(item.model_matrix(), item.texture_id);
            }
            render_state.update_storage();

            match render_state.render() {
                Ok(_) => {}
                Err(wgpu::SwapChainError::Lost) => render_state.resize(render_state.size),
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
