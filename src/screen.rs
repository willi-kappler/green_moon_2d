
use crate::process::{GM_Process_T};
use crate::update::{GM_UpdateResource_T};
use crate::draw::{GM_Draw_T};

pub trait GM_Screen_T: GM_Process_T + GM_UpdateResource_T + GM_Draw_T {
    fn enter(&mut self) {
    }

    fn leave(&mut self) {
    }
}
