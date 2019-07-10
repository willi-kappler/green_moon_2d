

// Local modules
use crate::vector::{GM_Vec2D};
use crate::velocity::{GM_Velocity};

pub struct GM_Position {
    base: GM_Vec2D,
}

impl GM_Position {
    pub fn new() -> GM_Position {
        GM_Position {
            base: GM_Vec2D::new(),
        }
    }

    pub fn update(&mut self, velocity: &GM_Velocity, time_elapsed: u16) {

    }

}
