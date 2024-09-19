use std::vec;

use raylib::prelude::*;

use crate::core::{texture_registry::TextureRegistry, Sprite};

use super::bullet::{Bullet, BulletBuilder};

pub struct Launcher {
    cooldown: f32,
    cooldown_left: f32,
    is_auto: bool,
    bullet_builder: BulletBuilder,
}

impl Launcher {
    pub fn new(cooldown: f32, is_auto: bool, bullet_builder: BulletBuilder) -> Self {
        Self {
            cooldown,
            is_auto,
            cooldown_left: 0.0,
            bullet_builder,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.cooldown_left > 0. {
            self.cooldown_left -= dt;
        }
    }

    pub fn launch(&mut self, position: Vector2, direction: f32) -> Vec<Bullet> {
        if self.cooldown_left > 0. {
            return vec![];
        }
        self.cooldown_left = self.cooldown;

        let mut bullet = self.bullet_builder.build();
        bullet.velocity.rotate(direction);
        bullet.position = position;
        vec![bullet]
    }
}

pub struct LauncherFactory {}
impl LauncherFactory {
    pub fn simple(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        texture_registry: &mut TextureRegistry,
    ) -> Launcher {
        let texture = match texture_registry.get("crystal_water") {
            None => {
                let image = Image::load_image("assets/crystal_water.png").unwrap();
                let texture = rl.load_texture_from_image(thread, &image).unwrap();
                texture_registry.add("crystal_water", texture)
            }
            Some(ref texture) => texture.clone(),
        };

        let mut bullet_sprite = Sprite::new(vec![texture]);
        bullet_sprite.set_scale(0.1);

        Launcher::new(0.3, false, {
            BulletBuilder::new(
                bullet_sprite,
                Vector2::new(300.0, 0.0),
                Vector2::zero(),
                Rectangle::new(-10.0, 10.0, 20.0, 20.0),
                100.0,
                0.0,
                vec![],
                5.0,
            )
        })
    }
}
