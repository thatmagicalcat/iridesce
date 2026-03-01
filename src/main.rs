use glam::Vec2;
use raylib::prelude::*;

mod drawable;
mod geometry;
mod intersection;
mod light;
mod ray;
mod surface;
mod utils;
mod world;

use drawable::Drawable;
use light::LightSource;
use surface::SurfaceShape;
use world::World;

const DEPTH: u32 = 3;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("2D Ray Tracing")
        .msaa_4x()
        .build();

    rl.set_target_fps(24);

    let mut world = World::new();

    // Add a rectangular boundary around the window
    world.add_surface(SurfaceShape::plane(
        Vec2::new(100.0,100.0),
        Vec2::new(700.0, 100.0),
        0.3,
    )); // Top
    world.add_surface(SurfaceShape::plane(
        Vec2::new(100.0, 100.0),
        Vec2::new(100.0, 700.0),
        0.3,
    )); // Left
    world.add_surface(SurfaceShape::plane(
        Vec2::new(100.0, 700.0),
        Vec2::new(700.0, 700.0),
        0.3,
    )); // Bottom
    world.add_surface(SurfaceShape::plane(
        Vec2::new(700.0, 100.0),
        Vec2::new(700.0, 700.0),
        0.3,
    )); // Right

    // Add a circle in the middle
    world.add_surface(SurfaceShape::circle(Vec2::new(400.0, 300.0), 50.0, 0.5));

    // Add a point light source
    world.add_light(LightSource::Point {
        origin: Vec2::new(200.0, 150.0),
        ray_count: 360,
        wavelength: 500.0,
    });

    // world.add_light(LightSource::Laser {
    //     origin: Vec2::new(600.0, 150.0),
    //     direction: Vec2::new(-1.0, 1.0).normalize(),
    //     wavelength: 700.0,
    // });

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        world.calculate_ray_paths(DEPTH);
        world.draw(&mut d);
    }
}
