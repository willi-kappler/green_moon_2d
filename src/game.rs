

// Rust modules
use std::thread;
use std::time::{Instant, Duration};

// Local modules
use crate::draw::{GM_Draw_T};
use crate::event::{GM_Event};
use crate::process::{GM_Process_T};
use crate::resources::{GM_Resources};
use crate::settings::{GM_Settings};
use crate::update::{GM_UpdateResource_T};


pub trait GM_Game_T : GM_Process_T + GM_UpdateResource_T + GM_Draw_T {
    fn initialize(&mut self, resources: &mut GM_Resources) {
    }
}

pub struct GreenMoon2D<U> {
    resources: GM_Resources,
    settings: GM_Settings,
    actual_game: U,
}

impl<U: GM_Game_T> GreenMoon2D<U> {
    pub fn new(actual_game: U) -> Result<GreenMoon2D<U>, GM_Init_Error> {
        Ok(GreenMoon2D {
            resources: GM_Resources::new(),
            settings: GM_Settings::new(),
            actual_game,
        })
    }

    pub fn run(&mut self) -> Result<(), GM_Game_Error> {
        self.actual_game.initialize(&mut self.resources);

        while self.game_still_running() {
            let instant = Instant::now();
            let event = GM_Event::new();

            self.process(&event);
            self.update();
            self.draw();

            let sleep_time = self.settings.frame_duration - (instant.elapsed().as_millis() as i16);
            if sleep_time > 0 {
                thread::sleep(Duration::from_millis(sleep_time as u64))
            }

            self.resources.time_elapsed = instant.elapsed().as_millis() as u16;
        }

        Ok(())
    }

    fn game_still_running(&self) -> bool {
        self.resources.game_still_running()
    }

    fn process(&mut self, event: &GM_Event) {
        self.actual_game.process(event, &mut self.resources);
    }

    fn update(&mut self) {
        self.resources.update();
        self.actual_game.update(&mut self.resources);
    }

    fn draw(&mut self) {
        self.resources.draw();
        self.actual_game.draw(&mut self.resources.canvas);
    }
}

pub enum GM_Init_Error {
    IO,
}

pub enum GM_Game_Error {
    TODO,
}
