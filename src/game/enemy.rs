use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D},
    RaylibHandle, RaylibThread,
};

use crate::core::{texture_registry::TextureRegistry, Sprite};

use super::bullet::{self, Bullet};

pub enum EnemyState {
    Ready, // ready to be use
    Alive,
    Dying, // playing dead animation
    Died, // can be reuse later
}

impl EnemyState {
    fn should_render(&self) -> bool {
        match self {
            EnemyState::Ready => false,
            EnemyState::Alive => true,
            EnemyState::Dying => true,
            EnemyState::Died => false,
        }
    }
}

// Should create an object pool and make is_dead flag
pub struct Enemy {
    pub kind: &'static str,
    sprite: Sprite,
    speed: f32,
    hp: f32,
    position: Vector2,          // center
    relative_hitbox: Rectangle, // now is absolute
    damaged_time: f32,
    // drop: ,
    //  TODO: enemy-drop map
    state: EnemyState
}

impl Enemy {
    pub fn new(
        kind: &'static str,
        sprite: Sprite,
        speed: f32,
        hp: f32,
        position: Vector2,
        relative_hitbox: Rectangle,
    ) -> Self {
        Self {
            kind,
            sprite,
            speed,
            hp,
            position,
            relative_hitbox,
            damaged_time: 0.0,
            state: EnemyState::Alive, // Fix later
        }
    }

    pub fn update(&mut self, dt: f32, player_position: Vector2) {
        self.position += (player_position - self.position).normalized() * self.speed * dt;
        self.damaged_time -= dt;
    }

    pub fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        self.sprite.set_position(self.position);
        self.sprite.draw(d);
        if self.damaged_time > 0.0 {
            self.sprite
                .draw_with_tint(Color::new(252, 146, 139, 255), d);
        }
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
        self.damaged_time = 0.1;
    }

    pub fn get_hit(&mut self, bullet: &mut Bullet) {}

    pub fn is_dead(&self) -> bool {
        !self.state.should_render()
    }
}

pub struct EnemyFactory {}

impl EnemyFactory {
    pub fn tee(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        texture_registry: &mut TextureRegistry,
        position: Vector2,
    ) -> Enemy {
        let texture = texture_registry.load_if_not_existed("tee", "tee.png", rl, thread);

        let mut tee_sprite = Sprite::new(vec![texture]);
        tee_sprite.set_scale(0.75);

        Enemy::new(
            "tee",
            tee_sprite,
            100.0,
            100.0,
            position,
            Rectangle::new(-40.0, -40.0, 80.0, 80.0),
        )
    }

}
