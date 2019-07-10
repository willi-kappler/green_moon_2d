

// Local modules
use crate::vector::{GM_Vec2D};

pub struct GM_Acceleration {
    base: GM_Vec2D,
}

impl GM_Acceleration {
    pub fn new() -> GM_Acceleration {
        GM_Acceleration {
            base: GM_Vec2D::new(),
        }
    }
}