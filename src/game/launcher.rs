use std::{f32::consts::PI, vec};

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
        bullet.rotation += direction.to_degrees(); // idk why the render function use degree but vec2 use radian
        bullet.velocity.rotate(direction);
        bullet.position += position;
        
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
        let texture =
            texture_registry.load_if_not_existed("bullet:1", "bullets/Bullet 001.png", rl, thread);
        let mut bullet_sprite = Sprite::new(vec![texture]);
        bullet_sprite.set_scale(2.0);

        Launcher::new(0.3, false, {
            BulletBuilder::new(
                bullet_sprite,
                Vector2::new(400.0, 0.0),
                Vector2::zero(),
                90.0,
                Rectangle::new(-10.0, 10.0, 20.0, 20.0),
                100.0,
                0.0,
                vec![],
                5.0,
            )
        })
    }
}
