use crate::font::{GMBitmapFont, GMFontT};

use std::rc::Rc;

pub trait GMTextT {
    fn draw(&self);
    fn update(&mut self) {
    }
    fn set_text(&mut self, text: &str);
    fn get_text(&self) -> &str;
    fn set_x(&mut self, x: f32);
    fn get_x(&self) -> f32;
    fn set_y(&mut self, y: f32);
    fn get_y(&self) -> f32;
    fn set_font(&mut self, font: Rc<GMBitmapFont>);
    fn get_font(&self) -> &Rc<GMBitmapFont>;
    fn from_other(&mut self, other: Box<dyn GMTextT>);
    fn get_extend(&self) -> (f32, f32);
}

pub struct GMText {
    pub(crate) data: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) font: Rc<GMBitmapFont>,
}

impl GMText {
    pub fn new(text: &str, x: f32, y: f32, font: &Rc<GMBitmapFont>) -> Self {
        Self {
            data: text.to_string(),
            x,
            y,
            font: font.clone(),
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

    fn get_text(&self) -> &str {
        &self.data
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn get_x(&self) -> f32 {
        self.x
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn set_font(&mut self, font: Rc<GMBitmapFont>) {
        self.font = font;
    }

    fn get_font(&self) -> &Rc<GMBitmapFont> {
        &self.font
    }

    fn from_other(&mut self, other: Box<dyn GMTextT>) {
        self.data = other.get_text().to_string();
        self.x = other.get_x();
        self.y = other.get_y();
        self.font = other.get_font().clone();
    }

    fn get_extend(&self) -> (f32, f32) {
        let mut text_width: f32 = 0.0;
        let mut text_height: f32 = 0.0;

        for c in self.data.chars() {
            let (char_width, char_height) = self.font.get_extend(c);
            text_width += char_width;
            text_height = text_height.max(char_height);
        }

        (text_width, text_height)
    }
}

pub struct GMArrowText {
    base: Box<dyn GMTextT>,
}

impl GMArrowText {
    pub fn new(mut base: Box<dyn GMTextT>) -> Self {
        let text = format!("-> {} <-", base.get_text());
        base.set_text(&text);

        Self {
            base,
        }
    }
}

impl GMTextT for GMArrowText {
    fn draw(&self) {
        self.base.draw();
    }

    fn set_text(&mut self, text: &str) {
        let arrow_text = format!("-> {} <-", text);
        self.base.set_text(&arrow_text);
    }

    fn get_text(&self) -> &str {
        self.base.get_text()
    }

    fn set_x(&mut self, x: f32) {
        self.base.set_x(x);
    }

    fn get_x(&self) -> f32 {
        self.base.get_x()
    }

    fn set_y(&mut self, y: f32) {
        self.base.set_y(y);
    }

    fn get_y(&self) -> f32 {
        self.base.get_y()
    }

    fn set_font(&mut self, font: Rc<GMBitmapFont>) {
        self.base.set_font(font);
    }

    fn get_font(&self) -> &Rc<GMBitmapFont> {
        self.base.get_font()
    }

    fn from_other(&mut self, other: Box<dyn GMTextT>) {
        self.base.from_other(other);
    }

    fn get_extend(&self) -> (f32, f32) {
        self.base.get_extend()
    }
}
