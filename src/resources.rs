
// Local modules
use crate::canvas::{GM_Canvas};
use crate::settings::{GM_Settings};
use crate::sound::{GM_Sound};
use crate::music::{GM_Music};


pub struct GM_Resources {
    // GFX
    canvas: GM_Canvas,

    // SFX
    sound_pool: Vec<GM_Sound>,
    music_pool: Vec<GM_Music>,

    // Misc
    settings: GM_Settings,
    pub quit: bool,
    pub time_elapsed: u16,
}

impl GM_Resources {
    pub fn new() -> GM_Resources {
        GM_Resources {
            canvas: GM_Canvas::new(),
            sound_pool: Vec::new(),
            music_pool: Vec::new(),
            settings: GM_Settings::new(),
            quit: false,
            time_elapsed: 0,
        }
    }

    pub fn quit_game(&mut self) {
        self.quit = true;
    }

    pub fn update(&mut self) {
        self.canvas.update(self.time_elapsed);
    }

    pub fn draw(&mut self) {
        self.canvas.draw();
    }
}
