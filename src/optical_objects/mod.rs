use enum_dispatch::enum_dispatch;
use glam::Vec2;
use raylib::drawing::RaylibDrawHandle;

use crate::{drawable::Drawable, geometry::Geometry, intersection::Intersection, ray::Ray};

mod plane_mirror;

pub use plane_mirror::PlaneMirror;

#[enum_dispatch]
pub trait OpticalObject {
    fn handle_intersection(&self, ray: &Ray, intersection: &Intersection) -> Vec<Ray>;
}

#[enum_dispatch(OpticalObject, Drawable, Geometry)]
pub enum OpticalObjectEnum {
    PlaneMirror,
}

#[derive(Clone, Copy)]
pub struct Material {
    pub reflectivity: f32,
    pub refractive_index: f32,
}
