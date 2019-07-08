

trait GM_Text_T : GM_Update_T + GM_Draw_T {

}

struct GM_Text {
    font_id: usize,
    text: String,
    position: GM_Position,
}
