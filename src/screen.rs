
pub trait GM_Screen_T: GM_Process_T + GM_Update_T + GM_Draw_T {
    pub fn enter(&mut self) {
    }

    pub leave(&mut self) {
    }
}
