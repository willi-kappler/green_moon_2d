


// Local modules
use crate::active::{GM_Active_T};
use crate::canvas::{GM_Canvas};
use crate::font::{GM_BitmapFont};
use crate::position::{GM_Position, GM_Position_T};
use crate::texture::{GM_Texture};
use crate::update::{GM_Update_Elapsed_T};


pub trait GM_Text_T : GM_Update_Elapsed_T + GM_Active_T {
    fn draw(&self,
        font_pool: &Vec<GM_BitmapFont>,
        texture_pool: &Vec<GM_Texture>,
        canvas: &mut GM_Canvas);
}

pub struct GM_Text {
    font_id: usize,
    text: String,
    position: GM_Position,
    active: bool,
}

impl GM_Text {
    pub fn new() -> GM_Text {
        GM_Text {
            font_id: 0,
            text: "".to_string(),
            position: GM_Position::new(),
            active: true,
        }
    }

    fn draw_char(&self, c: u8, x: u32, y: u32, font: &GM_BitmapFont, texture: &GM_Texture, canvas: &mut GM_Canvas) {

    }
}

impl GM_Update_Elapsed_T for GM_Text {}

impl GM_Active_T for GM_Text {
    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

impl GM_Text_T for GM_Text {
    fn draw(&self,
        font_pool: &Vec<GM_BitmapFont>,
        texture_pool: &Vec<GM_Texture>,
        canvas: &mut GM_Canvas) {

        let font = &font_pool[self.font_id];
        let texture = &texture_pool[font.texture_id];

        let mut px = self.position.get_x();
        let py = self.position.get_y();

        for c in self.text.chars() {
            self.draw_char(c as u8, px, py, font, texture, canvas);
            px += font.char_width as u32;
        }
    }
}
