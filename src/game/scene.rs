use std::f32::consts::PI;

use raylib::prelude::*;

use crate::{
    core::{
        input_handler::poll_movement, texture_registry::TextureRegistry, Drawable, Map, Updatable,
    },
    data::maps::{create_first_map, first_spawn_timing},
    game::{bullet::Bullet, collectible::DroppedCollectible, enemy::Enemy, player::Player},
    ui::{overlays::hud::HudOverlay, Overlay, Scene},
};

use super::{enemy::EnemyFactory, launcher::LauncherFactory, spawning::SpawnTiming};

pub struct GameScene {
    render_texture: RenderTexture2D,
    map: Map,
    hud: HudOverlay,
    spawn_timer: SpawnTimer,
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
            spawn_timer: SpawnTimer::new(first_spawn_timing()),
            camera,
            enemies: vec![EnemyFactory::tee(
                rl,
                thread,
                &mut texture_registry,
                Vector2::new(100.0, 100.0),
            )],
            // enemies: vec![],
            player,
            texture_registry,
            collectibles: vec![],
            bullets: vec![],
            render_texture: rl.load_render_texture(&thread, 720, 560).unwrap(),
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

    fn update(&mut self, dt: f32, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.player.update(dt);
        self.spawn_timer.update(
            dt,
            self.player.position(),
            &mut self.enemies,
            rl,
            thread,
            &mut self.texture_registry,
        );

        let mut bullets_to_remove = vec![];
        let mut enemies_to_remove = vec![];
        for (bullet_index, bullet) in self.bullets.iter_mut().enumerate() {
            bullet.update(dt);

            let mut will_be_removed = false;
            if bullet.should_die() {
                // die due to timeout
                bullets_to_remove.push(bullet_index);
                will_be_removed = true;
            }

            for (enemy_index, enemy) in &mut self.enemies.iter_mut().enumerate() {
                if bullet.is_collided(&enemy) {
                    if bullet.still_piercable() {
                        // TODO: make an internal cooldown for bullet and enemy pair
                        enemy.take_damage(bullet.damage);
                        bullet.hit();

                        if bullet.should_die() && !will_be_removed {
                            bullets_to_remove.push(bullet_index);
                            will_be_removed = true;
                        }
                    }

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

        self.update(dt, rl, thread);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d = d.begin_mode2D(&self.camera);
            self.draw(&mut d);
        }

        self.hud.draw(&mut d);
    }

    fn render_texture(&self) -> &RenderTexture2D {
        &self.render_texture
    }
}

// TODO: make this thing more complex
struct SpawnTimer {
    pub time: f32,
    pub timing: Vec<SpawnTiming>,
    next_spawn: f32,
    next_pattern: usize,
}

impl SpawnTimer {
    pub fn new(timing: Vec<SpawnTiming>) -> Self {
        Self {
            time: 0.0,
            next_spawn: if timing.is_empty() {
                0.0
            } else {
                timing[0].time
            },
            timing,
            next_pattern: 0,
        }
    }

    fn update(
        &mut self,
        dt: f32,
        center: Vector2,
        enemies: &mut Vec<Enemy>,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        texture_registry: &mut TextureRegistry,
    ) {
        self.time += dt;
        if self.time >= self.next_spawn {
            // spawn it
            self.timing[self.next_pattern].apply(center, enemies, rl, thread, texture_registry);
            if self.next_pattern + 1 < self.timing.len() {
                self.next_pattern += 1;
                self.next_spawn = self.timing[self.next_pattern].time;
            } else {
                self.next_spawn = f32::INFINITY;
            }
        }
    }
}
