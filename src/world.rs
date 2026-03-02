use raylib::prelude::*;

use crate::drawable::Drawable;
use crate::geometry::{IntoRaylibVector, LineSegment};
use crate::intersection::Intersection;
use crate::light::LightSource;
use crate::optical_objects::{OpticalObjectEnum, PlaneMirror, OpticalObject};
use crate::ray::Ray;
use crate::surface::Surface;
use crate::utils::{self, ColorIntensity};

pub struct World {
    objects: Vec<OpticalObjectEnum>,
    // surfaces: Vec<Surface>,
    lights: Vec<LightSource>,

    /// Line segments representing the paths of rays for rendering
    ray_paths: Vec<LineSegment>,

    /// Whether the ray paths need to be recalculated
    updated: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
            ray_paths: vec![],
            updated: true,
        }
    }

    pub fn add_object<T>(&mut self, object: T)
    where
        T: Into<OpticalObjectEnum>,
    {
        self.objects.push(object.into());
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
                let Some(closest_intersection) = self
                    .objects
                    .iter()
                    .filter_map(|object| object.intersect(ray))
                    .min_by(|a, b| {
                        a.sq_distance
                            .partial_cmp(&b.sq_distance)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                else {
                    self.ray_paths.push(LineSegment {
                        start: ray.origin,
                        end: ray.origin + ray.direction * 1000.0, // arbitrary long distance
                        wavelength: ray.wavelength,
                        intensity: ray.intensity,
                    });

                    continue;
                };

                self.ray_paths.push(LineSegment::from_ray_intersection(
                    ray,
                    &closest_intersection,
                ));

                let intensity = ray.intensity * closest_intersection.material.reflectivity;
                if ray.intensity < 0.01 {
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
}

impl Drawable for World {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        for ray_path in self.ray_paths.iter() {
            let color =
                utils::wavelength_to_rgb(ray_path.wavelength as _).intensity(ray_path.intensity);
            d.draw_line_ex(ray_path[0].into_rvec(), ray_path[1].into_rvec(), 1.0, color);
        }

        for obj in self.objects.iter() {
            obj.draw(d);
        }

        self.lights.iter().for_each(|light| light.draw(d));
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
