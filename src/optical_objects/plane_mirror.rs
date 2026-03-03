use glam::vec2;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

use crate::{aabb::AABB, geometry::IntoRaylibVector, surface::Surface, transform::Transform};

use super::*;

pub struct PlaneMirror {
    pub surface: Surface,
    pub transform: Transform,
    pub material: Material,
    pub one_side: bool,
    pub bounds: AABB,
}

impl PlaneMirror {
    pub fn new(surface: Surface, transform: Transform, material: Material, one_side: bool) -> Self {
        let bounds = match surface {
            Surface::Plane { start, end, .. } => AABB::new(start.min(end), start.max(end)),
            Surface::Circle { center, radius } => {
                AABB::new(center - vec2(radius, radius), center + vec2(radius, radius))
            }
        };

        Self {
            surface,
            transform,
            material,
            one_side,
            bounds,
        }
    }
}

impl OpticalObject for PlaneMirror {
    fn handle_intersection(&self, ray: &Ray, intersection: &Intersection) -> Vec<Ray> {
        todo!()
    }
}

// Maybe make everything in world space... as it can be a bit confusing to have
// some things in world space and some things in local space.
//
// Maybe the geometry should be in world space, and the transform is just for drawing?
// idk.. I'll think about this later.
//
// This is fine :)
impl Geometry for PlaneMirror {
    fn intersect(&self, world_ray: &Ray) -> Option<Intersection> {
        // get the ray inside the local space
        let inverse_transform = self.transform.world_to_local();

        let local_ray = Ray {
            origin: inverse_transform.transform_point2(world_ray.origin),
            direction: inverse_transform
                .transform_vector2(world_ray.direction)
                .normalize(),
            wavelength: world_ray.wavelength,
            intensity: world_ray.intensity,
        };

        let local_hit = self.closest_intersection(&local_ray)?;
        let transform = self.transform.local_to_world();

        Some(Intersection {
            point: transform.transform_point2(local_hit.point),
            normal: transform.transform_vector2(local_hit.normal).normalize(),
            sq_distance: transform
                .transform_point2(local_hit.point)
                .distance_squared(world_ray.origin),
            material: local_hit.material,
        })
    }

    fn contains_point(&self, point: Vec2) -> bool {
        self.bounds.contains(point)
    }

    fn set_position(&mut self, position: Vec2) {
        let w = self.bounds.v2.x - self.bounds.v1.x;
        let h = self.bounds.v2.y - self.bounds.v1.y;
        let half_dim = vec2(w, h) * 0.5;

        self.bounds = AABB::new(position - half_dim, position + half_dim);
        self.transform.position = position;
        self.surface.set_position(position);
    }
}

impl Drawable for PlaneMirror {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        match self.surface {
            Surface::Plane { start, end, .. } => {
                let start = self.transform.local_to_world().transform_point2(start);
                let end = self.transform.local_to_world().transform_point2(end);
                d.draw_line_v(start.into_rvec(), end.into_rvec(), Color::WHITE);
            }

            Surface::Circle { center, radius } => {
                let center = self.transform.local_to_world().transform_point2(center);
                let scale = self
                    .transform
                    .local_to_world()
                    .transform_vector2(vec2(radius, 0.0))
                    .length();
                d.draw_circle_v(center.into_rvec(), scale, Color::WHITE);
            }
        }
    }
}

impl PlaneMirror {
    pub fn closest_intersection(&self, ray: &Ray) -> Option<Intersection> {
        self.surface.intersect(ray, &self.material)
    }
}

// impl Drawable for PlaneMirror {
//     fn draw(&self, d: &mut RaylibDrawHandle) {
//     }
// }
