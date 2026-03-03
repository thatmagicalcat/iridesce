use glam::Vec2;
use raylib::prelude::*;

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

use drawable::Drawable;
use surface::Surface;
use world::World;

use crate::{
    lights::PointLight,
    optical_objects::{Material, PlaneMirror},
};

const DEPTH: u32 = 2;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        // .msaa_4x()
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    rl.set_target_fps(24);

    let mut world = World::new();

    world.add_object(PlaneMirror::new(
        Surface::plane(Vec2::new(100.0, 100.0), Vec2::new(700.0, 100.0)),
        transform::Transform::identity(),
        Material {
            reflectivity: 1.0,
            refractive_index: 1.0,
        },
        false,
    ));

    // Add a point light source
    // world.add_light(LightSource::Point {
    //     origin: Vec2::new(200.0, 150.0),
    //     ray_count: 360,
    //     wavelength: 500.0,
    // });

    world.add_light(PointLight::new(Vec2::new(400.0, 400.0), 700.0, 200));

    while !rl.window_should_close() {
        world.handle_event(&rl);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        world.calculate_ray_paths(DEPTH);
        world.draw(&mut d);
    }
}
