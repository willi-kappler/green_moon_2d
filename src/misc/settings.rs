
#[derive(Debug)]
pub struct GM_Settings {
    screen_width: u32,
    screen_height: u32,
    fullscreen: bool,
    music_volume: u8,
    sfx_volume: u8,
}

impl GM_Settings {
    pub fn new() -> GM_Settings {
        GM_Settings {
            screen_width: 1024,
            screen_height: 768,
            fullscreen: false,
            music_volume: 128,
            sfx_volume: 128,
        }
    }

    pub fn load(&mut self, path: String) {

    }

    pub fn save(&self, path: String) {

    }

    pub fn change_resolution(&mut self, width: u32, height: u32) {
        self.screen_width = width;
        self.screen_height = height;
    }

    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen
    }
}
