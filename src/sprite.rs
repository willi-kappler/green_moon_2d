
// Local modules
use crate::active::{GM_Active_T};
use crate::animation::{GM_Animation_T};
use crate::canvas::{GM_Canvas};
use crate::position::{GM_Position, GM_Position_T};
use crate::spritesheet::{GM_SpriteSheet};
use crate::texture::{GM_Texture};
use crate::update::{GM_UpdateElapsed_T};

// TODO:
// - Add GM_Behaviour_T
// - Add GM_SpriteEffect_T

pub trait GM_Sprite_T: GM_UpdateElapsed_T + GM_Active_T + GM_Position_T {
    fn draw(&self,
        sprite_sheet_pool: &Vec<GM_SpriteSheet>,
        texture_pool: &Vec<GM_Texture>,
        animation_pool: &Vec<Box<dyn GM_Animation_T>>,
        canvas: &mut GM_Canvas);
}



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

impl GM_UpdateElapsed_T for GM_Sprite {
    fn update(&mut self, time_elapsed: u16) {
    }    
}

impl GM_Sprite_T for GM_Sprite {
    fn draw(&self,
        sprite_sheet_pool: &Vec<GM_SpriteSheet>,
        texture_pool: &Vec<GM_Texture>,
        animation_pool: &Vec<Box<dyn GM_Animation_T>>,
        canvas: &mut GM_Canvas) {

        let sprite_sheet = &sprite_sheet_pool[self.sprite_sheet_id];
        let animation = &animation_pool[self.animation_id];
        let current_frame = animation.current_frame();

        sprite_sheet.draw(self.get_x(), self.get_y(), current_frame, texture_pool, canvas);
    }
}

impl GM_Active_T for GM_Sprite {
    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }
}

impl GM_Position_T for GM_Sprite {
    fn get_x(&self) -> u32 {
        self.position.get_x()
    }

    fn get_y(&self) -> u32 {
        self.position.get_y()
    }
}
