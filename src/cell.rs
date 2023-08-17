use core::fmt::Debug;
use dyn_clone::DynClone;

pub trait Cell: DynClone {
    fn new(x: u32, y: usize) -> Self
    where
        Self: Sized,
        Self: Clone;

    fn get_color(&self) -> [u8; 4];

    //To generalize, make "get_state"
    fn get_brightness(&self) -> f32 {
        return 0.;
    }
    fn set_state(&mut self, incoming_state: f32) {

    }

    //Not because needed, more convenient, see Grid
    fn update(&mut self, grid: &mut Vec<Vec<Box<dyn Cell>>>) -> Vec<Option<(usize, usize, f32)>>;
}

dyn_clone::clone_trait_object!(Cell);

impl Debug for dyn Cell {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "KYS")
    }
}