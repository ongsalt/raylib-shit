use raylib::prelude::*;

use crate::core::Drawable;

// There is no sprite here
pub struct DroppedCollectible {
    collectible: Collectible,
    position: Vector2,
}

impl DroppedCollectible {
    pub fn merge(&mut self, other: &DroppedCollectible) -> Option<DroppedCollectible> {
        match (&self.collectible, &other.collectible) {
            (Collectible::Exp(exp1), Collectible::Exp(exp2)) => Some(DroppedCollectible {
                collectible: Collectible::Exp(exp1 + exp2),
                position: (self.position + other.position).scale_by(0.5),
            }),
            _ => None,
        }
    }
}

impl Drawable for DroppedCollectible {
    fn draw(
        &self,
        d: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>,
        camera: &raylib::prelude::Camera2D,
    ) {
        self.collectible.draw(d, camera);
    }
}

pub enum Collectible {
    Item(&'static Item),
    Heal(&'static Heal),
    Exp(u32),
}

impl Drawable for Collectible {
    fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>, camera: &Camera2D) {
        // this thing need to store its texture somehow
        match &self {
            Collectible::Item(_) => {
                
            },
            Collectible::Heal(_) => {

            },
            Collectible::Exp(_) => {
                
            },
        }
    }
}

pub struct Item {
    texture_id: String,
    buff: Vec<Buff>,
    name: String
}

pub struct Heal {
    texture_id: String,
    amount: u32,
}

// HOW tf do i type and apply this thing
// idea 1: global multiplier object
pub struct Buff {}
