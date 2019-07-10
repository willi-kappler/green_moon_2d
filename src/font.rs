

// Local modules
use crate::canvas::{GM_Canvas};
use crate::draw::{GM_Draw_T};


pub struct GM_BitmapFont {
    texture_id: usize,
    char_width: u16,
    char_height: u16,
    rows: u16,
    cols: u16,
}

impl GM_Draw_T for GM_BitmapFont {
    fn draw(&self, canvas: &mut GM_Canvas) {
    }
}
