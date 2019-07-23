



use crate::gfx::canvas::{GM_Canvas};
use super::settings::{GM_Settings};
use super::event::{GM_Event};


pub struct GM_Runtime {
    settings: GM_Settings,
    current_screen: usize,
    screen_switched: bool,
    quit: bool,
    current_event: GM_Event,
    time_elapsed: u16,
    canvas: GM_Canvas,
}

impl GM_Runtime {
    pub fn new() -> GM_Runtime {
        GM_Runtime {
            settings: GM_Settings::new(),
            current_screen: 0,
            screen_switched: false,
            quit: false,
            current_event: GM_Event::None,
            time_elapsed: 0,
            canvas: GM_Canvas::new(),
        }
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn game_still_running(&self) -> bool {
        !self.quit
    }

    pub fn set_settings(&mut self, new_settings: GM_Settings) {
        self.settings = new_settings;
    }

    pub fn load_settings(&mut self, path: String) {
        self.settings.load(path);
    }

    pub fn save_settings(&self, path: String) {
        self.settings.save(path);
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.settings.set_resolution(width, height);
    }

    pub fn toggle_fullscreen(&mut self) {
        self.settings.toggle_fullscreen();
    }

    pub fn switch_to_screen(&mut self, new_screen: usize) {
        if self.current_screen != new_screen {
            self.current_screen = new_screen;
            self.screen_switched = true;
        }
    }

    pub fn is_screen_switching(&self) -> bool {
        self.screen_switched
    }

    pub fn screen_switching_done(&mut self) {
        self.screen_switched = false;
    }

    pub fn get_current_screen(&self) -> usize {
        self.current_screen
    }

    pub fn get_event(&self) -> &GM_Event {
        &self.current_event
    }

    pub fn set_event(&mut self, event: GM_Event) {
        self.current_event = event;
    }

    pub fn get_frame_duration(&self) -> i16 {
        self.settings.get_frame_duration()
    }

    pub fn set_time_elapsed(&mut self, elapsed: u16) {
        self.time_elapsed = elapsed;
    }

    pub fn get_time_elapsed(&mut self) -> u16 {
        self.time_elapsed
    }

}
