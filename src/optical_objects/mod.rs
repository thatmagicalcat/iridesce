use enum_dispatch::enum_dispatch;
use glam::Vec2;

use crate::{drawable::Drawable, geometry::Geometry, intersection::Intersection, ray::Ray, transform::Transform};

mod plane_mirror;

pub use plane_mirror::PlaneMirror;

#[enum_dispatch]
pub trait OpticalObject {
    fn handle_intersection(&self, ray: &Ray, intersection: &Intersection) -> Vec<Ray>;
    fn check_and_clear_dirty(&mut self) -> bool;
    fn draw_ui(&mut self, ui: &mut egui_macroquad::egui::Ui);
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
