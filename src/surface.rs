use enum_dispatch::enum_dispatch;
use glam::Vec2;

use crate::geometry::line_ray_intersection;
use crate::intersection::Intersection;
use crate::optical_objects::Material;
use crate::ray::Ray;

#[enum_dispatch]
pub trait SurfaceEnum {
    fn set_position(&mut self, position: Vec2);
    fn intersect(&self, ray: &Ray, material: &Material) -> Option<Intersection>;
}

pub struct PlaneSurface {
    pub start: Vec2,
    pub end: Vec2,
    pub normal: Vec2,
}

impl PlaneSurface {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self {
            start,
            end,
            normal: (end - start).perp().normalize(),
        }
    }
}

impl SurfaceEnum for PlaneSurface {
    fn set_position(&mut self, position: Vec2) {
        let center = (self.start + self.end) * 0.5;
        let offset = position - center;
        self.start += offset;
        self.end += offset;
    }

    fn intersect(&self, ray: &Ray, material: &Material) -> Option<Intersection> {
        line_ray_intersection(ray, self.start, self.end).map(|point| Intersection {
            point,
            normal: -self.normal * ray.direction.dot(self.normal).signum(),
            sq_distance: (point - ray.origin).length_squared(),
            material: *material,
        })
    }
}

pub struct CircularSurface {
    center: Vec2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
}

impl CircularSurface {
    pub fn new(center: Vec2, radius: f32, start_angle: f32, end_angle: f32) -> Self {
        Self {
            center,
            radius,
            start_angle,
            end_angle,
        }
    }
}

impl SurfaceEnum for CircularSurface {
    fn set_position(&mut self, position: Vec2) {
        self.center = position;
    }

    fn intersect(&self, ray: &Ray, material: &Material) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t >= 0.0 {
            return Some(Intersection {
                point: ray.origin + ray.direction * t,
                normal: (ray.origin + ray.direction * t - self.center).normalize(),
                sq_distance: t * t,
                material: *material,
            });
        }

        None
    }
}

#[enum_dispatch(SurfaceType)]
pub enum Surface {
    Plane(PlaneSurface),
    Circular(CircularSurface),
}
