use raylib::prelude::*;

use crate::{animation::spring::{SpringAnimator, SpringSpec}, engine::Sprite, input_handler::poll_movement, ui::{Scene}};

pub struct HomeScene {
    selected: i32,
    y: SpringAnimator
}

impl HomeScene {
    pub fn new() -> Self {
        let mut y = SpringAnimator::new(23.0);
        y.set_spec(SpringSpec::new(1.0, 800.0));    

        Self {
            selected: 0,
            y
        }
    }
}

impl Scene for HomeScene {
    fn setup(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
    }

    fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let dt = rl.get_frame_time();
        self.y.update(dt);
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_W => self.selected -= 1,
                KeyboardKey::KEY_S => self.selected += 1,
                _ => {}
            }
            self.y.animate_to(23.0 + 30.0 * self.selected as f32);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::YELLOW);

        d.draw_rectangle(
            0,
            self.y.value() as i32,
            d.get_screen_width(),
            32,
            Color::BLACK.alpha(0.1),
        );

        d.draw_text("text", 30, 30, 18, if self.selected == 0 {Color::WHITE} else{Color::BLACK});
        d.draw_text("text", 30, 60, 18, if self.selected == 1 {Color::WHITE} else{Color::BLACK});
        d.draw_text("text", 30, 90, 18, if self.selected == 2 {Color::WHITE} else{Color::BLACK});
    }
}