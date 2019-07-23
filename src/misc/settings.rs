
#[derive(Debug)]
pub struct GM_Settings {
    screen_width: u32,
    screen_height: u32,
    fullscreen: bool,
    music_volume: u8,
    sfx_volume: u8,
    frame_duration: i16,
}

impl GM_Settings {
    pub fn new() -> GM_Settings {
        GM_Settings {
            screen_width: 1024,
            screen_height: 768,
            fullscreen: false,
            music_volume: 128,
            sfx_volume: 128,
            frame_duration: 16,
        }
    }

    pub fn load(&mut self, path: String) {
        unimplemented!("GM_Settings::load(...) not implemented yet!");
    }

    pub fn save(&self, path: String) {
        unimplemented!("GM_Settings::save(...) not implemented yet!");
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        (self.screen_width, self.screen_height)
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.screen_width = width;
        self.screen_height = height;
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
