

use crate::misc::runtime::{GM_Runtime};

pub enum GM_Screen_State {
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
