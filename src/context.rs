use std::rc::Rc;

use crate::error::GMError;
use crate::font::{GMBitmapFont, GMFontManager};
use crate::text::GMTextContext;
use crate::bitmap::GMBitmap;

pub struct GMContext {
    screen_width: u32,
    screen_height: u32,
    window_width: u32,
    window_height: u32,
    full_screen: bool,
    screen_buffer: Vec<u8>, // TODO: Use pixels
    quit : bool,
    font_manager: GMFontManager,
}

impl GMContext {
    pub fn new() -> GMContext {
        GMContext {
            screen_width: 0,
            screen_height: 0,
            window_width: 0,
            window_height: 0,
            full_screen: false,
            screen_buffer: Vec::new(), // TODO: Use pixels
            quit: false,
            font_manager: GMFontManager::new(),
        }
    }

    pub fn exit_game(&self) -> bool {
        self.quit
    }

    pub fn get_font_by_name(&self, font_name: &str) -> Result<Rc<GMBitmapFont>, GMError> {
        self.font_manager.get_font_by_name(font_name)
    }

    pub fn blit_bitmap(&mut self, bitmap: &GMBitmap, px: u32, py: u32) {

    }
}
