use crate::error::GMError;
use crate::font::GMBitmapFont;
use crate::text::{GMText, GMTextEffect, GMTextEffectWrapper};
use crate::screen_buffer::GMScreenBuffer;
use crate::resource_manager::GMResourceManager;

pub struct GMContext {
    screen_width: u32,
    screen_height: u32,
    window_width: u32,
    window_height: u32,
    full_screen: bool,
    screen_buffer: GMScreenBuffer,
    quit : bool,
    current_fps: f32,
    expected_fps: f32,
    font_manager: GMResourceManager<GMBitmapFont>,
    text_effect_manager: GMResourceManager<GMTextEffectWrapper>,
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
            current_fps: 0.0,
            expected_fps: 60.0,
            font_manager: GMResourceManager::new("FontManager"),
            text_effect_manager: GMResourceManager::new("TextEffectManager"),
        }
    }

    pub fn exit_game(&self) -> bool {
        self.quit
    }
/*
    pub fn get_font_by_name(&self, font_name: &str) -> Result<Rc<GMBitmapFont>, GMError> {
        self.font_manager.get_font_by_name(font_name)
    }
*/
    pub fn draw_text(&mut self, text: &GMText) -> Result<(), GMError> {
        let text_effect = text.get_text_effect();

        text_effect.draw(&text.text_context, &mut self.screen_buffer)
    }
}
