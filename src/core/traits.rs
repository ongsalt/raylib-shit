use raylib::prelude::*;

pub trait Updatable {
    fn update(&mut self, dt: f32);
}

pub trait Drawable {
    fn y_index(&self) -> i32 {
        0
    }
    fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>, camera: &Camera2D);
    fn draw_bound(&self, d: &mut RaylibMode2D<RaylibDrawHandle>, camera: &Camera2D) {}
}
