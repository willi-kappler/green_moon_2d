use crate::font::{GMFontT};
use crate::resources::GMResourceManager;
use crate::sprite::GMSprite;
use crate::behavior::GMKeyValue;

use std::rc::Rc;
use std::f32::consts;

// TODO:
// - Text with sprite / tile border


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
    fn set_property(&mut self, _data: &GMKeyValue) {
    }
}

pub struct GMTextStatic {
    data: String,
    x: f32,
    y: f32,
    font: Rc<dyn GMFontT>,
}

impl GMTextStatic {
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

impl GMTextT for GMTextStatic {
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
    text: GMTextStatic,
    min_x: f32,
    max_x: f32,
    step: f32,
}

impl GMArrow {
    fn new(arrow: &str, base: &Box<dyn GMTextT>) -> Self {
        let font = base.get_font();
        let base_y = base.get_y();

        let text = GMTextStatic::new(arrow, 0.0, base_y, font);

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

pub struct GMTextArrow {
    base: Box<dyn GMTextT>,
    left_arrow: GMArrow,
    right_arrow: GMArrow,
}

impl GMTextArrow {
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
    pub fn new_static(text: &str, x: f32, y: f32, font: &Rc<dyn GMFontT>) -> Box<dyn GMTextT> {
        let base = GMTextStatic::new_box(text, x, y, font);
        Self::new_box(base)
    }
}

impl GMTextT for GMTextArrow {
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

pub struct GMTextSprite {
    base: Box<dyn GMTextT>,
    left_sprite: GMSprite,
    right_sprite: GMSprite,
}

impl GMTextSprite {
    pub fn new(base: Box<dyn GMTextT>, sprite: &GMSprite) -> Self {
        let left_sprite = sprite.clone();
        let right_sprite = sprite.clone();

        let mut result = Self {
            base,
            left_sprite,
            right_sprite,
        };

        result.right_sprite.flip_x(true);

        result.change_x(result.base.get_x());
        result.change_y(result.base.get_y());

        result.left_sprite.start_animation();
        result.right_sprite.start_animation();

        result
    }
    pub fn new_box(base: Box<dyn GMTextT>, sprite: &GMSprite) -> Box<dyn GMTextT> {
        Box::new(Self::new(base, sprite))
    }
    pub fn new_static(text: &str, x: f32, y: f32, font: &Rc<dyn GMFontT>, sprite: &GMSprite) -> Box<dyn GMTextT> {
        let base = GMTextStatic::new_box(text, x, y, font);
        Self::new_box(base, sprite)
    }
    pub fn new_from_resource(text: &str, x: f32, y: f32, resources: &GMResourceManager, font_name: &str, sprite_name: &str) -> Box<dyn GMTextT> {
        let font = resources.get_font(font_name).unwrap();
        let sprite = resources.get_sprite(sprite_name).unwrap();

        Self::new_static(text, x, y, &font, sprite)
    }
    pub fn set_sprite(&mut self, sprite: &GMSprite) {
        self.left_sprite = sprite.clone();
        self.right_sprite = sprite.clone();

        self.right_sprite.flip_x(true);

        self.left_sprite.start_animation();
        self.right_sprite.start_animation();

        self.change_x(self.base.get_x());
        self.change_y(self.base.get_y());
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

impl GMTextT for GMTextSprite {
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
    fn set_property(&mut self, data: &GMKeyValue) {
        if data.key == "sprite" {
            match data.value.downcast_ref::<GMSprite>() {
                Some(sprite) => {
                    self.set_sprite(&sprite);
                }
                None => {
                    eprintln!("GMTextSprite::set_property(), could not downcast value to GMSprite")
                }
            }
        } else {
            self.base.set_property(data)
        }
    }
}

pub struct GMTextWave {
    base: GMTextStatic,
    amplitude: f32,
    frequency: f32,
    offset: f32,
    time: f32,
}

impl GMTextWave {
    pub fn new(base: GMTextStatic, amplitude: f32, frequency: f32) -> Self {
        Self {
            base,
            amplitude,
            frequency,
            offset: 1.0,
            time: 0.0,
        }
    }
    pub fn new_box(base: GMTextStatic, amplitude: f32, frequency: f32) -> Box<dyn GMTextT> {
        let text = Self::new(base, amplitude, frequency);
        Box::new(text)
    }
    pub fn new_static(text: &str, x: f32, y: f32, font: &Rc<dyn GMFontT>, amplitude: f32, frequency: f32) -> Box<dyn GMTextT> {
        let base = GMTextStatic::new(text, x, y, font);
        Self::new_box(base, amplitude, frequency)
    }
    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }
    pub fn get_amplitude(&self) -> f32 {
        self.amplitude
    }
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }
    pub fn set_offset(&mut self, offset: f32) {
        self.offset = offset;
    }
    pub fn get_offset(&self) -> f32 {
        self.offset
    }
}

impl GMTextT for GMTextWave {
    fn draw(&self) {
        let mut current_x = self.base.x;
        let mut current_y: f32;
        let mut offset = 0.0;
        let mut value: f32;

        for c in self.base.data.chars() {
            value = offset + (self.frequency * self.time);
            current_y = self.base.y + (self.amplitude * value.sin());
            self.base.font.draw(c, current_x, current_y);
            let (char_width, _) = self.base.font.get_extend(c);
            current_x += char_width;
            offset += self.offset;
        }
    }
    fn update(&mut self) {
        self.time += 0.01;
        if self.time > consts::TAU {
            self.time -= consts::TAU;
        }
    }
    fn set_text(&mut self, text: &str) {
        self.base.set_text(text);
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
    fn set_font(&mut self, font: &Rc<dyn GMFontT>) {
        self.base.set_font(font);
    }
    fn get_font(&self) -> &Rc<dyn GMFontT> {
        self.base.get_font()
    }
    fn from_other(&mut self, other: Box<dyn GMTextT>) {
        self.base.from_other(other);
    }
    fn get_extend(&self) -> (f32, f32) {
        // TODO:: Calculate extend for y (= height) with amplitude
        self.base.get_extend()
    }
    fn set_property(&mut self, data: &GMKeyValue) {
        if data.key == "amplitude" {
            match data.value.downcast_ref::<f32>() {
                Some(a) => {
                    self.set_amplitude(*a);
                }
                None => {
                    eprintln!("GMTextWave::set_property(), '{}', could not downcast value to f32", data.key)
                }
            }
        } else if data.key == "frequency" {
            match data.value.downcast_ref::<f32>() {
                Some(f) => {
                    self.set_frequency(*f);
                }
                None => {
                    eprintln!("GMTextWave::set_property(), '{}', could not downcast value to f32", data.key)
                }
            }
        } else if data.key == "offset" {
            match data.value.downcast_ref::<f32>() {
                Some(f) => {
                    self.set_offset(*f);
                }
                None => {
                    eprintln!("GMTextWave::set_property(), '{}', could not downcast value to f32", data.key)
                }
            }
        }
        else {
            self.base.set_property(data)
        }
    }
}

