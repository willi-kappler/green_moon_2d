

// Local modules
use crate::canvas::{GM_Canvas};
use crate::texture::{GM_Texture};

pub struct GM_BitmapFont {
    texture_id: usize,
    char_width: u16,
    char_height: u16,
    rows: u16,
    cols: u16,
}

impl GM_BitmapFont {
    fn draw_char(&self, c: u8, x: u32, y: u32, texture_pool: &Vec<GM_Texture>, canvas: &mut GM_Canvas) {

    }
}
