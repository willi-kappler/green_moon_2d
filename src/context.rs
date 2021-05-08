
pub struct GMContext {
    screen_width: u32,
    screen_height: u32,
    window_width: u32,
    window_height: u32,
    full_screen: bool,
}

impl GMContext {
    pub fn new() -> GMContext {
        GMContext {
            screen_width: 0,
            screen_height: 0,
            window_width: 0,
            window_height: 0,
            full_screen: false,
        }
    }
}
