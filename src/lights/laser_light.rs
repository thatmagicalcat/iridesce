use glam::Vec2;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    aabb::AABB, drawable::Drawable, geometry::Geometry, intersection::Intersection, ray::Ray,
};
use crate::{geometry::IntoRaylibVector, lights::LightSource};

pub struct LaserLight {
    pub origin: Vec2,
    pub direction: Vec2,
    pub wavelength: f32,
    pub bounds: AABB,
}

impl LaserLight {
    pub fn new(origin: Vec2, direction: Vec2, wavelength: f32) -> Self {
        let bounds = AABB::new(origin - direction * 20.0, origin + direction * 20.0);
        LaserLight {
            origin,
            direction: direction.normalize(),
            wavelength,
            bounds,
        }
    }
}

impl LightSource for LaserLight {
    fn get_rays(&self) -> Vec<Ray> {
        vec![Ray {
            origin: self.origin,
            direction: self.direction,
            wavelength: self.wavelength,
            intensity: 1.0,
        }]
    }
}

impl Drawable for LaserLight {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        let end = self.origin + self.direction * 20.0;
        d.draw_line_ex(self.origin.into_rvec(), end.into_rvec(), 3.0, Color::YELLOW)
    }
}

impl Geometry for LaserLight {
    fn intersect(&self, _: &Ray) -> Option<Intersection> {
        panic!("LaserLight does not have a physical geometry to intersect with rays.")
    }

    fn contains_point(&self, point: Vec2) -> bool {
        self.bounds.contains(point)
    }

    fn set_position(&mut self, position: Vec2) {
        self.origin = position;
    }
}
