use ::glam::vec2;
use macroquad::prelude::*;

mod aabb;
mod drawable;
mod geometry;
mod intersection;
mod lights;
mod optical_objects;
mod ray;
mod surface;
mod transform;
mod utils;
mod world;

use world::World;

use crate::{
    lights::PointLight,
    optical_objects::{ConvexLens, Material, PlaneMirror},
};

const DEPTH: u32 = 5;

fn window_conf() -> Conf {
    Conf {
        window_title: "Reflections".to_string(),
        window_width: 800,
        window_height: 800,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();

    // world.add_object(PlaneMirror::new(
    //     400.0,
    //     transform::Transform::identity()
    //         .with_rotation(140.0_f32.to_radians())
    //         .with_position(vec2(400.0, 200.0)),
    //     Material {
    //         reflectivity: 0.9,
    //         refractive_index: 1.0,
    //     },
    // ));
    //
    // world.add_object(PlaneMirror::new(
    //     100.0,
    //     transform::Transform::identity()
    //         .with_rotation(90.0_f32.to_radians())
    //         .with_position(vec2(500.0, 500.0)),
    //     Material {
    //         reflectivity: 0.9,
    //         refractive_index: 1.0,
    //     },
    // ));

    world.add_object(ConvexLens::new(
        vec2(400.0, 400.0),
        100.0,
        150.0,
        Material {
            reflectivity: 0.1,
            refractive_index: 1.5,
        },
    ));

    // world.add_light(PointLight::new(vec2(400.0, 400.0), 700.0, 200));

    loop {
        egui_macroquad::cfg(|ctx| {
            if !ctx.wants_pointer_input() {
                if is_mouse_button_pressed(MouseButton::Left) {
                    world.mouse_pressed(mouse_position().into());
                }

                if is_mouse_button_down(MouseButton::Left) {
                    world.mouse_movement(mouse_position().into());
                }
            }
        });

        world.update();
        world.draw();

        next_frame().await;
    }
}
