use crate::drawable::Drawable;
use crate::geometry::IntoRaylibVector;
use crate::ray::Ray;
use glam::Vec2;
use raylib::prelude::*;

pub enum LightSource {
    Point {
        origin: Vec2,
        ray_count: u32,
        wavelength: f32,
    },
    Laser {
        origin: Vec2,
        direction: Vec2,
        wavelength: f32,
    },
}

impl Drawable for LightSource {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        match *self {
            LightSource::Point { origin, .. } => {
                d.draw_circle(origin.x as i32, origin.y as i32, 5.0, Color::YELLOW)
            }

            LightSource::Laser {
                origin, direction, ..
            } => {
                let end = origin + direction * 20.0;
                d.draw_line_ex(origin.into_rvec(), end.into_rvec(), 3.0, Color::YELLOW)
            }
        }
    }
}

impl LightSource {
    pub fn get_rays(&self) -> Vec<Ray> {
        match *self {
            LightSource::Point {
                origin,
                ray_count,
                wavelength,
            } => (0..ray_count)
                .map(|i| {
                    let angle = (i as f32 / ray_count as f32) * std::f32::consts::TAU;
                    Ray {
                        origin,
                        direction: Vec2::new(angle.cos(), angle.sin()),
                        wavelength,
                        intensity: 1.0,
                    }
                })
                .collect(),

            LightSource::Laser {
                origin,
                direction,
                wavelength,
            } => vec![Ray {
                origin,
                direction,
                wavelength,
                intensity: 1.0,
            }],
        }
    }
}
