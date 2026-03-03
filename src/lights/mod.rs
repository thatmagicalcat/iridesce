use enum_dispatch::enum_dispatch;
use glam::Vec2;

use crate::{drawable::Drawable, geometry::Geometry, intersection::Intersection, ray::Ray};

mod laser_light;
mod point_light;

pub use laser_light::LaserLight;
pub use point_light::PointLight;

#[enum_dispatch]
pub trait LightSource {
    fn get_rays(&self) -> Vec<Ray>;

    /// Returns true if the light source has been modified since the last time it was drawn, which
    /// means the ray tracing needs to be re-run.
    ///
    /// This function will mark the light source as clean after being called.
    fn check_and_clear_dirty(&mut self) -> bool;

    fn draw_ui(&mut self, ui: &mut egui_macroquad::egui::Ui);
}

#[enum_dispatch(LightSource, Geometry, Drawable)]
pub enum LightSourceEnum {
    LaserLight,
    PointLight,
}
