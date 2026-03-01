use crate::drawable::Drawable;
use crate::geometry::{IntoRaylibVector, line_ray_intersection};
use crate::intersection::Intersection;
use crate::ray::Ray;
use glam::Vec2;
use raylib::prelude::*;

pub enum SurfaceShape {
    Plane {
        start: Vec2,
        end: Vec2,
        normal: Vec2,
        reflectivity: f32,
    },

    Circle {
        center: Vec2,
        radius: f32,
        reflectivity: f32,
    },
}

impl SurfaceShape {
    pub fn plane(start: Vec2, end: Vec2, reflectivity: f32) -> Self {
        Self::Plane {
            start,
            end,
            normal: (end - start).perp().normalize(),
            reflectivity,
        }
    }

    pub fn circle(center: Vec2, radius: f32, reflectivity: f32) -> Self {
        Self::Circle {
            center,
            radius,
            reflectivity,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match *self {
            SurfaceShape::Plane {
                start,
                end,
                normal,
                reflectivity,
            } => line_ray_intersection(ray, start, end).map(|point| Intersection {
                point,
                normal: -normal * ray.direction.dot(normal).signum(),
                sq_distance: (point - ray.origin).length_squared(),
                reflectivity,
            }),

            SurfaceShape::Circle { center, radius, reflectivity  } => {
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
                        reflectivity,
                    });
                }

                None
            }
        }
    }
}

impl Drawable for SurfaceShape {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        match *self {
            SurfaceShape::Plane { start, end, .. } => {
                d.draw_line_ex(start.into_rvec(), end.into_rvec(), 3.0, Color::GRAY)
            }

            SurfaceShape::Circle { center, radius, .. } => {
                d.draw_circle_lines(center.x as i32, center.y as i32, radius, Color::GRAY)
            }
        }
    }
}
