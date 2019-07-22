

// Local modules
use crate::misc::position::{GM_Position};
use crate::misc::collision::{GM_CollisionShape};
use super::animation::{GM_Animation_T};

pub trait GM_Sprite_T {

}

pub struct GM_Sprite {
    sprite_sheet_id: usize,
    position: GM_Position,
    collision: GM_CollisionShape,
    animations: Vec<Box<dyn GM_Animation_T>>,
    current_animation: usize,
}

impl GM_Sprite {
    pub fn new() -> GM_Sprite {
        GM_Sprite {
            sprite_sheet_id: 0,
            position: GM_Position::new(),
            collision: GM_CollisionShape::GM_Empty,
            animations: Vec::new(),
            current_animation: 0,
        }
    }

    // get_pos
    // set_pos
    // move_rel
    // set_anim

}

