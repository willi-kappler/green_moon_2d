

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
}
