pub mod scenes;
pub mod overlays;
pub mod animation;
use raylib::prelude::*;

// pub type SceneRunner = Box<dyn FnMut(&mut RaylibHandle, &mut RaylibThread) -> ()>;
// pub type Scene = fn(rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneRunner;

pub trait Scene {
    fn setup(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);
    fn reset(&mut self) {}
    fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {}
}

pub trait Overlay {
    fn draw(&mut self, d: &mut RaylibDrawHandle);
}