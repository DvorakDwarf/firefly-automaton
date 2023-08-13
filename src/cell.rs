use rand::Rng;
use core::fmt::Debug;
use dyn_clone::DynClone;

use crate::{consts::{MAX_BRIGHTNESS, OFFSET, WIDTH, HEIGHT}, grid::Grid};

pub trait Cell: DynClone {
    fn new(x: u32, y: usize) -> Self
    where
        Self: Sized,
        Self: Clone;

    fn get_color(&self) -> [u8; 4];

    fn get_brightness(&self) -> u8;

    //Not because needed, more convenient, see Grid
    fn update(&mut self, grid: &Vec<Vec<Box<dyn Cell>>>);
}

dyn_clone::clone_trait_object!(Cell);

impl Debug for dyn Cell {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "KYS")
    }
}

#[derive(Debug, Clone)]
pub struct Light {
    brightness: u8,
    neighbors: Vec<(usize, usize)>,
    
}

impl Cell for Light {
    fn new(x: u32, y: usize) -> Light {
        let x = x as i32;
        let y = y as i32;

        let mut neighbors = Vec::new();

        for i in x-OFFSET..=x+OFFSET {
            for j in y-OFFSET..=y+OFFSET {
                if i < 0 || i >= WIDTH as i32 || j < 0 || j >= HEIGHT as i32 {
                    continue;
                } 

                neighbors.push((i as usize, j as usize));
            }
        }

        Light {
            brightness: 0,
            neighbors
        }
    }

    fn update(&mut self, grid: &Vec<Vec<Box<dyn Cell>>>) {
        // self.brightness += 1;
        // if self.brightness > 15 {
        //     self.brightness = 0;
        // }
    }  

    fn get_color(&self) -> [u8; 4] {
        let r_scale = 255 / MAX_BRIGHTNESS;
        return [r_scale * self.brightness, 0, 0, 255];
    }

    fn get_brightness(&self) -> u8 {
        return self.brightness;
    }
}
#[derive(Debug, Clone)]
pub struct Firefly {
    brightness: u8,
    cooldown: u8,
    neighbors: Vec<(usize, usize)>
}

impl Cell for Firefly {
    fn new(x: u32, y: usize) -> Firefly {
        let x = x as i32;
        let y = y as i32;

        let mut neighbors = Vec::new();

        for i in x-OFFSET..=x+OFFSET {
            for j in y-OFFSET..=y+OFFSET {
                if i < 0 || i >= WIDTH as i32 || j < 0 || j >= HEIGHT as i32 {
                    continue;
                } 

                neighbors.push((i as usize, j as usize));
            }
        }

        Firefly {
            brightness: rand::thread_rng().gen_range(0..=15),
            cooldown: 5,
            neighbors
        }
    }

    fn update(&mut self, grid: &Vec<Vec<Box<dyn Cell>>>) {
        let mut brightness_vec = Vec::new();

        for (x, y) in &self.neighbors {
            brightness_vec.push(grid[*y][*x].get_brightness());
        }

        let brightness: u32 = brightness_vec.iter().map(|x| *x as u32).sum();

        // if brightness < 10 {
        //     self.brightness += 1;
        // }

        self.brightness += 1;
        if self.brightness > 15 {
            self.brightness = 0;
        }
    }

    fn get_color(&self) -> [u8; 4] {
        let r_scale = 255 / MAX_BRIGHTNESS;
        return [r_scale * self.brightness, 0, 0, 255];
    }

    fn get_brightness(&self) -> u8 {
        return self.brightness;
    }
}