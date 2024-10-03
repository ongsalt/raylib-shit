use std::f32::consts::PI;

use raylib::prelude::*;

use crate::{
    core::{
        input_handler::poll_movement, texture_registry::TextureRegistry, Drawable, Map, Updatable,
    },
    data::maps::create_first_map,
    game::{bullet::Bullet, collectible::DroppedCollectible, enemy::Enemy, player::Player},
    ui::{overlays::hud::HudOverlay, Overlay, Scene},
};

use super::{enemy::EnemyFactory, launcher::LauncherFactory, player};

pub struct GameScene {
    map: Map,
    hud: HudOverlay,
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
        let map = create_first_map(rl, thread, &mut texture_registry);

        let launchers = vec![LauncherFactory::simple(rl, thread, &mut texture_registry)];
        let player = Player::new(100.0, 100.0, launchers, rl, thread, &mut texture_registry);

        Self {
            map,
            hud: HudOverlay::new(),
            paused: false,
            camera,
            enemies: vec![EnemyFactory::tee(rl, thread, &mut texture_registry, Vector2::new(100.0, 100.0))],
            // enemies: vec![],
            player,
            texture_registry,
            collectibles: vec![],
            bullets: vec![],
        }
    }
}

impl Updatable for GameScene {
    fn update(&mut self, dt: f32) {
        self.player.update(dt);

        let mut bullets_to_remove = vec![];
        let mut enemies_to_remove = vec![];
        for (bullet_index, bullet) in self.bullets.iter_mut().enumerate() {
            bullet.update(dt);

            if bullet.should_die() {
                bullets_to_remove.push(bullet_index)
            }

            for (enemy_index, enemy) in &mut self.enemies.iter_mut().enumerate() {
                if bullet.is_collided(&enemy) {
                    enemy.take_damage(bullet.damage);
                    bullets_to_remove.push(bullet_index);

                    // TODO: object pool
                    if enemy.is_dead() {
                        enemies_to_remove.push(enemy_index)
                    }
                }
            }
        }

        for enemy in &mut self.enemies {
            enemy.update(dt, self.player.position());
        }

        // Reverse becuase array will shift if we remove from the first
        for index in bullets_to_remove.into_iter().rev() {
            self.bullets.remove(index);
        }

        for index in enemies_to_remove.into_iter().rev() {
            let dead = self.enemies.remove(index);
        }
    }
}

impl GameScene {
    fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        self.map.draw(d, &self.camera);

        let mut drawables: Vec<Box<&dyn Drawable>> = vec![Box::new(&self.player)];

        for bullet in &self.bullets {
            drawables.push(Box::new(bullet));
        }

        for enemy in &mut self.enemies {
            drawables.push(Box::new(enemy));
        }

        drawables.sort_unstable_by_key(|it| it.y_index());

        for drawable in drawables {
            drawable.draw(d, &self.camera);
        }
    }
}

impl Scene for GameScene {
    fn setup(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {}

    fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // ------------ Setup shit --------------
        // TODO: fix scene size and proportion and then scale it later
        if rl.is_window_resized() {
            self.camera.offset.x = (rl.get_screen_width() / 2) as f32;
            self.camera.offset.y = (rl.get_screen_height() / 2) as f32;
        }

        // self.camera.offset is screen center
        let direction = rl.get_mouse_position().angle_to(self.camera.offset) - PI;

        let dt = rl.get_frame_time();

        let displacement = poll_movement(&rl);
        self.player.movee(displacement, dt);

        for bullet in self.player.shoot(direction) {
            self.bullets.push(bullet)
        }

        self.player.move_camera_if_should(&mut self.camera);

        self.update(dt);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d = d.begin_mode2D(&self.camera);
            self.draw(&mut d);
        }

        self.hud.draw(&mut d);
    }
}
