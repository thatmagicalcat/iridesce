use raylib::prelude::RaylibDrawHandle;

pub trait Drawable {
    fn draw(&self, d: &mut RaylibDrawHandle);
}
