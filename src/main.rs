use std::ops::Mul;

use engine::{
    interpolator::{Interpolator},
    Sprite,
};
use input_handler::poll_movement;
use raylib::prelude::*;
use ui::{Scene, SceneRunner};

mod engine;
mod input_handler;
mod scene;
mod ui;
mod utils;
mod animation;

fn main() {
    println!("Hello, world!");
    let (mut rl, mut thread) = raylib::init().size(640, 480).resizable().build();
    rl.set_target_fps(60);

    let scenes: Vec<Scene> = vec![
        scene::sample::run,
        scene::home::run,
    ];

    let mut active_scene = scenes[0];
    let mut runner: SceneRunner = active_scene(&mut rl, &thread);
    // let mut overlay: Option<SceneRunner> = None;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_X) {
            if active_scene == scenes[0] {
                active_scene = scenes[1]
            } else {
                active_scene = scenes[0]
            }
            runner = active_scene(&mut rl, &thread)
            // if overlay.is_none() {
            //     overlay = Some(scenes[1](&mut rl, &thread))
            // } else {
            //     overlay = None
            // }
        }

        runner(&mut rl, &mut thread);
        // if let Some(ref mut overlay) = overlay {
        //     overlay(&mut rl, &mut thread);
        // }
    }
}
