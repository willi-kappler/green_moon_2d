

// Local modules
use crate::canvas::{GM_Canvas};

pub trait GM_Draw_T {
    fn draw(&self, canvas: &mut GM_Canvas) {
    }
}
