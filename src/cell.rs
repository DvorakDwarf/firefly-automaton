use rand::Rng;

use crate::consts::MAX_BRIGHTNESS;

pub trait Cell {
    fn new() -> Self
    where
        Self: Sized;

    fn get_color(&self) -> [u8; 4];

    fn update(&mut self);
}

pub struct Light {
    brightness: u8,
}

impl Cell for Light {
    fn new() -> Light {
        Light {
            brightness: rand::thread_rng().gen_range(0..=15)
        }
    }

    fn update(&mut self) {
        self.brightness += 1;
        if self.brightness > 15 {
            self.brightness = 0;
        }
    }  

    fn get_color(&self) -> [u8; 4] {
        let r_scale = 255 / MAX_BRIGHTNESS;
        [r_scale * self.brightness, 0, 0, 255]
    }
}

pub struct Firefly {
    cooldown: u8
}

impl Cell for Firefly {
    fn new() -> Firefly {
        Firefly{
            cooldown: 5
        }
    }

    fn update(&mut self) {
        
    }

    fn get_color(&self) -> [u8; 4] {
        [0, 0, 0, 255]
    }
}