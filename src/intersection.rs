use glam::Vec2;

use crate::optical_objects::Material;

pub struct Intersection {
    pub point: Vec2,
    pub normal: Vec2,
    pub sq_distance: f32,
    pub material: Material,
}
