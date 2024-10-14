use raylib::prelude::*;

use crate::ui::animation::spring::SpringAnimator;

pub trait Popup {
    fn draw(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread);
    fn show(&mut self) -> Result<(), ()>;
    fn hide(&mut self) -> Result<(), ()>;
    fn should_show(&self) -> bool;
    fn toggle(&mut self) -> Result<(), ()> {
        if self.should_show() {
            self.hide()
        } else {
            self.show()
        }
    }
}

pub struct TestPopup {
    should_show: bool,
    font: Font,
    font_header: Font,
    render_texture: RenderTexture2D,
    scale: SpringAnimator,
    opacity: SpringAnimator,
}

impl TestPopup {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            should_show: false,
            font: rl
                .load_font(thread, "assets/fonts/Inter-Regular.ttf")
                .unwrap(),
            font_header: rl
                .load_font(thread, "assets/fonts/Inter-Medium.ttf")
                .unwrap(),
            scale: SpringAnimator::new(1.0),
            opacity: SpringAnimator::new(1.0),
            render_texture: rl.load_render_texture(&thread, 720, 560).unwrap(),
        }
    }

    fn update(&mut self, dt: f32) {
        if self.should_show {
            self.scale.animate_to(1.0);
            self.opacity.animate_to(1.0);
        } else {
            self.scale.animate_to(0.9);
            self.opacity.animate_to(0.0);
        }
        self.scale.update(dt);
        self.opacity.update(dt);
    }
}

impl Popup for TestPopup {
    fn draw(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        let dt = d.get_frame_time();
        self.update(dt);

        let screen_w = d.get_screen_width() as f32;
        let screen_h = d.get_screen_height() as f32;
        let w = 400.0;
        let h = 400.0;
        let boundary = Rectangle::new((screen_w - w) / 2.0, (screen_h - h) / 2.0, w, h);

        {
            let mut d = d.begin_texture_mode(thread, &mut self.render_texture);
            d.clear_background(Color::WHITE.alpha(0.0));
            d.draw_rectangle_rounded(boundary, 0.1, 1, Color::WHITE);
        }

        let texture_w = self.render_texture.texture.width as f32;
        let texture_h = self.render_texture.texture.height as f32;
        let scale = self.scale.value();
        let source_rec = Rectangle::new(0.0, 0.0, texture_w, -texture_h);
        let dest_rec = Rectangle::new(
            screen_w * (1.0 - scale) / 2.0,
            screen_h * (1.0 - scale) / 2.0,
            texture_w * scale,
            texture_h * scale,
        );

        d.draw_texture_pro(
            &self.render_texture,
            source_rec,
            dest_rec,
            Vector2::zero(),
            0.0,
            Color::WHITE.alpha(self.opacity.value()),
        );
    }

    fn show(&mut self) -> Result<(), ()> {
        self.should_show = true;
        Ok(())
    }

    fn hide(&mut self) -> Result<(), ()> {
        self.should_show = false;
        Ok(())
    }

    fn should_show(&self) -> bool {
        self.should_show
    }
}
