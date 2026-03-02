use glam::Vec2;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

use crate::drawable::Drawable;
use crate::geometry::{IntoRaylibVector, line_ray_intersection};
use crate::intersection::Intersection;
use crate::optical_objects::Material;
use crate::ray::Ray;

pub enum Surface {
    Plane {
        start: Vec2,
        end: Vec2,
        normal: Vec2,
    },

    Circle {
        center: Vec2,
        radius: f32,
    },
}

impl Surface {
    pub fn plane(start: Vec2, end: Vec2) -> Self {
        Self::Plane {
            start,
            end,
            normal: (end - start).perp().normalize(),
        }
    }

    pub fn circle(center: Vec2, radius: f32) -> Self {
        Self::Circle { center, radius }
    }

    pub fn intersect(&self, ray: &Ray, material: &Material) -> Option<Intersection> {
        match *self {
            Self::Plane { start, end, normal } => {
                line_ray_intersection(ray, start, end).map(|point| Intersection {
                    point,
                    normal: -normal * ray.direction.dot(normal).signum(),
                    sq_distance: (point - ray.origin).length_squared(),
                    material: *material,
                })
            }

            Self::Circle { center, radius } => {
                let oc = ray.origin - center;
                let a = ray.direction.dot(ray.direction);
                let b = 2.0 * oc.dot(ray.direction);
                let c = oc.dot(oc) - radius * radius;
                let discriminant = b * b - 4.0 * a * c;

                if discriminant < 0.0 {
                    return None;
                }

                let t = (-b - discriminant.sqrt()) / (2.0 * a);
                if t >= 0.0 {
                    return Some(Intersection {
                        point: ray.origin + ray.direction * t,
                        normal: (ray.origin + ray.direction * t - center).normalize(),
                        sq_distance: t * t,
                        material: *material,
                    });
                }

                None
            }
        }
    }
}

impl Drawable for Surface {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        match *self {
            Surface::Plane { start, end, .. } => {
                d.draw_line_ex(start.into_rvec(), end.into_rvec(), 3.0, Color::GRAY)
            }

            Surface::Circle { center, radius, .. } => {
                d.draw_circle_lines(center.x as i32, center.y as i32, radius, Color::GRAY)
            }
        }
    }
}
