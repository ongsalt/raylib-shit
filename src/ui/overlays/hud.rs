use raylib::prelude::*;

use crate::{game::player::Player, ui::Overlay};

pub struct HudOverlay {
    hp: f32,
    mp: f32
}

impl HudOverlay {
    pub fn new() -> Self {
        Self {
            hp: 100.0,
            mp: 0.0
        }
    }

    pub fn update(&mut self, player: &Player) {
        self.hp = player.hp();
        self.mp = player.mp();
    }
}

impl Overlay for HudOverlay {
    fn draw(&mut self, d: &mut raylib::prelude::RaylibDrawHandle) {
        let mut hp_box = Rectangle::new(20.0, 20.0, 240.0, 20.0);
        d.draw_rectangle_lines_ex(hp_box, 2.0, Color::WHITE);
        hp_box.height -= 10.0;
        hp_box.width -= 10.0;
        hp_box.x += 5.0;
        hp_box.y += 5.0;
        d.draw_rectangle_rec(hp_box, Color::GREEN);

        d.draw_text("100/100", (hp_box.x + hp_box.width) as i32 + 16, hp_box.y as i32, 12, Color::WHITE);
    }
}