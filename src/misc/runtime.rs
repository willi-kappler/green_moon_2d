



use crate::misc::settings::{GM_Settings};
use crate::misc::event::{GM_Event};

#[derive(Debug)]
pub struct GM_Runtime {
    settings: GM_Settings,
    active_screen: usize,
    quit: bool,
    current_event: GM_Event,
}

impl GM_Runtime {
    pub fn new() -> GM_Runtime {
        GM_Runtime {
            settings: GM_Settings::new(),
            active_screen: 0,
            quit: false,
            current_event: GM_Event::None,
        }
    }

    pub fn quit(&mut self) {
        self.quit = true;
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

    pub fn change_resolution(&mut self, width: u32, height: u32) {
        self.settings.change_resolution(width, height);
    }

    pub fn toggle_fullscreen(&mut self) {
        self.settings.toggle_fullscreen();
    }

    pub fn switch_to_screen(&mut self, new_screen: usize) {
        self.active_screen = new_screen;
    }

    pub fn get_event(&self) -> &GM_Event {
        &self.current_event
    }

    pub fn set_sevent(&mut self, event: GM_Event) {
        self.current_event = event;
    }


}
