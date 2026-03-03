use egui_macroquad::egui;
use glam::Vec2;

use crate::lights::LightSource;
use crate::{
    aabb::AABB, drawable::Drawable, geometry::Geometry, intersection::Intersection, ray::Ray,
};

pub struct LaserLight {
    origin: Vec2,
    direction: Vec2,
    wavelength: f32,
    bounds: AABB,
    is_dirty: bool,
}

impl LaserLight {
    pub fn new(origin: Vec2, direction: Vec2, wavelength: f32) -> Self {
        let bounds = AABB::new(origin - direction * 20.0, origin + direction * 20.0);
        LaserLight {
            origin,
            direction: direction.normalize(),
            wavelength,
            bounds,
            is_dirty: true,
        }
    }
}

impl LightSource for LaserLight {
    fn get_rays(&self) -> Vec<Ray> {
        vec![Ray {
            origin: self.origin,
            direction: self.direction,
            wavelength: self.wavelength,
            intensity: 1.0,
        }]
    }

    fn check_and_clear_dirty(&mut self) -> bool {
        let was_dirty = self.is_dirty;
        self.is_dirty = false;
        was_dirty
    }

    fn draw_ui(&mut self, ui: &mut egui_macroquad::egui::Ui) {
        self.is_dirty |= egui::Grid::new("laser_light_properties")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Position");
                ui.label(format!("({}, {})", self.origin.x, self.origin.y));
                ui.end_row();

                ui.label("Wavelength");
                let wavelength_changed = ui
                    .add(egui::DragValue::new(&mut self.wavelength).range(380.0..=750.0))
                    .changed();
                ui.end_row();

                wavelength_changed
            })
            .inner;
    }
}

impl Drawable for LaserLight {
    fn draw(&self) {
        let end = self.origin + self.direction * 20.0;
        macroquad::shapes::draw_line(
            self.origin.x,
            self.origin.y,
            end.x,
            end.y,
            2.0,
            macroquad::color::YELLOW,
        );
    }
}

impl Geometry for LaserLight {
    fn intersect(&self, _: &Ray) -> Option<Intersection> {
        panic!("LaserLight does not have a physical geometry to intersect with rays.")
    }

    fn contains_point(&self, point: Vec2) -> bool {
        self.bounds.contains(point)
    }

    fn set_position(&mut self, position: Vec2) {
        self.origin = position;
    }

    fn get_position(&self) -> Vec2 {
        self.origin
    }
}
