use glam::Vec2;
use raylib::prelude::*;

mod aabb;
mod drawable;
mod geometry;
mod intersection;
mod light;
mod optical_objects;
mod ray;
mod surface;
mod transform;
mod utils;
mod world;

use drawable::Drawable;
use light::LightSource;
use surface::Surface;
use world::World;

use crate::optical_objects::{Material, OpticalObject, PlaneMirror};

const DEPTH: u32 = 2;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("2D Ray Tracing")
        .msaa_4x()
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    rl.set_target_fps(24);

    let mut world = World::new();

    world.add_object(PlaneMirror {
        surface: Surface::plane(Vec2::new(100.0, 100.0), Vec2::new(700.0, 100.0)),
        transform: transform::Transform::identity(),
        material: Material {
            reflectivity: 1.0,
            refractive_index: 1.0,
        },
        one_side: false,
    });

    // Add a point light source
    // world.add_light(LightSource::Point {
    //     origin: Vec2::new(200.0, 150.0),
    //     ray_count: 360,
    //     wavelength: 500.0,
    // });

    world.add_light(LightSource::Point {
        origin: Vec2::new(400.0, 400.0),
        ray_count: 200,
        wavelength: 700.0,
    });

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        world.calculate_ray_paths(DEPTH);
        world.draw(&mut d);
    }
}
