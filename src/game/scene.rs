use raylib::prelude::*;

use crate::{
    engine::{registry::TextureRegistry, Sprite},
    game::{
        bullet::Bullet, collectible::DroppedCollectible, enemy::{self, Enemy, EnemyFactory}, player::Player
    },
    input_handler::poll_movement,
    ui::Scene,
};

pub struct GameScene {
    paused: bool,
    camera: Camera2D,
    player: Player,
    enemies: Vec<Enemy>,
    texture_registry: TextureRegistry,
    bullets: Vec<Bullet>,
    // stage: Stage,
    collectibles: Vec<DroppedCollectible>,
}

impl GameScene {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut camera = Camera2D::default();
        // Should be set based on window size
        camera.zoom = 1.;
        camera.offset.x = (rl.get_screen_width() / 2) as f32;
        camera.offset.y = (rl.get_screen_height() / 2) as f32;

        let mut texture_registry = TextureRegistry::new();

        Self {
            paused: false,
            camera,
            enemies: vec![EnemyFactory::tee(rl, thread, &mut texture_registry)],
            player: Player::new(rl, thread, &mut texture_registry),
            texture_registry,
            collectibles: vec![],
            bullets: vec![],
        }
    }
}

impl GameScene {
    fn update(&mut self, dt: f32) {
        self.player.update(dt);

        let mut to_remove_indexes = vec![];
        for (index, bullet) in &mut self.bullets.iter_mut().enumerate() {
            bullet.update(dt);

            if bullet.should_die() {
                to_remove_indexes.push(index)
            }   

            for enemy in &mut self.enemies {
                if bullet.is_collided(&enemy) {
                    enemy.take_damage(bullet.damage);
                    to_remove_indexes.push(index)
                }
            }
        }

        // Reverse becuase array will shift if we remove from the first
        for index in to_remove_indexes.into_iter().rev() {   
            self.bullets.remove(index);
        }

        for enemy in &mut self.enemies {
            enemy.update(dt, self.player.position());
        }
    }
    
    
    fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        // stage
        d.draw_rectangle(-10, -10, 1000, 1000, Color::GRAY);

        self.player.draw(d);

        for bullet in &mut self.bullets {
            bullet.draw(d);
        }

        for enemy in &mut self.enemies {
            enemy.draw(d);
            enemy.draw_hitbox(d);
        }
    }
}

impl Scene for GameScene {
    fn setup(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {}

    fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // ------------ Setup shit --------------
        if rl.is_window_resized() {
            self.camera.offset.x = (rl.get_screen_width() / 2) as f32;
            self.camera.offset.y = (rl.get_screen_height() / 2) as f32;
        }

        // self.camera.offset is screen center
        let direction = PI as f32 + rl.get_mouse_position().angle_to(self.camera.offset);

        let dt = rl.get_frame_time();

        let displacement = poll_movement(&rl);
        self.player.movee(displacement, dt);
        
        for bullet in self.player.shoot(direction) {
            self.bullets.push(bullet)
        }

        println!("{:.?}", self.bullets.len());

        self.player.move_camera_if_should(&mut self.camera);

        self.update(dt);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let mut d = d.begin_mode2D(self.camera);
        self.draw(&mut d);        

    }

    fn pause(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        todo!()
    }

    fn resume(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        todo!()
    }
}
