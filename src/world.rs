use egui_macroquad::egui::{self, Widget};

use crate::drawable::Drawable;
use crate::geometry::{Geometry, LineSegment};
use crate::lights::{LightSource, LightSourceEnum};
use crate::optical_objects::{OpticalObject, OpticalObjectEnum};
use crate::ray::Ray;
use crate::utils::{self, ColorIntensity};

#[derive(Debug)]
pub enum SelectState {
    None,
    Object(usize, glam::Vec2),
    Light(usize, glam::Vec2),
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
    is_dirty: bool,

    /// TODO: use selected state to show some UI for editing the object/light properties
    select_state: SelectState,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
            ray_paths: vec![],
            is_dirty: true,
            select_state: SelectState::None,
        }
    }

    pub fn add_object(&mut self, object: impl Into<OpticalObjectEnum>) {
        self.objects.push(object.into());
        self.is_dirty = true;
    }

    pub fn add_light(&mut self, light: impl Into<LightSourceEnum>) {
        self.lights.push(light.into());
        self.is_dirty = true;
    }

    pub fn check_and_clear_dirty(&mut self) -> bool {
        let was_dirty = self.is_dirty
            || self
                .lights
                .iter_mut()
                .any(|light| light.check_and_clear_dirty())
            || self
                .objects
                .iter_mut()
                .any(|obj| obj.check_and_clear_dirty());

        self.is_dirty = false;
        was_dirty
    }

    pub fn update(&mut self, depth: u32) {
        if !self.check_and_clear_dirty() {
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
                let Some((obj_index, closest_intersection)) = self
                    .objects
                    .iter()
                    .enumerate()
                    .filter_map(|(i, object)| object.intersect(ray).map(|int| (i, int)))
                    .min_by(|(_, a), (_, b)| {
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

                new_active_rays.extend(
                    self.objects[obj_index].handle_intersection(ray, &closest_intersection),
                );
            }

            // no intersections, stop tracing
            if new_active_rays.is_empty() {
                break 'outer;
            }

            std::mem::swap(&mut active_rays, &mut new_active_rays);
            new_active_rays.clear();
        }
    }

    pub fn mouse_pressed(&mut self, mouse_pos: glam::Vec2) {
        if let Some((index, obj)) = self
            .objects
            .iter()
            .enumerate()
            .find(|(_, obj)| obj.contains_point(mouse_pos))
        {
            let offset = obj.get_position() - mouse_pos;
            self.select_state = SelectState::Object(index, offset);
        } else if let Some((index, light)) = self
            .lights
            .iter()
            .enumerate()
            .find(|(_, light)| light.contains_point(mouse_pos))
        {
            let offset = light.get_position() - mouse_pos;
            self.select_state = SelectState::Light(index, offset);
        } else {
            self.select_state = SelectState::None;
        }
    }

    pub fn mouse_movement(&mut self, mouse_pos: glam::Vec2) {
        match self.select_state {
            SelectState::Object(index, offset) => {
                self.objects[index].set_position(mouse_pos + offset);
                self.request_redraw();
            }

            SelectState::Light(index, offset) => {
                self.lights[index].set_position(mouse_pos + offset);
                self.request_redraw();
            }

            SelectState::None => {}
        }
    }

    #[inline(always)]
    fn request_redraw(&mut self) {
        self.is_dirty = true;
    }

    pub fn draw(&mut self) {
        for ray_path in self.ray_paths.iter() {
            let color =
                utils::wavelength_to_rgb(ray_path.wavelength as _).intensity(ray_path.intensity);

            macroquad::shapes::draw_line(
                ray_path.start.x,
                ray_path.start.y,
                ray_path.end.x,
                ray_path.end.y,
                1.0,
                color,
            );
        }

        for obj in self.objects.iter() {
            obj.draw();
        }

        self.lights.iter().for_each(|light| light.draw());

        egui_macroquad::ui(|ctx| {
            egui::Window::new("info").show(ctx, |ui| match self.select_state {
                SelectState::Object(index, ..) => self.objects[index].draw_ui(ui),
                SelectState::Light(index, ..) => self.lights[index].draw_ui(ui),

                SelectState::None => {
                    ui.label("No selection");
                }
            });
        });

        egui_macroquad::draw();
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
