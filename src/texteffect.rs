

// Local modules
use crate::update::{GM_UpdateElapsed_T};
use crate::font::{GM_BitmapFont};
use crate::texture::{GM_Texture};
use crate::canvas::{GM_Canvas};


pub trait GM_TextEffect_T : GM_UpdateElapsed_T {
    fn draw(&self, text: &str, x: u32, y: u32,
        font_pool: &GM_BitmapFont,
        texture_pool: &Vec<GM_Texture>,
        canvas: &mut GM_Canvas);
}


