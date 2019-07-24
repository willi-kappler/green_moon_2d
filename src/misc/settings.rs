

use crate::gfx::screen::{GM_ScreenSize};

#[derive(Debug)]
pub struct GM_Settings {
    screen_size: GM_ScreenSize,
    fullscreen: bool,
    music_volume: u8,
    sfx_volume: u8,
    frame_duration: i16,
}

impl GM_Settings {
    pub fn new() -> GM_Settings {
        GM_Settings {
            screen_size: GM_ScreenSize::new(1024, 758),
            fullscreen: false,
            music_volume: 128,
            sfx_volume: 128,
            frame_duration: 16,
        }
    }

    pub fn load(path: &str) -> GM_Settings {
        // TODO: Load settings from file
        // unimplemented!("GM_Settings::load(...) not implemented yet!");
        GM_Settings::new()
    }

    pub fn save(&self, path: &str) {
        unimplemented!("GM_Settings::save(...) not implemented yet!");
    }

    pub fn get_screen_size(&self) -> GM_ScreenSize {
        self.screen_size.clone()
    }

    pub fn set_screen_size(&mut self, screen_size: GM_ScreenSize) {
        self.screen_size = screen_size;
    }

    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
    }

    pub fn get_fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    pub fn get_frame_duration(&self) -> i16 {
        self.frame_duration
    }

    pub fn set_frame_duration(&mut self, frame_duration: i16) {
        self.frame_duration = frame_duration;
    }
}
