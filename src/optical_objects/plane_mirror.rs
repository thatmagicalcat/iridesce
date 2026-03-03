use egui_macroquad::egui;
use glam::vec2;

use crate::{
    aabb::AABB,
    surface::{PlaneSurface, SurfaceEnum},
    transform::Transform,
};

use super::*;

pub struct PlaneMirror {
    surface: PlaneSurface,
    transform: Transform,
    material: Material,
    one_side: bool,
    bounds: AABB,
    is_dirty: bool,
}

impl PlaneMirror {
    pub fn new(length: f32, transform: Transform, material: Material, one_side: bool) -> Self {
        let plane = PlaneSurface::new(vec2(-length * 0.5, 0.0), vec2(length * 0.5, 0.0));
        let mut bounds = AABB::new(plane.start.min(plane.end), plane.start.max(plane.end));

        // make it easier to click on the plane mirror with the mouse
        bounds.expand(3.0);

        Self {
            surface: plane,
            transform,
            material,
            one_side,
            bounds,
            is_dirty: true,
        }
    }
}

impl OpticalObject for PlaneMirror {
    fn handle_intersection(&self, ray: &Ray, intersection: &Intersection) -> Vec<Ray> {
        let intensity = ray.intensity * intersection.material.reflectivity;

        if ray.intensity < 0.01 {
            return vec![];
        }

        // r = d - 2 (d . n) n
        vec![Ray {
            //                                 ------------- prevent self intersection
            origin: intersection.point + intersection.normal * 0.001,
            direction: ray.direction
                - 2.0 * ray.direction.dot(intersection.normal) * intersection.normal,
            wavelength: ray.wavelength,
            intensity,
        }]
    }

    fn check_and_clear_dirty(&mut self) -> bool {
        let was_dirty = self.is_dirty;
        self.is_dirty = false;
        was_dirty
    }

    fn draw_ui(&mut self, ui: &mut egui_macroquad::egui::Ui) {
        self.is_dirty |= egui::Grid::new("plane_mirror_properties")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Position");
                ui.label(format!(
                    "({}, {})",
                    self.transform.position.x, self.transform.position.y
                ));
                ui.end_row();

                ui.label("Rotation");
                let rotation_changed = ui
                    .add(egui::DragValue::new(&mut self.transform.rotation).speed(0.1))
                    .changed();
                ui.end_row();

                ui.label("Reflectivity");
                let reflectivity_changed = ui
                    .add(
                        egui::DragValue::new(&mut self.material.reflectivity)
                            .range(0.0..=1.0)
                            .speed(0.01),
                    )
                    .changed();
                ui.end_row();

                rotation_changed || reflectivity_changed
            })
            .inner;
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

        let local_hit = self.surface.intersect(&local_ray, &self.material)?;
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

impl Drawable for PlaneMirror {
    fn draw(&self) {
        let start = self
            .transform
            .local_to_world()
            .transform_point2(self.surface.start);
        let end = self.transform.local_to_world().transform_point2(self.surface.end);

        macroquad::shapes::draw_line(start.x, start.y, end.x, end.y, 2.0, macroquad::color::WHITE);
    }
}
