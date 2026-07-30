use std::time::{Duration, Instant};

use egui_macroquad::egui::{self, Widget};
use egui_macroquad::ui;
use glam::Vec2;

use crate::drawable::Drawable;
use crate::geometry::{Geometry, LineSegment};
use crate::lights::{LaserLight, LightSource, LightSourceEnum, PointLight};
use crate::optical_objects::{Material, OpticalObject, OpticalObjectEnum, PlaneMirror};
use crate::ray::Ray;
use crate::transform::Transform;
use crate::utils::{self, ColorIntensity};
use crate::{DEPTH, transform};

#[derive(Debug)]
pub enum SelectState {
    None,
    Object(usize, Vec2),
    Light(usize, Vec2),
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

    depth: u32,
    last_render_time: Duration,
    last_mouse_position: Option<Vec2>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
            ray_paths: vec![],
            is_dirty: true,
            select_state: SelectState::None,
            depth: DEPTH,
            last_render_time: Duration::ZERO,
            last_mouse_position: None,
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

    pub fn update(&mut self) {
        if !self.check_and_clear_dirty() {
            return;
        }

        let clock = Instant::now();
        self.ray_paths.clear();

        let mut new_active_rays = Vec::with_capacity(100);
        let mut active_rays: Vec<Ray> = self
            .lights
            .iter()
            .flat_map(|light| light.get_rays())
            .collect();

        'outer: for _ in 0..self.depth {
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
                        end: ray.origin + ray.direction * 3000.0, // arbitrary long distance
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

        self.last_render_time = clock.elapsed();
    }

    pub fn mouse_pressed(&mut self, mouse_pos: Vec2) {
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

    pub fn mouse_movement(&mut self, mouse_pos: Vec2) {
        if self.last_mouse_position == Some(mouse_pos) {
            return;
        }

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

        self.last_mouse_position = Some(mouse_pos);
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
            egui::Window::new("info").show(ctx, |ui| {
                self.draw_ui_settings(ui);
                ui.separator();
                self.draw_ui_status(ui);
                ui.separator();
                self.draw_ui_add_entities(ui);
                ui.separator();
                self.draw_ui_selected_state(ui);
            });
        });

        egui_macroquad::draw();
    }

    fn draw_ui_status(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("status_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Last Render Time");
                ui.label(format!(
                    "{:.2} ms",
                    self.last_render_time.as_secs_f64() * 1000.0
                ));
                ui.end_row();

                ui.label("Objects");
                ui.label(self.objects.len().to_string());
                ui.end_row();

                ui.label("Lights");
                ui.label(self.lights.len().to_string());
                ui.end_row();

                ui.label("Ray Paths");
                ui.label(self.ray_paths.len().to_string());
            });
    }

    fn draw_ui_settings(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Depth");
            ui.add(
                egui::Slider::new(&mut self.depth, 1..=100)
                    .step_by(1.0)
                    .show_value(true)
                    .text("bounces"),
            )
            .changed()
            .then(|| self.request_redraw());
        });
    }

    fn draw_ui_add_entities(&mut self, ui: &mut egui::Ui) {
        ui.label("Add Objects:");
        egui::ScrollArea::vertical().show(ui, |ui| {
            if ui.button("Plane Mirror").clicked() {
                self.add_object(PlaneMirror::new(
                    200.0,
                    Transform::identity()
                        .with_rotation(140.0_f32.to_radians())
                        .with_position(glam::vec2(400.0, 200.0)),
                    Material {
                        reflectivity: 0.9,
                        refractive_index: 1.0,
                    },
                ));

                self.select_state = SelectState::Object(self.objects.len() - 1, Vec2::ZERO);
            }

            if ui.button("Point Light").clicked() {
                self.add_light(PointLight::new(glam::vec2(400.0, 400.0), 700.0, 200));
                self.select_state = SelectState::Light(self.lights.len() - 1, Vec2::ZERO);
            }

            if ui.button("Laser Light").clicked() {
                self.add_light(LaserLight::new(
                    glam::vec2(400.0, 400.0),
                    glam::vec2(1.0, 0.0),
                    700.0,
                ));

                self.select_state = SelectState::Light(self.lights.len() - 1, Vec2::ZERO);
            }

            if ui
                .add(
                    egui::Button::new("Clear")
                        .fill(egui::Color32::DARK_RED)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::RED)),
                )
                .clicked()
            {
                self.objects.clear();
                self.lights.clear();
                self.select_state = SelectState::None;
                self.request_redraw();
            }
        });
    }

    fn draw_ui_selected_state(&mut self, ui: &mut egui::Ui) {
        match self.select_state {
            SelectState::Object(index, ..) => self.objects[index].draw_ui(ui),
            SelectState::Light(index, ..) => self.lights[index].draw_ui(ui),

            SelectState::None => {
                ui.label("No selection");
            }
        }

        if !self.select_state.is_none() {
            ui.separator();
            if ui.button("Remove selected").clicked() {
                match self.select_state {
                    SelectState::Object(index, ..) => {
                        self.objects.swap_remove(index);
                        self.request_redraw();
                    }

                    SelectState::Light(index, ..) => {
                        self.lights.swap_remove(index);
                        self.request_redraw();
                    }

                    SelectState::None => {}
                }

                self.select_state = SelectState::None;
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
