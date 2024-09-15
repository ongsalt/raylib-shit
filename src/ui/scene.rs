use raylib::prelude::*;

// Need to be trait for real

// pub type SceneRunner = Box<dyn FnMut(&mut RaylibHandle, &mut RaylibThread) -> ()>;
// pub type Scene = fn(rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneRunner;

pub trait Scene {
    fn setup(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);
    fn reset(&mut self) {}
    fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {}
    fn pause(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {}
    fn resume(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {}
}