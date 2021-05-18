#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Drawable {
    pub position: cgmath::Vector2::<f32>,
    pub rotation: cgmath::Vector2::<cgmath::Rad<f32>>,
    pub scale: cgmath::Vector2::<f32>,
    pub texture_id: u32,
}

impl Drawable {
    pub fn new(x: f32, y: f32, texture_id: u32) -> Self {
        Self {
            position: cgmath::vec2(x, y),
            rotation: cgmath::vec2(cgmath::Rad(0f32), cgmath::Rad(0f32)),
            scale: cgmath::vec2(1f32, 1f32),
            texture_id
        }
    }

    pub fn model_matrix(&self) -> cgmath::Matrix4::<f32> {
        let translation = cgmath::Matrix4::from_translation(self.position.extend(0f32));
        let rotation = cgmath::Matrix4::from_angle_x(self.rotation.x)
            * cgmath::Matrix4::from_angle_y(self.rotation.y);
        let scale = cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, 1f32);
        translation * rotation * scale
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Inputtable {
    pub speed: f32,
}

impl Inputtable {
    pub fn new(speed: f32) -> Self {
        Self {
            speed
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Animatable {
    pub frame_id: u32,
    pub frames_per_anim: u32,
    pub total_frames: u32,
    pub frames_since_change: u32,
    pub texture_offset: u32,
}

impl Animatable {
    pub fn new(frames_per_anim: u32, total_frames: u32) -> Self {
        Self {
            frame_id: 0,
            frames_per_anim,
            total_frames,
            frames_since_change: 0,
            texture_offset: 1,
        }
    }
}