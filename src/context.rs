use std::time::Instant;

use crate::error::GMError;
use crate::font::GMBitmapFont;
use crate::text::{GMText, GMTextEffect, GMTextEffectWrapper};
use crate::screen_buffer::GMScreenBuffer;
use crate::resource_manager::GMResourceManager;

pub struct GMContext {
    pub(crate) screen_width: u32,
    pub(crate) screen_height: u32,
    pub(crate) window_width: u32,
    pub(crate) window_height: u32,
    pub(crate) full_screen: bool,
    pub(crate) screen_buffer: GMScreenBuffer,
    pub(crate) quit : bool,
    pub(crate) instant: Instant,
    pub(crate) current_fps: f32,
    pub(crate) expected_fps: f32,
    pub(crate) expected_duration: f32,
    pub(crate) font_manager: GMResourceManager<GMBitmapFont>,
    pub(crate) text_effect_manager: GMResourceManager<GMTextEffectWrapper>,
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
            instant: Instant::now(),
            current_fps: 0.0,
            expected_fps: 60.0,
            expected_duration: 1000.0 / 60.0,
            font_manager: GMResourceManager::new("FontManager"),
            text_effect_manager: GMResourceManager::new("TextEffectManager"),
        }
    }

    pub fn elapsed(&self) -> f32 {
        self.instant.elapsed().as_millis() as f32
    }

    pub fn exit_game(&self) -> bool {
        self.quit
    }
}
