use egui_macroquad::egui;
use glam::Vec2;

use super::LightSource;
use crate::{
    aabb::AABB, drawable::Drawable, geometry::Geometry, intersection::Intersection, ray::Ray,
};

/// Circle of arbitrary radius 5
const RADIUS: f32 = 5.0;

/// I can't literally make it a point because then it would be so hard to click it with
/// the mouse and move around
pub struct PointLight {
    position: glam::Vec2,
    wavelength: f32,
    ray_count: usize,
    bounds: AABB,

    is_dirty: bool,
}

impl PointLight {
    pub fn new(position: Vec2, wavelength: f32, ray_count: usize) -> Self {
        let bounds = AABB::new(
            position - Vec2::splat(RADIUS),
            position + Vec2::splat(RADIUS),
        );

        Self {
            position,
            wavelength,
            ray_count,
            bounds,
            is_dirty: true,
        }
    }
}

impl LightSource for PointLight {
    fn get_rays(&self) -> Vec<Ray> {
        (0..self.ray_count)
            .map(|i| -> Ray {
                let angle = (i as f32 / self.ray_count as f32) * std::f32::consts::TAU;
                Ray {
                    origin: self.position,
                    direction: Vec2::new(angle.cos(), angle.sin()),
                    wavelength: self.wavelength,
                    intensity: 1.0,
                }
            })
            .collect()
    }

    fn check_and_clear_dirty(&mut self) -> bool {
        let was_dirty = self.is_dirty;
        self.is_dirty = false;
        was_dirty
    }

    fn draw_ui(&mut self, ui: &mut egui_macroquad::egui::Ui){
        self.is_dirty |= egui::Grid::new("point_light_properties")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Position");
                ui.label(format!("({:.1}, {:.1})", self.position.x, self.position.y));
                ui.end_row();

                ui.label("Wavelength");
                let wavelength_changed = ui
                    .add(egui::DragValue::new(&mut self.wavelength).clamp_range(380.0..=750.0))
                    .changed();
                ui.end_row();

                ui.label("Ray Count");
                let ray_count_changed = ui
                    .add(egui::DragValue::new(&mut self.ray_count).clamp_range(1..=1000))
                    .changed();
                ui.end_row();

                wavelength_changed || ray_count_changed
            })
            .inner;
    }
}

impl Drawable for PointLight {
    fn draw(&self) {
        macroquad::shapes::draw_circle(
            self.position.x,
            self.position.y,
            RADIUS,
            macroquad::color::YELLOW,
        );
    }
}

impl Geometry for PointLight {
    fn intersect(&self, _: &Ray) -> Option<Intersection> {
        // Having a panic here probably here means that my design is fucked up
        // but i don't wanna deal with it right now :(
        //
        // Future me: please forgive me

        panic!("PointLight does not support intersection");
    }

    fn contains_point(&self, point: Vec2) -> bool {
        self.bounds.contains(point)
    }

    fn set_position(&mut self, position: Vec2) {
        let r = Vec2::splat(RADIUS);

        self.position = position;
        self.bounds = AABB::new(position - r, position + r);
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }
}
