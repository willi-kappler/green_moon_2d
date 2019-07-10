
pub struct GM_Settings {
    pub frame_duration: i16,
}

impl GM_Settings {
    pub fn new() -> GM_Settings {
        GM_Settings {
            frame_duration: 16,
        }
    }
}
