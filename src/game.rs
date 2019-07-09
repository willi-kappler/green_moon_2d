use std::time::{Instant, Duration};
use std::thread;

pub trait GM_Game_T : GM_Process_T + GM_Update_T + GM_Draw_T {
    fn initialize(&mut self) {
    }
}

struct GreenMoon2D<U> {
    resources: GM_Resources,
    settinge: GM_Settings,
    actual_game: U,
}

impl<U> GreenMoon2D<U> {
    pub fn new<U: GM_Game_T>(actual_game: U) -> Result<GreenMoon2D, GM_Init_Error> {
        GreenMoon2D {
            resources: GM_Resources::new(),
            settings: GM_Settings::new(),
            actual_game,
        }
    }

    pub fn run(&mut self) -> Result<(), GM_Game_Error> {
        self.actual_game.initialize(&mut self.resources);

        while !self.resources.quit {
            let instant = Instant::now();

            self.process();
            self.update();
            self.draw();

            let sleep_time = self.settings.frame_duration - (instant.elapsed().as_millis() as i16);
            if sleep_time > 0 {
                thread::sleep(Duration::from_millis(sleep_time as u64))
            }

            self.resources.time_elapsed = instant.elapsed().as_millis() as u16;
        }
    }

    fn process(&mut self) {
        self.actual_game.process(event, &mut self.resources);
    }

    fn update(&mut self) {
        self.resources.update();
        self.actual_game.update(&mut self.resources);
    }

    fn draw(&mut self) {
        self.resources.draw();
        self.actual_game.draw(&mut self.resources);
    }
}

pub enum GM_Init_Error {
    IO,
}

pub enum GM_Game_Error {
    TODO,
}
