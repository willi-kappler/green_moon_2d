
use nanoserde::DeJson;

#[derive(Debug, Clone, DeJson)]
pub struct GMConfiguration {
    pub(crate) fps: u32,
    pub(crate) window_title: String,
    pub(crate) screen_width: u32,
    pub(crate) screen_height: u32,
    pub(crate) resources: Option<String>,
}

impl GMConfiguration {
    pub fn new() -> Self {
        Self {
            fps: 60,
            window_title: "GreenMoon2D".to_string(),
            screen_width: 1024,
            screen_height: 768,
            resources: None,
        }
    }
}
