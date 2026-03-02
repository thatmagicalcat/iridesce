use glam::{Affine2, Mat2, Vec2};

pub struct Transform {
    pub position: Vec2,
    pub rotation: f32, // in radians
    pub scale: Vec2,
}

impl Transform {
    pub fn local_to_world(&self) -> Affine2 {
        Affine2::from_scale_angle_translation(self.scale, self.rotation, self.position)
    }

    pub fn world_to_local(&self) -> Affine2 {
        self.local_to_world().inverse()
    }

    pub fn identity() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}
