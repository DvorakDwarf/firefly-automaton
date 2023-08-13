use rand::Rng;
use core::ops::Index;

use crate::cell::{Cell, Firefly, Light};
use crate::consts::*;

#[derive(Clone)]
pub struct Grid {
    grid: Vec<Vec<Box<dyn Cell>>>
}

impl Grid {
    pub fn new() -> Grid {
        let mut grid: Vec<Vec<Box<dyn Cell>>> = Vec::new();

        for y in 0..HEIGHT {
            grid.push(Vec::new());
        }

        for (y, row) in &mut grid.iter_mut().enumerate() {
            for x in 0..WIDTH {
                let rand_percent = rand::thread_rng().gen_range(0..100);
                if rand_percent >= 90 {
                    row.push(Box::new(Firefly::new(x, y)));
                } else {
                    row.push(Box::new(Light::new(x, y)));
                }
                
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
        let copy_grid = &self.grid.clone();
        for y in 0..HEIGHT as usize{
            for x in 0..WIDTH as usize{
                let cell = &mut self.grid[y][x];
                cell.update(copy_grid);
            }
        }
    }
}

impl Index<usize> for Grid {
    type Output = Vec<Box<dyn Cell>>;

    fn index(&self, y: usize) -> &Self::Output {
        return &self.grid[y];
    }
}