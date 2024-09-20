use std::rc::Rc;

use raylib::prelude::*;

// I should make a gif handler
#[derive(Clone, Debug)]
pub struct Sprite {
    textures: Vec<Rc<Texture2D>>, // vec because of fucking gif

    position: Vector2,
    scale: f32,
    rotation: f32,
    offset: Vector2,

    frame_index: usize,
    frame_duration: f32,
    frame_time: f32,
}

impl Sprite {
    pub fn new(textures: Vec<Rc<Texture2D>>) -> Self {
        let offset = if textures.len() != 0 {
            Vector2::new(
                textures[0].width as f32 / 2.0,
                textures[0].height as f32 / 2.0,
            )
        } else {
            Vector2::zero()
        };

        Self {
            offset,
            textures,
            scale: 1.,
            rotation: 0.,
            position: Vector2::zero(),
            frame_duration: 1.0 / 24.0,
            frame_index: 0,
            frame_time: 0.0,
        }
    }

    // pub fn from_resource() -> {

    // }

    pub fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        if self.textures.len() == 0 {
            return;
        }
        d.draw_texture_ex(
            self.textures[self.frame_index].as_ref(),
            self.position - self.offset.rotated(self.rotation.to_radians()) * self.scale,
            self.rotation,
            self.scale,
            Color::WHITE,
        );
    }

    // TODO: handle rotation
    pub fn draw_bound(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_rectangle(
            (self.position.x - self.offset.x * self.scale) as i32,
            (self.position.y - self.offset.y * self.scale) as i32,
            (self.textures[0].width as f32 * self.scale) as i32,
            (self.textures[0].height as f32 * self.scale) as i32,
            Color::RED.alpha(0.2),
        );
    }

    pub fn draw_with_tint(&mut self, tint: Color, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        if self.textures.len() == 0 {
            return;
        }
        d.draw_texture_ex(
            self.textures[self.frame_index].as_ref(),
            self.position - self.offset * self.scale,
            self.rotation,
            self.scale,
            tint,
        );
    }

    pub fn update(&mut self, dt: f32) {
        self.frame_time += dt;

        if self.frame_time > self.frame_duration {
            self.frame_time -= self.frame_duration;
            self.frame_index += 1;

            if self.frame_index >= self.textures.len() {
                self.frame_index = 0;
            }
        }
    }

    pub fn set_position(&mut self, position: Vector2) {
        self.position = position
    }

    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.offset = Vector2::new(
            self.textures[0].width as f32 / 2.0,
            self.textures[0].height as f32 / 2.0,
        );
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_frame_rate(&mut self, frame_rate: u32) {
        self.frame_duration = 1.0 / frame_rate as f32;
    }

    pub fn frame_rate(&self) -> u32 {
        (1.0 / self.frame_duration) as u32
    }
}
