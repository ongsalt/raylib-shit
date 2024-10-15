use crate::{
    core::{texture_registry::TextureRegistry, Map, Tile},
    game::spawning::{SpawnPattern, SpawnTiming},
};
use raylib::prelude::*;

pub fn create_first_map(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    texture_registry: &mut TextureRegistry,
) -> Map {
    let width = 200;
    let height = 200;
    let size = (width * height) as usize;

    let grass =
        texture_registry.load_if_not_existed("tile:grass:001", "tiles/Grass 001.png", rl, thread);
    let stone =
        texture_registry.load_if_not_existed("tile:rock:001", "tiles/Rock 001.png", rl, thread);

    let mut tiles: Vec<Tile> = Vec::with_capacity(size);

    // place stone randomly | TODO: Choose group position first then place in group
    for _ in 0..size {
        let mut tile = Tile::new(vec![grass.clone()]);
        if rl.get_random_value::<i32>(0..100) <= 8 {
            tile.texture.push(stone.clone());
        }
        tiles.push(tile)
    }

    Map::new(tiles, width, height, 64, 1.0)
}

pub fn first_spawn_timing() -> Vec<SpawnTiming> {
    vec![
        SpawnTiming::new(0.0, SpawnPattern::default(0.001, "tee")),
        SpawnTiming::new(6.0, SpawnPattern::default(0.002, "tee")),
        SpawnTiming::new(12.0, SpawnPattern::default(0.003, "tee")),
        SpawnTiming::new(18.0, SpawnPattern::default(0.004, "tee")),
        SpawnTiming::new(24.0, SpawnPattern::default(0.008, "tee")),
    ]
}
