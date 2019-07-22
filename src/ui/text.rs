

// Local modules
use crate::misc::position::{GM_Position};


pub struct GM_Text {
    font_id: usize,
    text_effect_id: usize,
    text: String,
    position: GM_Position,
}

impl GM_Text {
    pub fn new() -> GM_Text {
        GM_Text {
            font_id: 0,
            text_effect_id: 0,
            text: "".to_string(),
            position: GM_Position::new(),
        }
    }

    // set_text
    // get_text
    // set_pos
    // get_pos
}