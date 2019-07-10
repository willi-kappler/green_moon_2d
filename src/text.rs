


// Local modules
use crate::active::{GM_Active_T};
use crate::canvas::{GM_Canvas};
use crate::font::{GM_BitmapFont};
use crate::position::{GM_Position};
use crate::texture::{GM_Texture};
use crate::update::{GM_Update_Elapsed_T};


pub trait GM_Text_T : GM_Update_Elapsed_T + GM_Active_T {
    fn draw(&self,
        font_pool: &Vec<GM_BitmapFont>,
        texture_pool: &Vec<GM_Texture>,
        canvas: &mut GM_Canvas
    );
}

pub struct GM_Text {
    font_id: usize,
    text: String,
    position: GM_Position,
}

impl GM_Text {
    pub fn new() -> GM_Text {
        GM_Text {
            font_id: 0,
            text: "".to_string(),
            position: GM_Position::new(),
        }
    }
}

impl GM_Update_Elapsed_T for GM_Text {
    fn update(&mut self, elapsed: u16) {
    }
}
