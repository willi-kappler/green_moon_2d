
pub struct GMOption {
    sound_volume: f32,
    music_volume: f32,
    screen_width: f32,
    screen_height: f32,
    language: String,
    // TODO: keyboard / controller options
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
}
