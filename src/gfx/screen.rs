

use crate::misc::{GM_Runtime};
use super::canvas::{GM_Canvas};

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

    fn update(&mut self, runtime: &GM_Runtime) {
    }

    fn draw(&self, canvas: &mut GM_Canvas) {
    }
}
