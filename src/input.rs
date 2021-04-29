use winit::event::{ElementState, VirtualKeyCode};
use std::collections::HashSet;

pub struct Input {
    keys_down: HashSet<VirtualKeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => self.keys_down.insert(key),
            ElementState::Released => self.keys_down.remove(&key),
        };
    }

    pub fn key_down(&mut self, key: VirtualKeyCode) -> bool {
        self.keys_down.contains(&key)
    }
}