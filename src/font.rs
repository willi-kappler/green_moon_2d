

// Local modules
use crate::draw::{GM_Draw_T};
use crate::resources::{GM_Resources};


pub struct GM_BitmapFont {
    texture_id: usize,
    char_width: u16,
    char_height: u16,
    rows: u16,
    cols: u16,
}

impl GM_Draw_T for GM_BitmapFont {
    fn draw(&self, resources: &mut GM_Resources) {
    }
}
