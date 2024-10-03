use raylib::ffi::Vector2;

use crate::core::Drawable;

// There is no sprite here
pub struct DroppedCollectible {
    collectible: Collectible,
    position: Vector2,
}

impl Drawable for DroppedCollectible {
    fn draw(&self, d: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>, camera: &raylib::prelude::Camera2D) {
        todo!()
    }
}

pub enum Collectible {
    Item(&'static Item),
    Heal(&'static Heal),
    Exp(u32)
}

pub struct Item {
    buff: [Buff]
}

pub struct Heal {
    texture_id: String,
    amount: u32
}

// HOW tf do i type and apply this thing
// idea 1: global multiplier object
pub struct Buff {

}