
use nanoserde::DeJson;

#[derive(Clone, DeJson)]
pub struct GMConfiguration {
    pub fps: u32,
    pub window_title: String,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl GMConfiguration {
    pub fn new() -> Self {
        Self {
            fps: 60,
            window_title: "GreenMoon2D".to_string(),
            screen_width: 1024,
            screen_height: 768,
        }
    }
}
