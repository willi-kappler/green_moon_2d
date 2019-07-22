
use crate::gfx::screen::{GM_Screen_T};
use crate::misc::runtime::{GM_Runtime};
use crate::misc::settings::{GM_Settings};

pub struct GreenMoon2D {
    runtime: GM_Runtime,
    screen_pool: Vec<Box<dyn GM_Screen_T>>,
}

impl GreenMoon2D {
    pub fn new() -> GreenMoon2D {
        GreenMoon2D {
            runtime: GM_Runtime::new(),
            screen_pool: Vec::new(),
        }
    }

    pub fn set_settings(&mut self, new_settings: GM_Settings) {
        self.runtime.set_settings(new_settings);
    }

    // TODO: change to AsRef
    pub fn load_settings(&mut self, path: String) {
        self.runtime.load_settings(path);
    }

    pub fn save_settings(&self, path: String) {
        self.runtime.save_settings(path);
    }

    pub fn add_screen(&mut self, new_screen: Box<dyn GM_Screen_T>) {
        self.screen_pool.push(new_screen);
    }

    pub fn run(&mut self) {

    }
}
