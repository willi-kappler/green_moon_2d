
// Local modules

use crate::position::{GM_Position, GM_Position_T};
use crate::velocity::{GM_Velocity};
use crate::acceleration::{GM_Acceleration};
use crate::update::{GM_Update_Elapsed_T};
use crate::draw::{GM_Draw_T};
use crate::active::{GM_Active_T};

pub trait GM_Sprite_T: GM_Update_Elapsed_T + GM_Draw_T + GM_Active_T + GM_Position_T {
    fn get_sprite_sheet_id(&self) -> usize;
    fn get_animation_id(&self) -> usize;
}



pub struct GM_Sprite {
    sprite_sheet_id: usize,
    position: GM_Position,
    velocity: GM_Velocity,
    acceleration: GM_Acceleration,
    collision_id: usize,
    animation_id: usize,
    // path_id: usize,
}

impl GM_Sprite {
    pub fn new() -> GM_Sprite {
        GM_Sprite {
            sprite_sheet_id: 0,
            position: GM_Position::new(),
            velocity: GM_Velocity::new(),
            acceleration: GM_Acceleration::new(),
            collision_id: 0,
            animation_id: 0,
            // path_id: 0,
        }
    }
}

impl GM_Update_Elapsed_T for GM_Sprite {
    fn update(&mut self, time_elapsed: u16) {
        self.velocity.update(&self.acceleration, time_elapsed);
        self.position.update(&self.velocity, time_elapsed);
    }    
}
