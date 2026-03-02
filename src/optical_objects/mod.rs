use enum_dispatch::enum_dispatch;
use raylib::drawing::RaylibDrawHandle;

use crate::{intersection::Intersection, ray::Ray, drawable::Drawable};

mod plane_mirror;

pub use plane_mirror::PlaneMirror;

#[enum_dispatch]
pub trait OpticalObject {
    fn intersect(&self, world_ray: &Ray) -> Option<Intersection>;
    fn handle_intersection(&self, ray: &Ray, intersection: &Intersection) -> Vec<Ray>;
}

#[enum_dispatch(OpticalObject, Drawable)]
pub enum OpticalObjectEnum {
    PlaneMirror,
}

#[derive(Clone, Copy)]
pub struct Material {
    pub reflectivity: f32,
    pub refractive_index: f32,
}
