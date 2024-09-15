use std::rc::Rc;

use raylib::prelude::*;


// I should make a gif handler
#[derive(Clone, Debug)]
pub struct Sprite {
    textures: Vec<Rc<Texture2D>>, // vec because of fucking gif
    frame_index: usize,
    animation_speed: i32,
    frame_count: i32,
    position: Vector2,
    scale: f32,
    rotation: f32,
    offset: Vector2,
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
            animation_speed: 1,
            frame_index: 0,
            frame_count: 0,
            scale: 1.,
            rotation: 0.,
            position: Vector2::zero(),
        }
    }

    // pub fn from_resource() -> {

    // }

    pub fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        if self.textures.len() == 0 {
            return;
        }
        self.cycle_frame();
        d.draw_texture_ex(
            self.textures[self.frame_index].as_ref(),
            self.position - self.offset * self.scale,
            self.rotation,
            self.scale,
            Color::WHITE,
        );
    }

    pub fn draw_nth_texture(&mut self, texture_index: usize, d: &mut RaylibMode2D<RaylibDrawHandle>) {

        d.draw_texture_ex(
            self.textures[self.frame_index].as_ref(),
            self.position - self.offset * self.scale,
            self.rotation,
            self.scale,
            Color::WHITE,
        );
    }

    // TODO: move this into update(dt)
    fn cycle_frame(&mut self) {
        self.frame_count += 1;
        if self.frame_count % self.animation_speed == 0 {
            self.frame_index += 1;
            if self.frame_index == self.textures.len() {
                self.frame_count = 0;
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

    pub fn set_animation_speed(&mut self, animation_speed: i32) {
        self.animation_speed = animation_speed
    }

    pub fn animation_speed(&self) -> i32 {
        self.animation_speed
    }
}
