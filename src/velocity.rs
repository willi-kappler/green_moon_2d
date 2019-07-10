


// Local modules
use crate::vector::{GM_Vec2D};
use crate::acceleration::{GM_Acceleration};

pub struct GM_Velocity {
    base: GM_Vec2D,
}

impl GM_Velocity {
    pub fn new() -> GM_Velocity {
        GM_Velocity {
            base: GM_Vec2D::new(),
        }
    }

    pub fn update(&mut self, acceleration: &GM_Acceleration, time_elapsed: u16) {

    }
}