
use crate::misc::{GM_Process_T, GM_Update_T, GM_Draw_T};

pub enum GM_Screen_State {
    GM_Enter,
    GM_Normal,
    GM_Leave,
}

pub trait GM_Screen_T: GM_Process_T + GM_Update_T + GM_Draw_T {
    fn enter(&mut self) {
    }
}
