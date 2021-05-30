
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GMTextAlignment {
    LeftOrTop,
    Center,
    RightOrBottom,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMText {
    pub(crate) content: String,
    pub(crate) px: f32,
    pub(crate) py: f32,
    pub(crate) horizontal: bool,
    pub(crate) alignment: GMTextAlignment,
}

impl GMText {
    pub fn new(text: &str, px: f32, py: f32) -> GMText {
        GMText {
            content: text.to_string(),
            px,
            py,
            horizontal: true,
            alignment: GMTextAlignment::LeftOrTop,
        }
    }

    pub fn set_text(&mut self, content: &str) {
        self.content = content.to_string();
    }

    pub fn set_position(&mut self, px: f32, py: f32) {
        self.px = px;
        self.py = py;
    }
}
