use raylib::prelude::*;

use crate::drawable::Drawable;
use crate::geometry::{IntoRaylibVector, LineSegment};
use crate::intersection::Intersection;
use crate::light::LightSource;
use crate::ray::Ray;
use crate::surface::SurfaceShape;
use crate::utils::{self, ColorIntensity};

pub struct World {
    surfaces: Vec<SurfaceShape>,
    lights: Vec<LightSource>,

    /// Line segments representing the paths of rays for rendering
    ray_paths: Vec<LineSegment>,

    /// Whether the ray paths need to be recalculated
    updated: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            surfaces: Vec::new(),
            lights: Vec::new(),
            ray_paths: Vec::new(),
            updated: true,
        }
    }

    pub fn add_surface(&mut self, surface: SurfaceShape) {
        self.surfaces.push(surface);
        self.updated = true;
    }

    pub fn add_light(&mut self, light: LightSource) {
        self.lights.push(light);
        self.updated = true;
    }

    pub fn calculate_ray_paths(&mut self, depth: u32) {
        if !self.updated {
            return;
        }

        self.ray_paths.clear();

        let mut new_active_rays = Vec::with_capacity(100);
        let mut active_rays: Vec<Ray> = self
            .lights
            .iter()
            .flat_map(|light| light.get_rays())
            .collect();

        'outer: for _ in 0..depth {
            for ray in active_rays.iter() {
                let Some(closest_intersection) = self.closest_intersection(ray) else {
                    // somewhere outside the screen
                    // FIXME: Calculate this based on camera's position in future
                    let far_point = ray.origin + ray.direction * 1000.0;
                    self.ray_paths.push(LineSegment {
                        start: ray.origin,
                        end: far_point,
                        wavelength: ray.wavelength,
                        intensity: ray.intensity,
                    });

                    continue;
                };

                self.ray_paths.push(LineSegment::from_ray_intersection(ray, &closest_intersection));

                let intensity = ray.intensity * closest_intersection.reflectivity;

                // No need to trace further if the intensity is too low
                if intensity < 0.01 {
                    continue;
                }

                // r = d - 2 (d . n) n
                let reflected_ray = Ray {
                    //                                 ------------- prevent self intersection
                    origin: closest_intersection.point + closest_intersection.normal * 0.001,
                    direction: ray.direction
                        - 2.0
                            * ray.direction.dot(closest_intersection.normal)
                            * closest_intersection.normal,
                    wavelength: ray.wavelength,
                    intensity,
                };

                new_active_rays.push(reflected_ray);
                // TODO: Refraction
            }

            // no intersections, stop tracing
            if new_active_rays.is_empty() {
                break 'outer;
            }

            std::mem::swap(&mut active_rays, &mut new_active_rays);
            new_active_rays.clear();
        }

        self.updated = false;
    }

    /// Find the closest intersection of a ray with the surfaces in the world
    pub fn closest_intersection(&self, ray: &Ray) -> Option<Intersection> {
        self.surfaces
            .iter()
            .filter_map(|surface| surface.intersect(ray))
            .min_by(|a, b| {
                a.sq_distance
                    .partial_cmp(&b.sq_distance)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}

impl Drawable for World {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        for ray_path in self.ray_paths.iter() {
            let color =
                utils::wavelength_to_rgb(ray_path.wavelength as _).intensity(ray_path.intensity);
            d.draw_line_ex(ray_path[0].into_rvec(), ray_path[1].into_rvec(), 1.0, color);
        }

        for surface in self.surfaces.iter() {
            surface.draw(d);
        }

        self.lights.iter().for_each(|light| light.draw(d));
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
