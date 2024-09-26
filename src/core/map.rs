use std::rc::Rc;

use raylib::prelude::*;

use crate::extensions::is_visible;

use super::Drawable;

#[derive(Clone, Debug)]
pub struct Tile {
    // i think we should compute this statically
    pub texture: Vec<Rc<Texture2D>>,
}

impl Tile {
    pub fn new(texture: Vec<Rc<Texture2D>>) -> Self {
        Self { texture }
    }

    pub fn is_visible(&self, grid_position: Vector2, camera: &Camera2D, tile_size: u32) -> bool {
        // at least one four corner must be visible
        let size = tile_size as f32;
        is_visible(&(grid_position * size), camera) || // top left
        is_visible(&(grid_position * size + Vector2::new(0.0, size)), camera) ||
        is_visible(&(grid_position * size + Vector2::new(size, 0.0)), camera) ||
        is_visible(&(grid_position * size + Vector2::new(size, size)), camera)
    }
}

pub struct Map {
    tiles: Vec<Tile>,
    width: u32, // should be half usize so tiles.len() would be usize
    height: u32,
    tile_size: u32,
    tile_scale: f32,
}

impl Map {
    pub fn new(tiles: Vec<Tile>, width: u32, height: u32, tile_size: u32, tile_scale: f32) -> Self {
        Self {
            tiles,
            width,
            height,
            tile_size,
            tile_scale,
        }
    }
}

impl Drawable for Map {
    // Should build this once and not every frame
    fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>, camera: &Camera2D) {
        for x in 0..self.width {
            for y in 0..self.height {
                let tile = &self.tiles[(x + y * self.height) as usize];
                let grid_position = Vector2::new(x as f32, y as f32);

                if tile.is_visible(grid_position, camera, self.tile_size) {
                    // from lowest to highest
                    for layer in &tile.texture {
                        d.draw_texture_ex(
                            layer.as_ref(),
                            grid_position * self.tile_size as f32,
                            0.0,
                            self.tile_scale,
                            Color::WHITE,
                        );
                    }
                }
            }
        }
    }
}
