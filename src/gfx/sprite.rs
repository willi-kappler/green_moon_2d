

// Local modules
use crate::misc::{GM_Position};
use crate::misc::{GM_CollisionShape};

pub trait GM_Sprite_T {

}

pub struct GM_Sprite {
    sprite_sheet_id: usize,
    position: GM_Position,
    collision: GM_CollisionShape,
    current_animation: usize,
}

impl GM_Sprite {
    pub fn new() -> GM_Sprite {
        GM_Sprite {
            sprite_sheet_id: 0,
            position: GM_Position::new(0, 0),
            collision: GM_CollisionShape::GM_Empty,
            current_animation: 0,
        }
    }

    // get_pos
    // set_pos
    // move_rel
    // set_anim

}

