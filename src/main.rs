// #![windows_subsystem = "windows"]

use std::rc::Rc;

use game::scene::GameScene;
use raylib::prelude::*;
use ui::scenes::home::HomeScene;
use ui::Scene;
use ui::animation;
use ui::UIResources;

mod core;
mod data;
mod game;
mod ui;
mod extensions;

fn main() {
    let (mut rl, thread) = raylib::init().size(720, 560).build();
    rl.set_target_fps(60);

    // TODO: wtf why cant i copy 
    let mut scenes: Vec<Box<dyn Scene>> = vec![
        Box::new(HomeScene::new(&mut rl, &thread)),
        Box::new(GameScene::new(&mut rl, &thread)),
    ];

    let mut active_scene = 0;

    let mut target = rl.load_render_texture(&thread, 720, 560).unwrap();

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
