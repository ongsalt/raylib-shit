use std::vec;

use raylib::prelude::*;

use crate::{
    core::{texture_registry::TextureRegistry, Drawable, Sprite},
    extensions::{ImageExtension, RaylibHandleExtension},
};

use super::{
    bullet::Bullet,
    collectible::Item,
    launcher::{Launcher, LauncherFactory},
};

pub const PLAYER_SIZE: f32 = 80.0;
pub const SCREEN_EDGE_PADDING: f32 = 20.0;
pub const CAMERA_PADDING: f32 = PLAYER_SIZE / 2.0 + SCREEN_EDGE_PADDING;
pub struct Player {
    sprite: Sprite,
    position: Vector2,
    items: Vec<&'static Item>,
    launchers: Vec<Launcher>,
    _speed: f32, // in pixel per sec
}

impl Player {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        texture_registry: &mut TextureRegistry,
    ) -> Self {
        let image = Image::load_image_anim("assets/slime.gif").unwrap();

        let textures = rl
            .load_textures_from_image(thread, &image, 24)
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(index, it)| {
                texture_registry.add(format!("player:sprite:{}", index).as_str(), it)
            })
            .collect();

        let mut sprite = Sprite::new(textures);
        sprite.set_scale(0.4);
        sprite.set_frame_rate(24);

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
        self.launchers
            .iter_mut()
            .flat_map(|it| it.launch(self.position, direction))
            .collect()
    }

    pub fn movee(&mut self, displacement: Vector2, dt: f32) {
        self.position += displacement * dt * self.speed();
    }

    pub fn update(&mut self, dt: f32) {
        for launcher in &mut self.launchers {
            launcher.update(dt)
        }
        self.sprite.update(dt);
        self.sprite.set_position(self.position);
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
}

impl Drawable for Player {
    fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>, camera: &Camera2D) {
        self.sprite.draw(d, camera);
        // d.draw_text(
        //     &format!("{:.?}", self.position),
        //     self.position.x as i32,
        //     self.position.y as i32,
        //     12,
        //     Color::WHITE,
        // );
    }
}
