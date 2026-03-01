use glam::Vec2;

pub struct Intersection {
    pub point: Vec2,
    pub normal: Vec2,
    pub sq_distance: f32,
    pub reflectivity: f32,
}
