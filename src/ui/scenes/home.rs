use std::{f32::consts::PI, rc::Rc};

use raylib::prelude::*;

use crate::{
    animation::spring::{SpringAnimator, SpringSpec},
    ui::{
        overlays::popup::{Popup, TestPopup},
        Scene, UIResources,
    },
};

pub struct HomeScene {
    selected: i32,
    popup: TestPopup,
    render_texture: RenderTexture2D,
    y: SpringAnimator,
    blur_radius: SpringAnimator,
    box_blur_shader: Shader,
    blur_radius_location: i32
}

impl HomeScene {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut y = SpringAnimator::new(23.0);
        y.set_spec(SpringSpec::new(1.0, 800.0));
        let mut box_blur_shader = rl
                .load_shader(thread, None, Some("assets/shaders/box_blur.fs"))
                .unwrap();

        let render_width_location= box_blur_shader.get_shader_location("renderWidth");
        let render_height_location= box_blur_shader.get_shader_location("renderHeight");

        // TODO: add onScreenResize event to traits
        box_blur_shader.set_shader_value(render_width_location, 720.0);
        box_blur_shader.set_shader_value(render_height_location, 540.0);

        Self {
            selected: 0,
            popup: TestPopup::new(rl, thread),
            y,
            render_texture: rl.load_render_texture(&thread, 720, 560).unwrap(),
            blur_radius_location: box_blur_shader.get_shader_location("blurRadius"),
            box_blur_shader,
            blur_radius: SpringAnimator::new(0.0),
        }
    }
}

impl Scene for HomeScene {
    fn setup(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {}

    fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let dt = rl.get_frame_time();
        self.y.update(dt);
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_W => self.selected -= 1,
                KeyboardKey::KEY_S => self.selected += 1,
                KeyboardKey::KEY_E => {
                    self.popup.toggle().unwrap();
                }
                _ => {}
            }
            self.y.animate_to(23.0 + 30.0 * self.selected as f32);
        }

        let mut d = rl.begin_drawing(thread);
        {
            let mut d = d.begin_texture_mode(thread, &mut self.render_texture);
            d.clear_background(Color::YELLOW);

            d.draw_rectangle(
                0,
                self.y.value() as i32,
                d.get_screen_width(),
                32,
                Color::BLACK.alpha(0.1),
            );

            d.draw_text("text", 30, 30, 18, Color::BLACK);
            d.draw_text("text", 30, 60, 18, Color::BLACK);
            d.draw_text("text", 30, 90, 18, Color::BLACK);
        }

        // d.draw_texture(&self.render_texture, 0, 0, Color::WHITE);
        d.draw_texture_ex(
            &self.render_texture,
            Vector2::zero(),
            0.0,
            1.0,
            Color::WHITE,
        );

        let source_rec = Rectangle::new(
            0.0,
            0.0,
            self.render_texture.texture.width as f32,
            -self.render_texture.texture.height as f32,
        );
        let dest_rec = Rectangle::new(
            0.0,
            0.0,
            self.render_texture.texture.width as f32,
            self.render_texture.texture.height as f32,
        );

        {
            self.box_blur_shader.set_shader_value(self.blur_radius_location, self.blur_radius.value());
            let mut d = d.begin_shader_mode(&self.box_blur_shader);
            d.draw_texture_pro(
                &self.render_texture,
                source_rec,
                dest_rec,
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }

        self.popup.draw(&mut d, &thread);
    }

    fn render_texture(&self) -> &RenderTexture2D {
        &self.render_texture
    }
}
