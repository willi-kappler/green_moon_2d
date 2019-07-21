
use crate::gfx::screen::{GM_Screen_T};

pub struct GreenMoon2D {
    screen_pool: Vec<Box<dyn GM_Screen_T>>,
    active_screen: usize,
}

impl GreenMoon2D {
    pub fn new() -> GreenMoon2D {
        GreenMoon2D {
            screen_pool: Vec::new(),
            active_screen: 0,
        }
    }

    pub fn add_screen(&mut self, new_screen: Box<dyn GM_Screen_T>) {
        self.screen_pool.push(new_screen);
    }
}
