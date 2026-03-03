use glam::Vec2;
use raylib::{color::Color, drawing::RaylibDraw};

use super::LightSource;
use crate::{
    aabb::AABB, drawable::Drawable, geometry::Geometry, intersection::Intersection, ray::Ray,
};

/// Circle of arbitrary radius 5
const RADIUS: f32 = 5.0;

/// I can't literally make it a point because then it would be so hard to click it with
/// the mouse and move around
pub struct PointLight {
    pub position: glam::Vec2,
    pub wavelength: f32,
    pub ray_count: usize,

    pub bounds: AABB,
}

impl PointLight {
    pub fn new(position: Vec2, wavelength: f32, ray_count: usize) -> Self {
        let bounds = AABB::new(
            position - Vec2::splat(RADIUS),
            position + Vec2::splat(RADIUS),
        );

        dbg!(&bounds);

        Self {
            position,
            wavelength,
            ray_count,
            bounds,
        }
    }
}

impl LightSource for PointLight {
    fn get_rays(&self) -> Vec<Ray> {
        (0..self.ray_count)
            .map(|i| -> Ray {
                let angle = (i as f32 / self.ray_count as f32) * std::f32::consts::TAU;
                Ray {
                    origin: self.position,
                    direction: Vec2::new(angle.cos(), angle.sin()),
                    wavelength: self.wavelength,
                    intensity: 1.0,
                }
            })
            .collect()
    }
}

impl Drawable for PointLight {
    fn draw(&self, d: &mut raylib::drawing::RaylibDrawHandle) {
        d.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            RADIUS,
            Color::YELLOW,
        )
    }
}

impl Geometry for PointLight {
    fn intersect(&self, _: &Ray) -> Option<Intersection> {
        // Having a panic here probably here means that my design is fucked up
        // but i don't wanna deal with it right now :(
        //
        // Future me: please forgive me

        panic!("PointLight does not support intersection");
    }

    fn contains_point(&self, point: Vec2) -> bool {
        self.bounds.contains(point)
    }

    fn set_position(&mut self, position: Vec2) {
        let r = Vec2::splat(RADIUS);

        self.position = position;
        self.bounds = AABB::new(position - r, position + r);
    }
}
