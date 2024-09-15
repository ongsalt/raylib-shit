use raylib::prelude::*;

pub trait Preloadable {
    fn preload(&mut self);
}

pub trait Updatable {
    fn update(&mut self, dt: f32);
}

// pub trait Drawable {
//     // TODO: cycle gif frame from update and make this not mut
//     fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>, camera: &Camera2D);
// }

