use std::{rc::Rc, vec};

use raylib::prelude::*;

use crate::{
    engine::{registry::TextureRegistry, Sprite},
    utils::{ImageExtension, RaylibHandleExtension},
};

use super::{
    bullet::{self, Bullet},
    collectible::Item, launcher::{Launcher, LauncherFactory},
};

pub const PLAYER_SIZE: f32 = 80.0;
pub const SCREEN_EDGE_PADDING: f32 = 20.0;
pub const CAMERA_PADDING: f32 = PLAYER_SIZE / 2.0 + SCREEN_EDGE_PADDING;
pub struct Player {
    sprite: Sprite,
    position: Vector2,
    items: Vec<Item>,
    launchers: Vec<Launcher>,
    _speed: f32, // in pixel per sec
}

impl Player {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        texture_registry: &mut TextureRegistry,
    ) -> Self {
        let image = Image::load_image_anim("resources/slime.gif").unwrap();

        let textures = rl
            .load_textures_from_image(thread, &image, 24)
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(index, it)| texture_registry.add(format!("player-frame-{}", index), it))
            .collect();

        let mut sprite = Sprite::new(textures);
        sprite.set_scale(0.5);
        sprite.set_animation_speed(4);

        // let bullet_image = Image::load_image("resources/crystal_water.png").unwrap();
        // let bullet_texture = rl.load_texture_from_image(thread, &bullet_image).unwrap();
        // let bullet_textures = vec![texture_registry.add("bullet-1", bullet_texture)];

        Self {
            sprite,
            items: vec![],
            launchers: vec![LauncherFactory::simple(rl, thread, texture_registry)],
            position: Vector2::zero(),
            _speed: 200.0,
        }
    }

    // calculate base on buff | cache
    pub fn speed(&self) -> f32 {
        self._speed
    }

    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn shoot(&mut self, direction: f32) -> Vec<Bullet> {
        self.launchers.iter_mut().flat_map(|it| it.launch(self.position, direction)).collect()
    }

    pub fn movee(&mut self, displacement: Vector2, dt: f32) {
        self.position += displacement * dt * self.speed();
    }

    // TODO: make trait game object | update
    pub fn update(&mut self, dt: f32) {
        for launcher in &mut self.launchers {
            launcher.update(dt)
        }
    }

    // assumming camera offset is correctly set
    pub fn move_camera_if_should(&self, camera: &mut Camera2D) {
        camera.target = self.position;
        // if camera.target.x + camera.offset.x - CAMERA_PADDING < self.position.x {
        //     camera.target.x = self.position.x - camera.offset.x + CAMERA_PADDING;
        // } else if camera.target.x - camera.offset.x + CAMERA_PADDING > self.position.x {
        //     camera.target.x = self.position.x + camera.offset.x - CAMERA_PADDING;
        // }
        // if camera.target.y + camera.offset.y - CAMERA_PADDING < self.position.y {
        //     camera.target.y = self.position.y - camera.offset.y + CAMERA_PADDING;
        // } else if camera.target.y - camera.offset.y + CAMERA_PADDING > self.position.y {
        //     camera.target.y = self.position.y + camera.offset.y - CAMERA_PADDING;
        // }
    }

    pub fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        self.sprite.set_position(self.position);
        self.sprite.draw(d);
    }
}
