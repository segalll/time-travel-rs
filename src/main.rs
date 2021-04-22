mod render;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    use futures::executor::block_on;

    let mut render_state: render::Render = block_on(render::Render::new(&window));

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
