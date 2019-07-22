

// Local modules
use crate::misc::position::{GM_Position};

pub struct GM_Sprite {
    sprite_sheet_id: usize,
    position: GM_Position,
    collision_id: usize,
    animation_id: usize,
    active: bool,
}

impl GM_Sprite {
    pub fn new() -> GM_Sprite {
        GM_Sprite {
            sprite_sheet_id: 0,
            position: GM_Position::new(),
            collision_id: 0,
            animation_id: 0,
            active: false,
        }
    }
}

