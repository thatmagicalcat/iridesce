use glam::Vec2;

pub struct Ray {
    pub origin: Vec2,
    pub direction: Vec2,
    pub wavelength: f32,
    pub intensity: f32,
}
