

// Rust modules
use std::rc::Rc;


// Local modules
use super::texture::{GM_Texture};


pub struct GM_BitmapFont {
    // Use spritesheet instead ?
    texture: GM_Texture,
    char_width: u16,
    char_height: u16,
    rows: u16,
    cols: u16,
}

impl GM_BitmapFont {
    pub fn new(texture: GM_Texture, char_width: u16, char_height: u16,
        rows: u16, cols: u16) -> GM_BitmapFont {
        GM_BitmapFont {
            texture,
            char_width,
            char_height,
            rows,
            cols,
        }
    }

    pub fn load(path: &str) -> Rc<GM_BitmapFont> {
        // TODO: implement it !

        Rc::new(GM_BitmapFont {
            texture: GM_Texture::new(0, 0, Vec::new()),
            char_width: 0,
            char_height: 0,
            rows: 0,
            cols: 0,
        }) 
    }
}
