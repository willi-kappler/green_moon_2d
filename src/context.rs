use std::rc::Rc;

use crate::{error::GMError, text::GMTextEffect};
use crate::font::{GMBitmapFont, GMFontManager};
use crate::bitmap::GMBitmap;
use crate::text::{GMText, GMTextEffectManager};
use crate::screen_buffer::GMScreenBuffer;

pub struct GMContext {
    screen_width: u32,
    screen_height: u32,
    window_width: u32,
    window_height: u32,
    full_screen: bool,
    screen_buffer: GMScreenBuffer,
    quit : bool,
    font_manager: GMFontManager,
    text_effect_manager: GMTextEffectManager,
}

impl GMContext {
    pub fn new() -> GMContext {
        GMContext {
            screen_width: 0,
            screen_height: 0,
            window_width: 0,
            window_height: 0,
            full_screen: false,
            screen_buffer: GMScreenBuffer::new(),
            quit: false,
            font_manager: GMFontManager::new(),
            text_effect_manager: GMTextEffectManager::new()
        }
    }

    pub fn exit_game(&self) -> bool {
        self.quit
    }

    pub fn get_font_by_name(&self, font_name: &str) -> Result<Rc<GMBitmapFont>, GMError> {
        self.font_manager.get_font_by_name(font_name)
    }

    pub fn draw_text(&mut self, text: &GMText) -> Result<(), GMError> {
        let font = self.get_font_by_name(text.get_font_name())?;
        let text_effect = self.text_effect_manager.get_text_effect(text.get_text_effect())?;
        text_effect.draw(&text.get_context(), font, &mut self.screen_buffer);
        Ok(())
    }
}
