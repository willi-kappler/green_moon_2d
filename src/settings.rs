
pub struct GM_Settings {
    frame_duration: i16,
}

impl GM_Settings {
    pub fn new() -> GM_Settings {
        GM_Settings {
            frame_duration: 16,
        }
    }

    pub fn frame_duration(&self) -> i16 {
        self.frame_duration
    }
}
