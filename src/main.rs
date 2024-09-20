// #![windows_subsystem = "windows"]

use game::scene::GameScene;
use raylib::prelude::*;
use scenes::home::HomeScene;
use ui::Scene;

mod animation;
mod core;
mod data;
mod game;
mod scenes;
mod ui;
mod extensions;

fn main() {
    let (mut rl, mut thread) = raylib::init().size(720, 560).resizable().build();
    rl.set_target_fps(60);

    let mut scenes: Vec<Box<dyn Scene>> = vec![
        Box::new(HomeScene::new()),
        Box::new(GameScene::new(&mut rl, &thread)),
    ];

    let mut active_scene = 0;

    // let mut overlay: Option<SceneRunner> = None;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_X) {
            if active_scene == 0 {
                active_scene = 1;
            } else {
                active_scene = 0;
            }
            scenes[active_scene].setup(&mut rl, &thread);
        }
        scenes[active_scene].run(&mut rl, &thread)
    }
}
