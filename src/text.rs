


// Local modules
use crate::active::{GM_Active_T};
use crate::canvas::{GM_Canvas};
use crate::font::{GM_BitmapFont};
use crate::position::{GM_Position, GM_Position_T};
use crate::texteffect::{GM_TextEffect_T};
use crate::texture::{GM_Texture};



pub struct GM_Text {
    font_id: usize,
    text_effect_id: usize,
    text: String,
    position: GM_Position,
    active: bool,
}

impl GM_Text {
    pub fn new() -> GM_Text {
        GM_Text {
            font_id: 0,
            text_effect_id: 0,
            text: "".to_string(),
            position: GM_Position::new(),
            active: true,
        }
    }

    pub fn update(&self, time_elapsed: u16, text_effect_pool: &mut Vec<Box<dyn GM_TextEffect_T>>) {
        let text_effect = &mut text_effect_pool[self.text_effect_id];
        text_effect.update(time_elapsed);
    }

    pub fn draw(&self,
        font_pool: &Vec<GM_BitmapFont>,
        texture_pool: &Vec<GM_Texture>,
        text_effect_pool: &Vec<Box<dyn GM_TextEffect_T>>,
        canvas: &mut GM_Canvas) {

        let text_effect = &text_effect_pool[self.text_effect_id];
        let font = &font_pool[self.font_id];

        let px = self.position.get_x();
        let py = self.position.get_y();

        text_effect.draw(&self.text, px, py, font, texture_pool, canvas);
    }
}

impl GM_Active_T for GM_Text {
    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}
