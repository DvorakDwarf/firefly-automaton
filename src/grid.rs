use crate::cell::{Cell, Firefly, Light};
use crate::consts::*;

pub struct Grid {
    grid: Vec<Vec<Box<dyn Cell>>>
}

impl Grid {
    pub fn new() -> Grid {
        let mut grid: Vec<Vec<Box<dyn Cell>>> = Vec::new();

        for y in 0..HEIGHT {
            grid.push(Vec::new());
        }

        for row in &mut grid {
            for x in 0..WIDTH {
                row.push(Box::new(Light::new()));
            }
        }

        Grid {
            grid
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // dbg!(i, pixel);

            let x = (i % WIDTH as usize) as usize;
            let y = (i / WIDTH as usize) as usize;

            let rgba = self.grid[y][x].get_color();

            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn update(&mut self) {
        for y in 0..HEIGHT as usize{
            for x in 0..WIDTH as usize{
                self.grid[y][x].update();
            }
        }
    }
}