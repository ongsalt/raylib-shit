use std::iter::zip;

use rand::random;
use raylib::prelude::*;

use crate::core::texture_registry::TextureRegistry;

use super::enemy::{Enemy, EnemyFactory};

// TODO: lazily create spawn pattern
pub type Spawner = fn(&mut Vec<Enemy>, &mut RaylibHandle, &RaylibThread, &mut TextureRegistry);

// TODO: There is no need to create a new
pub struct SpawnPattern {
    position: Vec<Vector2>,
    enemy_kind: &'static str,
}

impl SpawnPattern {
    // Need to calculate position here not in the factory
    // fuck it im gonna optimize later
    pub fn apply(
        &self,
        center: Vector2,
        enemies: &mut Vec<Enemy>,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        texture_registry: &mut TextureRegistry,
    ) {
        let Vector2 { x, y } = center;
        for position in &self.position {
            enemies.push(
                EnemyFactory::create_by_kind(
                    self.enemy_kind,
                    rl,
                    thread,
                    texture_registry,
                    Vector2::new(position.x + x, position.y + y),
                )
                .unwrap(),
            );
        }
    }

    pub fn default(spawn_rate: f32, kind: &'static str) -> Self {
        let mut position = vec![];

        // randomly place enemy outside the screen 1024 * 768
        // in 4 lines
        for i in -512..512 {
            if random::<f32>() <= spawn_rate {
                position.push(Vector2::new(i as f32, -376.0));
            }

            if random::<f32>() <= spawn_rate {
                position.push(Vector2::new(i as f32, 376.0));
            }
        }

        for i in -376..376 {
            if random::<f32>() <= spawn_rate {
                position.push(Vector2::new(-512.0, i as f32));
            }

            if random::<f32>() <= spawn_rate {
                position.push(Vector2::new(512.0, i as f32));
            }
        }

        Self {
            position,
            enemy_kind: kind,
        }
    }
}

pub struct SpawnTiming {
    pub time: f32,
    pub spawn_pattern: SpawnPattern,
}

impl SpawnTiming {
    pub fn apply(
        &self,
        center: Vector2,
        enemies: &mut Vec<Enemy>,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        texture_registry: &mut TextureRegistry,
    ) {
        self.spawn_pattern
            .apply(center, enemies, rl, thread, texture_registry);
    }

    pub fn new(time: f32, spawn_pattern: SpawnPattern) -> Self {
        Self {
            time,
            spawn_pattern,
        }
    }
}
