use raylib::prelude::*;

use crate::{engine::Sprite, input_handler::poll_movement, ui::SceneRunner};

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneRunner {
    let mut camera = Camera2D::default();
    camera.zoom = 1.;

    let mut slime = Sprite::new(rl, &thread, "resources/slime.gif", true);
    slime.scale = 0.5;

    return Box::new(move |rl, thread| {
        let movement = poll_movement(&rl).scale_by(3.);
        slime.position += movement;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let mut d = d.begin_mode2D(camera);
        d.draw_rectangle(-10, -10, 1000, 1000, Color::GRAY);
        slime.draw(&mut d);
    });
}