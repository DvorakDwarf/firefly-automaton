pub const WIDTH: u32 = 320;
pub const HEIGHT: u32 = 240;

pub const MAX_BRIGHTNESS: f32 = 64.;

pub const FPS: f32 = 30.;

pub const OFFSET: i32 = 25;

pub const BLINK_COOLDOWN: i16 = FPS as i16 * 2;
pub const ROUGH_MAX_WAIT: i16 = FPS as i16 * 2;
pub const PEER_PRESSURE_THRESHOLD: f32 = 1.;

pub const LIGHT_DECAY: f32 = (MAX_BRIGHTNESS as f32 / 16.) + 0.;