use raylib::prelude::*;

// Need to be trait for real

pub type SceneRunner = Box<dyn FnMut(&mut RaylibHandle, &mut RaylibThread) -> ()>;
pub type Scene = fn(rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneRunner;

pub trait Scene_ {
    fn setup(rl: &mut RaylibHandle, thread: &RaylibThread);
    fn run(rl: &mut RaylibHandle, thread: &RaylibThread);
    fn pause(rl: &mut RaylibHandle, thread: &RaylibThread);
    fn resume(rl: &mut RaylibHandle, thread: &RaylibThread);
}