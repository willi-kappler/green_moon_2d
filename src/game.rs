

trait GM_Game_T : GM_Process_T + GM_Update_T + GM_Draw_T {
    fn initialize(&mut self) {

    }
}


struct GreenMoon2D<U> {
    resources: GM_Resources,
    settinge: GM_Settings,
    actual_game: U,
}
