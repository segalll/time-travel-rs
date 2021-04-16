use winit::event::{ElementState, VirtualKeyCode};

#[derive(Debug, Default)]
pub struct Input {
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        let pressed = state == ElementState::Pressed;
        match key {
            VirtualKeyCode::W => {
                self.up_pressed = pressed;
                true
            }
            VirtualKeyCode::S => {
                self.down_pressed = pressed;
                true
            }
            VirtualKeyCode::A => {
                self.left_pressed = pressed;
                true
            }
            VirtualKeyCode::D => {
                self.right_pressed = pressed;
                true
            }
            _ => false,
        }
    }
}