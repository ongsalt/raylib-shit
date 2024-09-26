use std::iter::zip;

use rand::random;
use raylib::prelude::*;

use crate::core::texture_registry::TextureRegistry;

use super::enemy::{Enemy, EnemyFactory};

struct SpawnPattern {
    position: Vec<Vector2>,
    enemy_kind: Vec<&'static str>,
}

impl SpawnPattern {
    pub fn apply(
        &self,
        enemies: &mut Vec<Enemy>,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        texture_registry: &mut TextureRegistry,
    ) {
        for (position, kind) in zip(&self.position, &self.enemy_kind) {
            enemies.push(EnemyFactory::create_by_kind(
                kind,
                rl,
                thread,
                texture_registry,
                position.clone(),
            ).unwrap());
        }
    }

    pub fn default(spawn_rate: f32, kind: &'static str, center: Vector2) -> Self {
        let mut position = vec![];

        // randomly place enemy outside the screen 1024 * 768
        // in 4 lines
        for i in -512..512 {
            if random::<f32>() <= spawn_rate {
                position.push(Vector2::new(i as f32 + center.x, center.y - 376.0));
            }

            if random::<f32>() <= spawn_rate {
                position.push(Vector2::new(i as f32 + center.x, center.y + 376.0));
            }
        }

        for i in -376..376 {
            if random::<f32>() <= spawn_rate {
                position.push(Vector2::new(center.x - 512.0, center.y + i as f32));
            }

            if random::<f32>() <= spawn_rate {
                position.push(Vector2::new(center.x + 512.0, center.y + i as f32));
            }
        }

        let enemy_kind = vec![kind; position.len()];

        Self {
            position,
            enemy_kind,
        }
    }
}
