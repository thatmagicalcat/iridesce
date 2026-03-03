use crate::{intersection::Intersection, ray::Ray};
use enum_dispatch::enum_dispatch;
use glam::Vec2;
use raylib::prelude::Vector2;

#[enum_dispatch]
pub trait Geometry {
    fn intersect(&self, world_ray: &Ray) -> Option<Intersection>;
    fn contains_point(&self, point: Vec2) -> bool;
    fn set_position(&mut self, position: Vec2);
}

pub fn line_ray_intersection(ray: &Ray, v1: Vec2, v2: Vec2) -> Option<Vec2> {
    let r = v2 - v1;
    let s = ray.direction;

    let q_minus_p = ray.origin - v1;
    let r_cross_s = r.cross(s);

    let t = q_minus_p.cross(s) / r_cross_s;
    let u = q_minus_p.cross(r) / r_cross_s;

    ((0.0..=1.0).contains(&t) && u >= 0.0).then(|| v1 + r * t)
}

pub trait CrossProduct2D {
    fn cross(self, other: Vec2) -> f32;
}

impl CrossProduct2D for Vec2 {
    fn cross(self, other: Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

pub trait IntoRaylibVector {
    fn into_rvec(self) -> Vector2;
}

impl IntoRaylibVector for Vec2 {
    fn into_rvec(self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
    pub wavelength: f32,
    pub intensity: f32,
}

impl LineSegment {
    /// Returns a line segment from ray origin to point of intersection
    pub fn from_ray_intersection(ray: &Ray, intersection: &Intersection) -> Self {
        Self {
            start: ray.origin,
            end: intersection.point,
            wavelength: ray.wavelength,
            intensity: ray.intensity,
        }
    }
}

impl std::ops::Index<usize> for LineSegment {
    type Output = Vec2;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.start,
            1 => &self.end,
            _ => panic!("Index out of bounds for LineSegment"),
        }
    }
}
