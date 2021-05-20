mod render;
mod systems;
mod input;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use legion::*;

use crate::systems::PrevMove;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    use futures::executor::block_on;

    let mut render_state: render::Render = block_on(render::Render::new(&window));

    let mut input_state: input::Input = input::Input::new();

    let mut world = World::default();

    world.push((
        systems::Drawable::new(0f32, 0f32, 0),
    ));
    world.push((
        systems::Drawable::new(0f32, 0f32, 0),
        systems::Inputtable::new(0.25),
        systems::Animatable::new(0.15f32, 6),
    ));

    let mut last_render_time = std::time::Instant::now();

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
                KeyboardInput {
                    state,
                    virtual_keycode: Some(key),
                    ..
                } => input_state.process_keyboard(*key, *state),
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
            let now = std::time::Instant::now();
            let dt = now - last_render_time;
            last_render_time = now;

            let mut input_query = <(&mut systems::Inputtable, &mut systems::Drawable)>::query();

            for (inputtable, drawable) in input_query.iter_mut(&mut world) {
                let delta = inputtable.speed * dt.as_secs_f32();
                let mut new_move = PrevMove::None;

                if input_state.key_down(VirtualKeyCode::W) {
                    drawable.position.y += delta;
                    new_move = PrevMove::Up;
                }
                if input_state.key_down(VirtualKeyCode::S) {
                    drawable.position.y -= delta;
                    new_move = PrevMove::Down;
                }
                if input_state.key_down(VirtualKeyCode::A) {
                    drawable.position.x -= delta;
                    new_move = PrevMove::Left;
                }
                if input_state.key_down(VirtualKeyCode::D) {
                    drawable.position.x += delta;
                    new_move = PrevMove::Right;
                }
                inputtable.should_change = inputtable.prev_move != new_move;
                inputtable.prev_move = new_move;
            }

            let mut input_anim_query = <(&systems::Inputtable, &mut systems::Animatable)>::query();

            for (inputtable, animatable) in input_anim_query.iter_mut(&mut world) {
                animatable.texture_offset = match inputtable.prev_move {
                    PrevMove::Left => 12,
                    PrevMove::Right => 18,
                    PrevMove::Up => 0,
                    PrevMove::Down => 6,
                    PrevMove::None => animatable.texture_offset
                };
                if inputtable.prev_move != PrevMove::None && inputtable.should_change {
                    animatable.frame_id = 0;
                    animatable.delta_since_change = 0f32;
                }
            }

            let mut anim_query = <(&mut systems::Animatable, &mut systems::Drawable)>::query();

            for (animatable, drawable) in anim_query.iter_mut(&mut world) {
                if animatable.delta_since_change >= animatable.frame_delta {
                    animatable.frame_id = (animatable.frame_id + 1) % animatable.total_frames;
                    animatable.delta_since_change = 0f32;
                } else {
                    animatable.delta_since_change += dt.as_secs_f32();
                }
                drawable.texture_id = animatable.frame_id + animatable.texture_offset;
            }

            let mut draw_query = <&systems::Drawable>::query();

            for drawable in draw_query.iter(&world) {
                render_state.add_sprite(drawable.model_matrix(), drawable.texture_id);
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
