use std::f32::consts::PI;

use raylib::prelude::*;

struct Grid<T> where T: Copy {
    pub grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> where T: Copy {
    pub fn new(width: usize, height: usize) -> Self {
        // TODO: detect integer overflow
        let grid = Vec::with_capacity(width * height);
        Self {
            grid,
            width,
            height,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> T {
        self.grid[x + y * self.width]
    }
    
    // TODO: impl an index trait
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.grid[x + y * self.width] = value;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

// create low res grid first
// then scale and interpolate it

// produce result between 0-1
pub fn create_perlin_noise(scale: u32, width: usize, height: usize) -> Grid<f32> {
    let grid: Grid<f32> = Grid::new(width, height);
    let mut vectors: Grid<Vector2> = Grid::new(width, height);

    for _ in 0..(width * height) {
        vectors.grid.push(Vector2::one().rotated(rand::random::<f32>() * 2.0 * PI))
    }

    grid
}

// linear interpolation
fn interpolate(start: f32, end: f32, progess: f32) -> f32 {
    (start - end) * progess + start
}
