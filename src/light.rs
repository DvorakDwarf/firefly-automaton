use crate::consts::*;
use crate::cell::Cell;

#[derive(Debug, Clone)]
pub struct Light {
    brightness: f32,    
}

impl Cell for Light {
    fn new(x: u32, y: usize) -> Light {
        Light {
            brightness: 0.,
        }
    }

    fn update(&mut self, grid: &mut Vec<Vec<Box<dyn Cell>>>) -> Vec<Option<(usize, usize, f32)>> {
        self.brightness_step();

        return vec!(None);
    }  

    fn get_color(&self) -> [u8; 4] {
        let r_scale = 255. / MAX_BRIGHTNESS;
        let r = (r_scale * self.brightness).round() as u8;
        // dbg!(r);
        return [r, 0, 0, 255];
    }

    fn get_brightness(&self) -> f32 {
        return self.brightness;
    }

    fn set_state(&mut self, incoming_state: f32) {
        self.brightness += incoming_state;
        if self.brightness > MAX_BRIGHTNESS {
            self.brightness = MAX_BRIGHTNESS;
        }
    }
}

impl Light {
    //Auto dim
    fn brightness_step(&mut self) {
        if self.brightness > 0. {
            self.brightness -= LIGHT_DECAY;
        }
        if self.brightness < 0. {
            self.brightness = 0.;
        }
    }
}
