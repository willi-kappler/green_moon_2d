

trait GM_Text_T : GM_Update_T + GM_Draw_T {
}

struct GM_Text {
    font_id: usize,
    text: String,
    position: GM_Position,
}

impl GM_Text {
    pub fn new() -> GM_Text {
        GM_Text {
            font_id: 0,
            text: "".to_string(),
            position: GM_Position::new(),
        }
    }
}

impl GM_Update_T for GM_Text {
    fn update(&mut self) {
    }
}
