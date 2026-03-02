use enum_dispatch::enum_dispatch;
use raylib::prelude::RaylibDrawHandle;

#[enum_dispatch]
pub trait Drawable {
    fn draw(&self, d: &mut RaylibDrawHandle);
}
