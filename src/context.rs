use crate::error::{GMError};
use crate::font::{GMBitmapFont, GMFontManager};
use crate::canvas::{GMCanvas};


pub struct GMContext {
    screen_width: u32,
    screen_height: u32,
    window_width: u32,
    window_height: u32,
    full_screen: bool,
    quit : bool,
    font_manager: GMFontManager,
    canvas: GMCanvas,
}

impl GMContext {
    pub fn new() -> GMContext {
        GMContext {
            screen_width: 0,
            screen_height: 0,
            window_width: 0,
            window_height: 0,
            full_screen: false,
            quit: false,
            font_manager: GMFontManager::new(),
            canvas: GMCanvas::new(),
        }
    }

    pub fn exit_game(&self) -> bool {
        self.quit
    }

    pub fn get_font_by_name(&self, font_name: &str) -> Result<&GMBitmapFont, GMError> {
        self.font_manager.get_font_by_name(font_name)
    }
}
