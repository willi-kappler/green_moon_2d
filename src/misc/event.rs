

#[derive(Debug)]
pub enum GM_Event {
    GM_None,
    GM_Key_Enter_Press,
}

impl GM_Event {
    pub fn new() -> GM_Event {
        GM_Event::GM_None
    }
}