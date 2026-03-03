use glam::vec2;
use raylib::prelude::*;

use crate::drawable::Drawable;
use crate::geometry::{Geometry, IntoRaylibVector, LineSegment};
use crate::intersection::Intersection;
use crate::lights::{LightSource, LightSourceEnum, PointLight};
use crate::optical_objects::{OpticalObject, OpticalObjectEnum, PlaneMirror};
use crate::ray::Ray;
use crate::surface::Surface;
use crate::utils::{self, ColorIntensity};

#[derive(Debug)]
pub enum SelectState {
    None,
    Object(usize),
    Light(usize),
}

impl SelectState {
    fn is_none(&self) -> bool {
        matches!(self, SelectState::None)
    }
}

pub struct World {
    objects: Vec<OpticalObjectEnum>,
    lights: Vec<LightSourceEnum>,
    // surfaces: Vec<Surface>,
    /// Line segments representing the paths of rays for rendering
    ray_paths: Vec<LineSegment>,

    /// Whether the ray paths need to be recalculated
    updated: bool,

    /// TODO: use selected state to show some UI for editing the object/light properties
    select_state: SelectState,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
            ray_paths: vec![],
            updated: true,
            select_state: SelectState::None,
        }
    }

    pub fn add_object(&mut self, object: impl Into<OpticalObjectEnum>) {
        self.objects.push(object.into());
        self.updated = true;
    }

    pub fn add_light(&mut self, light: impl Into<LightSourceEnum>) {
        self.lights.push(light.into());
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

    pub fn handle_event(&mut self, rl: &RaylibHandle) {
        let Vector2 { x, y } = rl.get_mouse_position();
        let mouse_pos = vec2(x, y);

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            self.select_state = SelectState::None;
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            dbg!(mouse_pos);
            if let Some((index, _)) = self
                .objects
                .iter()
                .enumerate()
                .find(|(_, obj)| obj.contains_point(mouse_pos))
            {
                self.select_state = SelectState::Object(index);
            } else if let Some((index, _)) = self
                .lights
                .iter()
                .enumerate()
                .find(|(_, light)| light.contains_point(mouse_pos))
            {
                self.select_state = SelectState::Light(index);
            }
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            match self.select_state {
                SelectState::Object(index) => {
                    self.objects[index].set_position(mouse_pos);
                    self.request_redraw();
                }

                SelectState::Light(index) => {
                    self.lights[index].set_position(mouse_pos);
                    self.request_redraw();
                }

                SelectState::None => {}
            }
        }
    }

    #[inline(always)]
    fn request_redraw(&mut self) {
        self.updated = true;
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
