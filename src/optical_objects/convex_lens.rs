use std::f32::consts::PI;

use crate::{
    aabb::AABB,
    drawable::Drawable,
    geometry::Geometry,
    intersection::Intersection,
    optical_objects::OpticalObject,
    ray::Ray,
    surface::{CircularSurface, Surface},
    transform::Transform,
};
use egui_macroquad::egui::debug_text::print;
use glam::{Vec2, vec2};

use super::Material;

pub struct ConvexLens {
    transform: Transform,
    focal_length: f32,
    surfaces: [CircularSurface; 2],
    material: Material,
    aperture: f32,
    bounds: AABB,
    is_dirty: bool,
}

impl ConvexLens {
    pub fn new(position: Vec2, focal_length: f32, aperture: f32, material: Material) -> Self {
        // lens maker formula
        let r = 2.0 * focal_length * (material.refractive_index - 1.0);
        let half_height = aperture / 2.0;

        assert!(
            r >= half_height,
            "Focal length is too short for this aperture!"
        );

        let d = (r * r - half_height * half_height).sqrt();

        let theta = (half_height / r).asin();
        let left_center = vec2(d, 0.0);
        let left_surface = CircularSurface::new(left_center, r, PI - theta, PI + theta);

        // Right-facing surface (bulges right, so its center of curvature is on the LEFT at -d)
        let right_center = vec2(-d, 0.0);
        let right_surface = CircularSurface::new(
            right_center,
            r,
            -theta, // down
            theta,  // up
        );

        Self {
            transform: Transform::identity().with_position(position),
            surfaces: [left_surface, right_surface],
            material,
            is_dirty: true,
            focal_length,
            aperture,
            bounds: AABB::new(
                position - vec2(d + r, half_height),
                position + vec2(d + r, half_height),
            ),
        }
    }
}

impl Geometry for ConvexLens {
    fn intersect(&self, world_ray: &Ray) -> Option<Intersection> {
        let inverse_transform = self.transform.world_to_local();
        let local_ray = Ray {
            origin: inverse_transform.transform_point2(world_ray.origin),
            direction: inverse_transform.transform_vector2(world_ray.direction),
            wavelength: world_ray.wavelength,
            intensity: world_ray.intensity,
        };

        let transform = self.transform.local_to_world();
        let local_hit = self
            .surfaces
            .iter()
            .filter_map(|surface| surface.intersect(&local_ray, &self.material))
            .min_by(|a, b| a.sq_distance.partial_cmp(&b.sq_distance).unwrap())?;

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
        let local_point = self.transform.world_to_local().transform_point2(point);
        self.bounds.contains(local_point)
    }

    fn set_position(&mut self, position: Vec2) {
        self.transform.position = position;
    }

    fn get_position(&self) -> Vec2 {
        self.transform.position
    }
}

impl Drawable for ConvexLens {
    fn draw(&self) {
        let local_to_world = self.transform.local_to_world();

        for &CircularSurface {
            center,
            radius,
            start_angle,
            end_angle,
        } in &self.surfaces
        {
            let center = local_to_world.transform_point2(center);
            let rot = self.transform.rotation;

            macroquad::shapes::draw_arc(
                center.x,
                center.y,
                !0,
                radius,
                (start_angle + rot).to_degrees(),
                2.0,
                (end_angle - start_angle).to_degrees(),
                macroquad::color::WHITE,
            );
        }
    }
}

impl OpticalObject for ConvexLens {
    fn handle_intersection(&self, ray: &Ray, intersection: &Intersection) -> Vec<Ray> {
        let refracted_intensity = ray.intensity * (1.0 - self.material.reflectivity);
        let reflected_intensity = ray.intensity * self.material.reflectivity;

        if ray.direction.dot(intersection.normal) < 0.0 {
            // outside

            let refracted_ray = Ray {
                origin: intersection.point + intersection.normal * 0.001,
                direction: ray
                    .direction
                    .refract(intersection.normal, 1.0 / self.material.refractive_index),
                wavelength: ray.wavelength,
                intensity: refracted_intensity,
            };

            let reflected_ray = Ray {
                origin: intersection.point + intersection.normal * 0.001,
                direction: ray.direction.reflect(intersection.normal),
                wavelength: ray.wavelength,
                intensity: reflected_intensity,
            };

            vec![refracted_ray, reflected_ray]
        } else {
            // inside

            let refracted_ray = Ray {
                origin: intersection.point - intersection.normal * 0.001,
                direction: ray
                    .direction
                    .refract(-intersection.normal, self.material.refractive_index),
                wavelength: ray.wavelength,
                intensity: refracted_intensity,
            };

            let reflected_ray = Ray {
                origin: intersection.point - intersection.normal * 0.001,
                direction: ray.direction.reflect(-intersection.normal),
                wavelength: ray.wavelength,
                intensity: reflected_intensity,
            };

            vec![refracted_ray, reflected_ray]
        }
    }

    fn check_and_clear_dirty(&mut self) -> bool {
        false
    }

    fn draw_ui(&mut self, ui: &mut egui_macroquad::egui::Ui) {
        todo!()
    }
}
