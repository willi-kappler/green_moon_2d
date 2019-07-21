
use crate::misc::{GM_Process_T, GM_Update_T, GM_Draw_T};

pub trait GM_Screen_T: GM_Process_T + GM_Update_T + GM_Draw_T {
    fn enter(&mut self) {
    }

    fn leave(&mut self) {
    }
}
