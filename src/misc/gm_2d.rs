
// Rust modules
use std::thread;
use std::time::{Instant, Duration};




// Local modules
use crate::gfx::screen::{GM_Screen_T};
use super::runtime::{GM_Runtime};
use super::settings::{GM_Settings};
use super::event::{GM_Event};

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
        assert!(!self.screen_pool.is_empty(), "No screens found, use the add_screen(...) method!");

        while self.runtime.game_still_running() {
            let instant = Instant::now();
            let event = GM_Event::new();
            self.runtime.set_event(event);

            let current_screen = self.runtime.get_current_screen();
            self.screen_pool[current_screen].process(&mut self.runtime);
            self.screen_pool[current_screen].update(&mut self.runtime);
            self.screen_pool[current_screen].draw(&mut self.runtime);

            let sleep_time = self.runtime.get_frame_duration() - (instant.elapsed().as_millis() as i16);
            if sleep_time > 0 {
                thread::sleep(Duration::from_millis(sleep_time as u64))
            }

            self.runtime.set_time_elapsed(instant.elapsed().as_millis() as u16);

            if self.runtime.is_screen_switching() {
                self.screen_pool[self.runtime.get_current_screen()].enter();
                self.runtime.screen_switching_done();
            }
        }
    }
}
