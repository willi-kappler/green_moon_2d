use crate::font::{GMBitmapFont, GMFontT};

use std::rc::Rc;

pub trait GMTextT {
    fn draw(&self);
    fn update(&mut self) {
    }
    fn set_text(&mut self, text: &str);
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn set_font(&mut self, font: Rc<GMBitmapFont>);
    fn from_other(&mut self, other: GMText);
}

pub struct GMText {
    pub(crate) data: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) font: Rc<GMBitmapFont>,
}

impl GMText {
    pub fn new(text: &str, x: f32, y: f32, font: Rc<GMBitmapFont>) -> Self {
        Self {
            data: text.to_string(),
            x,
            y,
            font,
        }
    }
}

impl GMTextT for GMText {
    fn draw(&self) {
        let mut current_x = self.x;

        for c in self.data.chars() {
            self.font.draw(c, current_x, self.y);
            current_x += self.font.char_width;
        }
    }

    fn set_text(&mut self, text: &str) {
        self.data = text.to_string();
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn set_font(&mut self, font: Rc<GMBitmapFont>) {
        self.font = font;
    }

    fn from_other(&mut self, other: GMText) {
        self.data = other.data;
        self.x = other.x;
        self.y = other.y;
        self.font = other.font;
    }
}
