use crate::core::{Drawable, Sprite};
use raylib::prelude::*;

use super::{
    effect::{StatusEffect},
    enemy::Enemy,
};

// Or should I made a bullet type lookup map
// then just pass bullet reference around
pub struct BulletBuilder {
    pub sprite: Sprite,
    pub velocity: Vector2,
    pub position: Vector2,
    rotation: f32,
    relative_hitbox: Rectangle,
    pub damage: f32,
    pub angular_velocity: f32,
    pub effects: Vec<StatusEffect>,
    pub lifetime: f32,
}

impl BulletBuilder {
    pub fn new(
        sprite: Sprite,
        velocity: Vector2,
        position: Vector2,
        rotation: f32, // In degrees
        relative_hitbox: Rectangle,
        damage: f32,
        angular_velocity: f32,
        effects: Vec<StatusEffect>,
        lifetime: f32,
    ) -> Self {
        Self {
            sprite,
            velocity,
            position,
            rotation,
            relative_hitbox,
            damage,
            angular_velocity,
            effects,
            lifetime,
        }
    }

    // Accept buff
    pub fn build(&self) -> Bullet {
        Bullet::new(
            self.sprite.clone(),
            self.velocity,
            self.position,
            self.rotation,
            self.relative_hitbox,
            self.damage,
            self.angular_velocity,
            self.effects.clone(),
            self.lifetime
        )
    }
}

#[derive(Debug)]
pub struct Bullet {
    sprite: Sprite,
    pub velocity: Vector2,
    pub position: Vector2,
    pub rotation: f32,
    relative_hitbox: Rectangle,
    pub damage: f32,
    angular_velocity: f32,
    pub effects: Vec<StatusEffect>,
    lifetime: f32,
}

impl Bullet {
    pub fn new(
        sprite: Sprite,
        velocity: Vector2,
        position: Vector2,
        rotation: f32,
        relative_hitbox: Rectangle,
        damage: f32,
        angular_velocity: f32,
        effects: Vec<StatusEffect>,
        lifetime: f32,
    ) -> Self {
        Self {
            sprite,
            velocity,
            position,
            rotation,
            relative_hitbox,
            damage,
            angular_velocity,
            effects,
            lifetime,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.lifetime -= dt;

        self.sprite.set_position(self.position);
        self.sprite.set_rotation(self.rotation);
    }

    pub fn hitbox(&self) -> Rectangle {
        Rectangle::new(
            self.relative_hitbox.x + self.position.x,
            self.relative_hitbox.y + self.position.y,
            self.relative_hitbox.width,
            self.relative_hitbox.height,
        )
    }

    pub fn is_collided(&self, enemy: &Enemy) -> bool {
        enemy.hitbox().check_collision_recs(&self.hitbox())
    }

    pub fn should_die(&self) -> bool {
        self.lifetime <= 0.0
    }
}

impl Drawable for Bullet {
    fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>, camera: &Camera2D) {
        self.sprite.draw(d, camera);
    }
}

