use crate::font::{GMFontT};
use crate::sprite::GMSprite;

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
    fn set_font(&mut self, font: &Rc<dyn GMFontT>);
    fn get_font(&self) -> &Rc<dyn GMFontT>;
    fn from_other(&mut self, other: Box<dyn GMTextT>);
    fn get_extend(&self) -> (f32, f32);
}

pub struct GMStaticText {
    pub(crate) data: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) font: Rc<dyn GMFontT>,
}

impl GMStaticText {
    pub fn new(text: &str, x: f32, y: f32, font: &Rc<dyn GMFontT>) -> Self {
        Self {
            data: text.to_string(),
            x,
            y,
            font: font.clone(),
        }
    }
    pub fn new_box(text: &str, x: f32, y: f32, font: &Rc<dyn GMFontT>) -> Box<dyn GMTextT> {
        Box::new(Self::new(text, x, y, font))
    }
}

impl GMTextT for GMStaticText {
    fn draw(&self) {
        let mut current_x = self.x;

        for c in self.data.chars() {
            self.font.draw(c, current_x, self.y);
            let (char_width, _) = self.font.get_extend(c);
            current_x += char_width;
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
    fn set_font(&mut self, font: &Rc<dyn GMFontT>) {
        self.font = font.clone();
    }
    fn get_font(&self) -> &Rc<dyn GMFontT> {
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

pub(crate) struct GMArrow {
    pub(crate) text: GMStaticText,
    pub(crate) min_x: f32,
    pub(crate) max_x: f32,
    pub(crate) step: f32,
}

impl GMArrow {
    fn new(arrow: &str, base: &Box<dyn GMTextT>) -> Self {
        let font = base.get_font();
        let base_y = base.get_y();

        let text = GMStaticText::new(arrow, 0.0, base_y, font);

        let mut result = Self {
            text,
            min_x: 0.0,
            max_x: 0.0,
            step: 0.0,
        };

        result.set_x(base);

        result
    }
    fn set_y(&mut self, y: f32) {
        self.text.set_y(y);
    }
    fn set_x(&mut self, base: &Box<dyn GMTextT>) {
        let base_x = base.get_x();

        match self.text.data.as_str() {
            "->" => {
                let (width, _) = self.text.get_extend();
                self.min_x = base_x - width - 50.0;
                self.max_x = base_x - width - 10.0;
                self.text.set_x(self.min_x);
                self.step = 2.0;
            }
            "<-" => {
                let (width, _) = base.get_extend();
                self.min_x = base_x + width + 10.0;
                self.max_x = base_x + width + 50.0;
                self.text.set_x(self.max_x);
                self.step = -2.0;
            }
            _ => {

            }
        }
    }
    fn change_all(&mut self, base: &Box<dyn GMTextT>) {
        self.text.set_font(base.get_font());
        self.set_x(base);
        self.set_y(base.get_y());
    }
    fn draw(&self) {
        self.text.draw();
    }
    fn update(&mut self) {
        if self.step > 0.0 {
            self.text.x += self.step;
            if self.text.x > self.max_x {
                self.text.x = self.max_x;
                self.step = -self.step;
            }
        } else {
            self.text.x += self.step;
            if self.text.x < self.min_x {
                self.text.x = self.min_x;
                self.step = -self.step;
            }
        }
    }
}

pub struct GMArrowText {
    pub(crate) base: Box<dyn GMTextT>,
    pub(crate) left_arrow: GMArrow,
    pub(crate) right_arrow: GMArrow,
}

impl GMArrowText {
    pub fn new(base: Box<dyn GMTextT>) -> Self {
        let left_arrow = GMArrow::new("->", &base);
        let right_arrow = GMArrow::new("<-", &base);

        Self {
            base,
            left_arrow,
            right_arrow,
        }
    }
    pub fn new_box(base: Box<dyn GMTextT>) -> Box<dyn GMTextT> {
        Box::new(Self::new(base))
    }
}

impl GMTextT for GMArrowText {
    fn draw(&self) {
        self.base.draw();
        self.left_arrow.draw();
        self.right_arrow.draw();
    }
    fn update(&mut self) {
        self.base.update();
        self.left_arrow.update();
        self.right_arrow.update();
    }
    fn set_text(&mut self, text: &str) {
        self.base.set_text(&text);
        self.right_arrow.set_x(&self.base);
    }
    fn get_text(&self) -> &str {
        self.base.get_text()
    }
    fn set_x(&mut self, x: f32) {
        self.base.set_x(x);
        self.left_arrow.set_x(&self.base);
        self.right_arrow.set_x(&self.base);
    }
    fn get_x(&self) -> f32 {
        self.base.get_x()
    }
    fn set_y(&mut self, y: f32) {
        self.base.set_y(y);
        self.right_arrow.set_y(y);
        self.left_arrow.set_y(y);
    }
    fn get_y(&self) -> f32 {
        self.base.get_y()
    }
    fn set_font(&mut self, font: &Rc<dyn GMFontT>) {
        self.base.set_font(font);
        self.left_arrow.change_all(&self.base);
        self.right_arrow.change_all(&self.base);
    }
    fn get_font(&self) -> &Rc<dyn GMFontT> {
        self.base.get_font()
    }
    fn from_other(&mut self, other: Box<dyn GMTextT>) {
        self.base.from_other(other);
        self.left_arrow.change_all(&self.base);
        self.right_arrow.change_all(&self.base);
    }
    fn get_extend(&self) -> (f32, f32) {
        self.base.get_extend()
    }
}

pub struct GMSpriteText {
    pub(crate) base: Box<dyn GMTextT>,
    pub(crate) left_sprite: GMSprite,
    pub(crate) right_sprite: GMSprite,
}

impl GMSpriteText {
    pub fn new(base: Box<dyn GMTextT>, sprite: GMSprite) -> Self {
        let left_sprite = sprite.clone_sprite();
        let right_sprite = sprite;

        let mut result = Self {
            base,
            left_sprite,
            right_sprite,
        };

        result.change_x(result.base.get_x());
        result.change_y(result.base.get_y());

        result
    }
    pub fn new_box(base: Box<dyn GMTextT>, sprite: GMSprite) -> Box<dyn GMTextT> {
        Box::new(Self::new(base, sprite))
    }
    fn change_x(&mut self, x: f32) {
        let (text_width, _) = self.base.get_extend();
        let (sprite_width, _) = self.left_sprite.get_extend();
        let left_x = x - 10.0 - sprite_width;
        let right_x = x + text_width + 10.0;
        self.left_sprite.set_x(left_x);
        self.right_sprite.set_x(right_x);
    }
    fn change_y(&mut self, y: f32) {
        let (_, text_height) = self.base.get_extend();
        let (_, sprite_height) = self.left_sprite.get_extend();
        let new_y = y + (text_height / 2.0) - (sprite_height / 2.0);
        self.left_sprite.set_y(new_y);
        self.right_sprite.set_y(new_y);
    }
}

impl GMTextT for GMSpriteText {
    fn draw(&self) {
        self.base.draw();
        self.left_sprite.draw();
        self.right_sprite.draw();
    }
    fn update(&mut self) {
        self.base.update();
        self.left_sprite.update();
        self.right_sprite.update();
    }

    fn set_text(&mut self, text: &str) {
        self.base.set_text(text);
        self.change_x(self.base.get_x());
    }
    fn get_text(&self) -> &str {
        self.base.get_text()
    }
    fn set_x(&mut self, x: f32) {
        self.base.set_x(x);
        self.change_x(x);
    }
    fn get_x(&self) -> f32 {
        self.base.get_x()
    }
    fn set_y(&mut self, y: f32) {
        self.base.set_y(y);
        self.change_y(y);
    }
    fn get_y(&self) -> f32 {
        self.base.get_y()
    }
    fn set_font(&mut self, font: &Rc<dyn GMFontT>) {
        self.base.set_font(font);
        self.change_x(self.base.get_x());
        self.change_y(self.base.get_y());
    }
    fn get_font(&self) -> &Rc<dyn GMFontT> {
        self.base.get_font()
    }
    fn from_other(&mut self, other: Box<dyn GMTextT>) {
        self.base.from_other(other);
        self.change_x(self.base.get_x());
        self.change_y(self.base.get_y());
    }
    fn get_extend(&self) -> (f32, f32) {
        self.base.get_extend()
    }
}
