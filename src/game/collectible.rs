use raylib::ffi::Vector2;

use crate::engine::Sprite;

pub struct DroppedCollectible {
    collectible: Collectible,
    position: Vector2,
    sprite: Sprite
}

pub enum Collectible {
    Item(Item),
    Heal,
    Exp
}

pub struct Item {

}