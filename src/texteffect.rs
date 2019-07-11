

// Local modules
use crate::update::{GM_UpdateElapsed_T};
use crate::font::{GM_BitmapFont};
use crate::texture::{GM_Texture};
use crate::canvas::{GM_Canvas};


pub trait GM_TextEffect_T: GM_UpdateElapsed_T {
    fn draw(&self, text: &str, x: u32, y: u32,
        font: &GM_BitmapFont,
        texture_pool: &Vec<GM_Texture>,
        canvas: &mut GM_Canvas);
}

pub struct GM_TE_Static {}

impl GM_UpdateElapsed_T for GM_TE_Static {}

impl GM_TextEffect_T for GM_TE_Static {
    fn draw(&self, text: &str, x: u32, y: u32,
        font: &GM_BitmapFont,
        texture_pool: &Vec<GM_Texture>,
        canvas: &mut GM_Canvas) {

        let mut px = x;
        
        for c in text.chars() {
            font.draw_char(c as u8, px, y, texture_pool, canvas);
            px += font.get_char_width() as u32;
        }
    }
}
