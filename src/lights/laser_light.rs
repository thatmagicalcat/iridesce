use egui_macroquad::egui;
use glam::Vec2;

use crate::lights::LightSource;
use crate::transform::Transform;
use crate::{
    aabb::AABB, drawable::Drawable, geometry::Geometry, intersection::Intersection, ray::Ray,
};

pub struct LaserLight {
    transform: Transform,
    direction: Vec2,
    wavelength: f32,
    bounds: AABB,
    is_dirty: bool,
}

impl LaserLight {
    pub fn new(origin: Vec2, direction: Vec2, wavelength: f32) -> Self {
        let mut bounds = AABB::new(-Vec2::splat(3.0), Vec2::splat(3.0));

        // make it easier to click on the laser light with the mouse
        bounds.expand(6.0);

        LaserLight {
            transform: Transform::identity().with_position(origin),
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
            origin: self.transform.position,
            direction: self.transform.local_to_world().transform_vector2(self.direction),
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
                ui.label(format!(
                    "({}, {})",
                    self.transform.position.x, self.transform.position.y
                ));
                ui.end_row();

                ui.label("Wavelength");
                let wavelength_changed = ui
                    .add(egui::DragValue::new(&mut self.wavelength).range(380.0..=750.0))
                    .changed();
                ui.end_row();

                ui.label("Rotation");
                let mut angle: f32 = self.transform.rotation.to_degrees();
                let rotation_changed = ui
                    .add(egui::DragValue::new(&mut angle).range(0.0..=360.0))
                    .changed();

                if rotation_changed {
                    self.transform.rotation = angle.to_radians();
                }

                ui.end_row();

                wavelength_changed || rotation_changed
            })
            .inner;
    }
}

impl Drawable for LaserLight {
    fn draw(&self) {
        let Vec2 { x: x1, y: y1 } = self.bounds.v1;
        let Vec2 { x: x2, y: y2 } = self.bounds.v2;

        let w = x2 - x1;
        let h = y2 - y1;

        macroquad::shapes::draw_rectangle_ex(
            self.transform.position.x,
            self.transform.position.y,
            w,
            h,
            macroquad::shapes::DrawRectangleParams {
                offset: (0.5, 0.5).into(),
                rotation: self.transform.rotation,
                color: macroquad::color::RED,
            },
        );
    }
}

impl Geometry for LaserLight {
    fn intersect(&self, _: &Ray) -> Option<Intersection> {
        panic!("LaserLight does not have a physical geometry to intersect with rays.")
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
