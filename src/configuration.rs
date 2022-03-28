
use serde_derive::Deserialize;

#[derive(Clone, Deserialize)]
pub struct GMConfiguration {
    pub fps: f32,
    pub window_title: String,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl GMConfiguration {
    pub fn new() -> Self {
        Self {
            fps: 60.0,
            window_title: "GreenMoon2D".to_string(),
            screen_width: 1024,
            screen_height: 768,
        }
    }
}
