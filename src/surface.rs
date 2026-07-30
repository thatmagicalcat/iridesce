use std::f32::consts::PI;

use enum_dispatch::enum_dispatch;
use glam::Vec2;

use crate::geometry::line_ray_intersection;
use crate::intersection::Intersection;
use crate::optical_objects::Material;
use crate::ray::Ray;

#[enum_dispatch]
pub trait Surface {
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

impl Surface for PlaneSurface {
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
    pub center: Vec2,
    pub radius: f32,
    pub start_angle: f32,
    pub end_angle: f32,
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

    fn is_angle_on_arc(&self, angle: f32) -> bool {
        let mut angle = angle;
        let mut start = self.start_angle;
        let mut end = self.end_angle;

        let normalize = |a: f32| (a % (2.0 * PI) + 2.0 * PI) % (2.0 * PI);
        angle = normalize(angle);
        start = normalize(start);
        end = normalize(end);

        if start <= end {
            angle >= start && angle <= end
        } else {
            angle >= start || angle <= end
        }
    }
}

impl Surface for CircularSurface {
    fn set_position(&mut self, position: Vec2) {
        self.center = position;
    }

    fn intersect(&self, ray: &Ray, material: &Material) -> Option<Intersection> {
        let l = ray.origin - self.center;
        let b = 2.0 * ray.direction.dot(l);
        let c = l.dot(l) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let t_values = [(-b - sqrt_d) / 2.0, (-b + sqrt_d) / 2.0];

        for &t in &t_values {
            if t < 0.0 {
                continue;
            }

            let point = ray.origin + ray.direction * t;
            let hit_vector = point - self.center;
            let angle = hit_vector.y.atan2(hit_vector.x);

            if self.is_angle_on_arc(angle) {
                return Some(Intersection {
                    point,
                    normal: hit_vector.normalize(),
                    sq_distance: t * t,
                    material: *material,
                });
            }
        }

        None
    }

    // fn intersect(&self, ray: &Ray, material: &Material) -> Option<Intersection> {
    //     let oc = ray.origin - self.center;
    //     let a = ray.direction.dot(ray.direction);
    //     let b = 2.0 * oc.dot(ray.direction);
    //     let c = oc.dot(oc) - self.radius * self.radius;
    //     let discriminant = b * b - 4.0 * a * c;
    //
    //     if discriminant < 0.0 {
    //         return None;
    //     }
    //
    //     let t = (-b - discriminant.sqrt()) / (2.0 * a);
    //     if t < 0.0 {
    //         return None;
    //     }
    //
    //     let hit_point = ray.origin + ray.direction * t;
    //     let local_point = hit_point - self.center;
    //     let angle = local_point.y.atan2(local_point.x).abs();
    //
    //     (self.start_angle..self.end_angle)
    //         .contains(&angle)
    //         .then(|| Intersection {
    //             point: hit_point,
    //             normal: (ray.origin + ray.direction * t - self.center).normalize(),
    //             sq_distance: t * t,
    //             material: *material,
    //         })
    // }

    // fn intersect(&self, ray: &Ray, material: &Material) -> Option<Intersection> {
    //     let oc = ray.origin - self.center;
    //     let a = ray.direction.dot(ray.direction);
    //     let b = 2.0 * oc.dot(ray.direction);
    //     let c = oc.dot(oc) - self.radius * self.radius;
    //     let discriminant = b * b - 4.0 * a * c;
    //     if discriminant < 0.0 {
    //         return None;
    //     }
    //     let discriminant_sqrt = discriminant.sqrt();
    //
    //     let t1 = (-b - discriminant_sqrt) / (2.0 * a);
    //     let t2 = (-b + discriminant_sqrt) / (2.0 * a);
    //
    //     for t in [t1, t2] {
    //         // prevent self-intersection bugs
    //         if t < 0.001 {
    //             continue;
    //         }
    //
    //         let hit_point = ray.origin + ray.direction * t;
    //         let local_point = hit_point - self.center;
    //         let angle = local_point.y.atan2(local_point.x).abs();
    //
    //         if (self.start_angle..self.end_angle).contains(&angle) {
    //             return Some(Intersection {
    //                 point: hit_point,
    //                 normal: (hit_point - self.center).normalize(),
    //                 sq_distance: t * t,
    //                 material: *material,
    //             });
    //         }
    //     }
    //     None
    // }
}

#[enum_dispatch(SurfaceType)]
pub enum SurfaceEnum {
    Plane(PlaneSurface),
    Circular(CircularSurface),
}
