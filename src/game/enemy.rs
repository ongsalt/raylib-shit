use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D},
    texture::Image,
    RaylibHandle, RaylibThread,
};

use crate::core::{texture_registry::TextureRegistry, Sprite};

use super::bullet::{self, Bullet};

pub struct Enemy {
    sprite: Sprite,
    speed: f32,
    hp: f32,
    position: Vector2, // center
    relative_hitbox: Rectangle, // now is absolute
    should_draw_with_damage_taken: bool
}

impl Enemy {
    pub fn new(
        sprite: Sprite,
        speed: f32,
        hp: f32,
        position: Vector2,
        relative_hitbox: Rectangle,
    ) -> Self {
        Self {
            sprite,
            speed,
            hp,
            position,
            relative_hitbox,
            should_draw_with_damage_taken: false,
        }
    }

    pub fn update(&mut self, dt: f32, player_position: Vector2) {
        self.position += (player_position - self.position).normalized() * self.speed * dt;
    }

    pub fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        self.sprite.set_position(self.position);
        if self.should_draw_with_damage_taken {
            
        }
        self.sprite.draw(d);
    }

    pub fn hitbox(&self) -> Rectangle {
        Rectangle::new(
            self.relative_hitbox.x + self.position.x,
            self.relative_hitbox.y + self.position.y,
            self.relative_hitbox.width,
            self.relative_hitbox.height,
        )
    }

    pub fn draw_hitbox(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_rectangle_rec(self.hitbox(), Color::RED.alpha(0.1));
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.hp -= damage;
    }

    pub fn get_hit(&mut self, bullet: &mut Bullet) {

    }
}

pub struct EnemyFactory {}

impl EnemyFactory {
    pub fn tee(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        texture_registry: &mut TextureRegistry,
    ) -> Enemy {
        let texture = match texture_registry.get("tee") {
            None => {
                let image = Image::load_image("resources/tee.png").unwrap();
                let texture = rl.load_texture_from_image(thread, &image).unwrap();
                texture_registry.add("tee", texture)
            }
            Some(ref texture) => texture.clone(),
        };

        let mut tee_sprite = Sprite::new(vec![texture]);
        tee_sprite.set_scale(0.75);

        Enemy::new(
            tee_sprite,
            100.0,
            100.0,
            Vector2::new(200.0, 200.0),
            Rectangle::new(-40.0, -40.0, 80.0, 80.0),
        )
    }
}
