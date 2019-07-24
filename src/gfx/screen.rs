

use crate::misc::runtime::{GM_Runtime};

pub enum GM_ScreenState {
    GM_Enter,
    GM_Normal,
    GM_Leave,
}

pub trait GM_Screen_T {
    fn enter(&mut self) {
    }

    fn process(&mut self, runtime: &mut GM_Runtime) {
    }

    fn update(&mut self, runtime: &mut GM_Runtime) {
    }

    fn draw(&mut self, runtime: &mut GM_Runtime) {
    }
}

#[derive(Debug, Clone)]
pub struct GM_ScreenSize {
    width: u32,
    height: u32,
}

impl GM_ScreenSize {
    pub fn new(width: u32, height: u32) -> GM_ScreenSize {
        GM_ScreenSize {
            width,
            height,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}
