
use rand::Rng;

use crate::consts::*;
use crate::cell::Cell;

fn pythagoras(a: i32, b: i32) -> f32 {
    let a = (a as f32).powf(2.);
    let b = (b as f32).powf(2.);
    let c = (a + b).sqrt();

    return c;
}

//Thanks Acerola
fn ease_out_expo(x: f32) -> f32 {
    if x == 1. {
        return 1.;
    }

    return -1.*(2.0f32.powf(7.*(x-1.))) + 1.; 
}

fn ease_out_quart(x: f32) -> f32 {
    return 1. - (x.powf(4.));
}

fn ease_out_quad(x: f32) -> f32 {
    let x = x + 1.;

    return 1. - (1. - x) * (1. - x);
}

#[derive(Debug, Clone)]
pub struct Firefly {
    cooldown: i16,
    neighbors: Vec<(usize, usize, f32)>
}

impl Cell for Firefly {
    fn new(x: u32, y: usize) -> Firefly {
        let x = x as i32;
        let y = y as i32;

        let mut neighbors = Vec::new();

        for i in x-OFFSET..=x+OFFSET {
            for j in y-OFFSET..=y+OFFSET {
                if i < 0 || i >= WIDTH as i32 || 
                j < 0 || j >= HEIGHT as i32 {
                    continue;
                } 

                let x_distance = (x - i).abs();
                let y_distance = (y - j).abs();
                let c = pythagoras(x_distance, y_distance);
                // println!("x: {}, y: {}, pyth: {}", x_distance, y_distance, c);
                if c.round() > OFFSET as f32 {
                    continue;
                }

                neighbors.push((i as usize, j as usize, c));
            }
        }

        Firefly {
            cooldown: rand::thread_rng().gen_range(0..=BLINK_COOLDOWN),
            neighbors
        }
    }

    fn update(&mut self, grid: &mut Vec<Vec<Box<dyn Cell>>>) -> Vec<Option<(usize, usize, f32)>> {
        let mut brightness_vec = Vec::new();

        for (x, y, c) in &self.neighbors {
            brightness_vec.push(grid[*y][*x].get_brightness());
        }

        let nearby_brightness: f32 = brightness_vec.iter().map(|x| *x as f32).sum();

        if self.cooldown <= 0 {
            if nearby_brightness >= PEER_PRESSURE_THRESHOLD {
                // dbg!("PEER PRESSURE");
                self.cooldown = BLINK_COOLDOWN;
                return self.flash(grid);
            }

            let rand_percent = rand::thread_rng().gen_range(0..=ROUGH_MAX_WAIT);
            if rand_percent == ROUGH_MAX_WAIT {
                self.cooldown = BLINK_COOLDOWN;
                return self.flash(grid);
            }
        } else {
            self.cooldown_step();
        }

        return vec!(None);
    }

    fn get_color(&self) -> [u8; 4] {
        return [255, 0, 0, 255];
    }
}

impl Firefly {
    fn cooldown_step(&mut self) {
        self.cooldown -= 1;
        if self.cooldown < 0 {
            self.cooldown = BLINK_COOLDOWN;
        }
    }

    #[allow(unused_mut)] 
    fn flash(&self, grid: &mut Vec<Vec<Box<dyn Cell>>>) -> Vec<Option<(usize, usize, f32)>> {
        let mut target_vec = Vec::new();
        // let max_c = OFFSET as f32 / MAX_BRIGHTNESS ;

        for (x, y, c) in &self.neighbors {
            if *c == 0. {
                continue;
            }  
            //We use an easing function to make it more bright near center
            let c_ratio = 1. - (c / (OFFSET as f32 + 0.5));
            let mut target_brightness = MAX_BRIGHTNESS * c_ratio;
            // let easing_coeff = ease_out_quad(c_ratio);
            // target_brightness *= easing_coeff;

            target_vec.push(Some((*x, *y, target_brightness)));
        }
        return target_vec;
    }
}