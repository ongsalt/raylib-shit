use raylib::prelude::*;

use crate::{animation::spring::{SpringAnimator, SpringSpec}, engine::Sprite, input_handler::poll_movement, ui::SceneRunner};

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneRunner {
    let mut selected = 0;
    let mut y = SpringAnimator::new(23.0);
    y.set_spec(SpringSpec::new(1.0, 800.0));

    return Box::new(move |rl, thread| {
        let dt = rl.get_frame_time();
        y.update(dt);
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_W => selected -= 1,
                KeyboardKey::KEY_S => selected += 1,
                _ => {}
            }
            y.animate_to(23.0 + 30.0 * selected as f32);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::YELLOW);

        d.draw_rectangle(
            0,
            y.value() as i32,
            d.get_screen_width(),
            32,
            Color::BLACK.alpha(0.1),
        );

        d.draw_text("text", 30, 30, 18, if selected == 0 {Color::WHITE} else{Color::BLACK});
        d.draw_text("text", 30, 60, 18, if selected == 1 {Color::WHITE} else{Color::BLACK});
        d.draw_text("text", 30, 90, 18, if selected == 2 {Color::WHITE} else{Color::BLACK});
    });
}
