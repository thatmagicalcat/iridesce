use enum_dispatch::enum_dispatch;
use glam::Vec2;
use raylib::drawing::RaylibDrawHandle;

use crate::{drawable::Drawable, geometry::Geometry, ray::Ray, intersection::Intersection,};

mod laser_light;
mod point_light;

pub use laser_light::LaserLight;
pub use point_light::PointLight;

#[enum_dispatch]
pub trait LightSource {
    fn get_rays(&self) -> Vec<Ray>;
}

#[enum_dispatch(LightSource, Geometry, Drawable)]
pub enum LightSourceEnum {
    LaserLight,
    PointLight,
}
