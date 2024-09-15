use raylib::prelude::*;

pub trait Updatable {
    fn update(&mut self, dt: f32);
}

pub trait Drawable {
    fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>);
}

// TODO: cycle gif frame from update
