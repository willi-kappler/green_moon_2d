


// TODO:
// - keyboard / controller options
// - load and save options


pub struct GMOption {
    sound_volume: f32,
    music_volume: f32,
    screen_width: f32,
    screen_height: f32,
    language: String,
}

impl GMOption {
    pub fn new() -> Self {
        Self {
            sound_volume: 1.0,
            music_volume: 1.0,
            screen_width: 800.0,
            screen_height: 600.0,
            language: "eng".to_string(),
        }
    }
    pub fn get_sound_volume(&self) -> f32 {
        self.sound_volume
    }
    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }
    pub fn get_screen_width(&self) -> f32 {
        self.screen_width
    }
    pub fn get_screen_height(&self) -> f32 {
        self.screen_height
    }
    pub fn get_language(&self) -> &str {
        &self.language
    }
    pub fn set_sound_volume(&mut self, volume: f32) {
        self.sound_volume = volume;
    }
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume= volume;
    }
    pub fn set_screen_width(&mut self, width: f32) {
        self.screen_width = width;
    }
    pub fn set_screen_height(&mut self, height: f32) {
        self.screen_height = height;
    }
    pub fn set_language(&mut self, language: &str) {
        self.language = language.to_string();
    }
}
