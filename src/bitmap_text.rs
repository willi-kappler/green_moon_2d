

use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Debug;

use log::debug;

use crate::bitmap_text_effects::GMTextEffectT;
use crate::data::GMData;
use crate::texture::GMTexture;
use crate::context::GMContext;
use crate::util::{error_panic, GMAlign};
use crate::math::{GMVec2D, GMSize};

#[derive(Debug, Clone)]
pub struct GMBitmapFont {
    texture: Rc<GMTexture>,
    mapping: HashMap<char, u32>,
}

impl GMBitmapFont {
    pub fn new(texture: &Rc<GMTexture>, char_mapping: &str) -> Self {
        debug!("GMBitmapFont::new(), char_mapping: '{}'", char_mapping);

        let mut mapping = HashMap::new();

        for (i, c) in char_mapping.chars().enumerate() {
            mapping.insert(c, i as u32);
        }

        Self {
            texture: texture.clone(),
            mapping,
        }
    }

    pub fn get_char_dimensions(&self) -> (f32, f32) {
        self.texture.get_unit_dimension()
    }

    pub fn get_index(&self, c: char) -> u32 {
        match self.mapping.get(&c) {
            Some(index) => {
                *index
            }
            None => {
                error_panic(&format!("GMBitmapFont::draw_opt(), Character '{}' not in map.", c));
            }
        }
    }

    pub fn draw(&self, index: u32, x: f32, y: f32, context: &mut GMContext) {
        self.draw_opt(index, x, y, 0.0, false, false, context);
    }

    pub fn draw_opt(&self, index: u32, x: f32, y: f32, angle: f32, flip_x: bool, flip_y: bool, context: &mut GMContext) {
        self.texture.draw_opt(x, y, index, angle, flip_x, flip_y, context);
    }
}

#[derive(Debug, Clone)]
pub struct GMBitmapChar {
    pub index: u32,
    pub position: GMVec2D,
    pub angle: f32,
    pub scale: f32,
}

impl GMBitmapChar {
    pub fn new(index: u32, position: GMVec2D) -> Self {
        Self {
            index,
            position,
            angle: 0.0,
            scale: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMBitmapTextBase {
    font: Rc<GMBitmapFont>,
    text: String,

    position: GMVec2D,
    spacing: GMVec2D,
    horizontal: bool,
    size: GMSize,
    align: GMAlign,

    chars: Vec<GMBitmapChar>,
}

impl GMBitmapTextBase {
    pub fn new(font: &Rc<GMBitmapFont>) -> Self {
        debug!("GMBitmapText::new()");

        Self {
            font: font.clone(),
            text: "".to_string(),
            position: GMVec2D::new(0.0, 0.0),
            spacing: GMVec2D::new(0.0, 0.0),
            horizontal: true,
            size: GMSize::new(0.0, 0.0),
            align: GMAlign::TopLeft,
            chars: Vec::new(),
        }
    }

    pub fn reset_chars(&mut self) {
        // Remove all the characters and recreate them
        self.chars.clear();

        for c in self.text.chars() {
            let index = self.font.get_index(c);
            let position = GMVec2D::new(0.0, 0.0);
            let bitmap_char = GMBitmapChar::new(index, position);
            self.chars.push(bitmap_char);
        }

        self.reset_chars2();
    }

    pub fn reset_chars2(&mut self) {
        // Keep characters, just change position
        let (dx, dy) = self.font.get_char_dimensions();
        let num_of_chars = self.chars.len() as f32;
        let mut x;
        let mut y;
        let mut dx2 = dx + self.spacing.x;
        let mut dy2 = dy + self.spacing.y;

        if self.horizontal {
            self.size.width = (dx * num_of_chars) + (self.spacing.x * (num_of_chars - 1.0));
            self.size.height = dy;
            dy2 = 0.0;
        } else {
            self.size.width = dx;
            self.size.height = (dy * num_of_chars) + (self.spacing.y * (num_of_chars - 1.0));
            dx2 = 0.0;
        }

        match self.align {
            GMAlign::TopLeft => {
                x = self.position.x;
                y = self.position.y;
            }
            GMAlign::TopCenter => {
                x = self.position.x - (self.size.width / 2.0);
                y = self.position.y;
            }
            GMAlign::TopRight => {
                x = self.position.x - self.size.width;
                y = self.position.y;
            }
            GMAlign::MiddleLeft => {
                x = self.position.x;
                y = self.position.y - (self.size.height / 2.0);
            }
            GMAlign::MiddleCenter => {
                x = self.position.x - (self.size.width / 2.0);
                y = self.position.y - (self.size.height / 2.0);
            }
            GMAlign::MiddleRight => {
                x = self.position.x - self.size.width;
                y = self.position.y - (self.size.height / 2.0);
            }
            GMAlign::BottomLeft => {
                x = self.position.x;
                y = self.position.y - self.size.height;
            }
            GMAlign::BottomCenter => {
                x = self.position.x - (self.size.width / 2.0);
                y = self.position.y - self.size.height;
            }
            GMAlign::BottomRight => {
                x = self.position.x - self.size.width;
                y = self.position.y - self.size.height;
            }
        }

        for bitmap_char in self.chars.iter_mut() {
            bitmap_char.position.x = x;
            bitmap_char.position.y = y;
            bitmap_char.angle = 0.0;

            x += dx2;
            y += dy2;
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        for bitmap_char in self.chars.iter() {
            let position = bitmap_char.position;
            self.font.draw_opt(bitmap_char.index, position.x, position.y, bitmap_char.angle, false, false, context);
        }
    }

    pub fn set_font(&mut self, font: &Rc<GMBitmapFont>) {
        // Even if the dimension of each char is equal, the mapping could be different.
        // So just reset all the characters
        self.font = font.clone();
        self.reset_chars();
    }

    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
        self.reset_chars();
    }

    pub fn set_position<T: Into<GMVec2D>>(&mut self, position: T) {
        self.position = position.into();
        self.reset_chars2();
    }

    pub fn set_position_x(&mut self, x: f32) {
        self.position.x = x;
        self.reset_chars2();
    }

    pub fn set_position_y(&mut self, y: f32) {
        self.position.y = y;
        self.reset_chars2();
    }

    pub fn add_position<T: Into<GMVec2D>>(&mut self, position: T) {
        self.position.add2(&position.into());
        self.reset_chars2();
    }

    pub fn add_position_x(&mut self, x: f32) {
        self.position.x += x;
        self.reset_chars2();
    }

    pub fn add_position_y(&mut self, y: f32) {
        self.position.y += y;
        self.reset_chars2();
    }

    pub fn get_position(&self) -> &GMVec2D {
        &self.position
    }

    pub fn set_spacing<T: Into<GMVec2D>>(&mut self, spacing: T) {
        self.spacing = spacing.into();
        self.reset_chars2();
    }

    pub fn set_spacing_x(&mut self, x: f32) {
        self.spacing.x = x;
        self.reset_chars2();
    }

    pub fn set_spacing_y(&mut self, y: f32) {
        self.spacing.y = y;
        self.reset_chars2();
    }

    pub fn add_spacing<T: Into<GMVec2D>>(&mut self, spacing: T) {
        self.spacing.add2(&spacing.into());
        self.reset_chars2();
    }

    pub fn add_spacing_x(&mut self, x: f32) {
        self.spacing.x += x;
        self.reset_chars2();
    }

    pub fn add_spacing_y(&mut self, y: f32) {
        self.spacing.y += y;
        self.reset_chars2();
    }

    pub fn get_spacing(&self) -> &GMVec2D {
        &self.spacing
    }

    pub fn set_horizontal(&mut self, horizontal: bool) {
        if self.horizontal != horizontal {
            self.horizontal = horizontal;
            self.reset_chars2();
        }
    }

    pub fn get_horizontal(&self) -> bool {
        self.horizontal
    }

    pub fn set_align(&mut self, align: GMAlign) {
        if self.align != align {
            self.align = align;
            self.reset_chars2();
        }
    }

    pub fn get_align(&self) -> &GMAlign {
        &self.align
    }

    pub fn get_chars(&self) -> &Vec<GMBitmapChar> {
        &self.chars
    }

    pub fn get_chars_mut(&mut self) -> &mut Vec<GMBitmapChar> {
        &mut self.chars
    }
}

#[derive(Debug, Clone)]
pub struct GMBitmapText {
    pub base: GMBitmapTextBase,
    effects: Vec<Box<dyn GMTextEffectT>>,
}

impl GMBitmapText {
    pub fn new(bitmap_font: &Rc<GMBitmapFont>) -> Self {
        Self {
            base: GMBitmapTextBase::new(bitmap_font),
            effects: Vec::new(),
        }
    }

    // Text methods
    pub fn update(&mut self, context: &mut GMContext) {
        for effect in self.effects.iter_mut() {
            effect.update(&mut self.base, context);
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        self.base.draw(context);

        for effect in self.effects.iter() {
            effect.draw(&self.base, context);
        }
    }

    // Text effect methods
    pub fn add_effect<T: 'static + GMTextEffectT>(&mut self, effect: T) {
        debug!("GMBitmapText::add_effect()");
        self.add_effect2(Box::new(effect));
    }

    pub fn add_effect2(&mut self, effect: Box<dyn GMTextEffectT>) {
        debug!("GMBitmapText::add_effect2()");
        self.effects.push(effect);
    }

    pub fn set_effects(&mut self, effects: Vec<Box<dyn GMTextEffectT>>) {
        debug!("GMBitmapText::set_effect()");
        self.effects = effects;
    }

    pub fn remove_effect(&mut self, index: usize) {
        debug!("GMBitmapText::remove_effect(), index: {}", index);
        self.effects.remove(index);
    }

    pub fn swap_effects(&mut self, index1: usize, index2: usize) {
        debug!("GMBitmapText::swap_effects(), index1: {}, index2: {}", index1, index2);
        self.effects.swap(index1, index2);
    }

    pub fn send_effect_message(&mut self, index: usize, message: &str, context: &mut GMContext) {
        self.effects[index].send_message(message, context)
    }

    pub fn send_effect_message_data(&mut self, index: usize, message: &str, data: GMData, context: &mut GMContext) {
        self.effects[index].send_message_data(message, data, context)
    }
}


pub struct GMBitmapTextBuilder {
    bitmap_text: GMBitmapText,
}

impl GMBitmapTextBuilder {
    pub fn new(bitmap_font: &Rc<GMBitmapFont>) -> Self {
        Self {
            bitmap_text: GMBitmapText::new(bitmap_font),
        }
    }

    pub fn with_text<T: Into<String>>(mut self, text: T) -> Self {
        let text = text.into();
        debug!("GMBitmapTextBuilder::with_text(), text: '{}'", text);

        self.bitmap_text.base.text = text;
        self
    }

    pub fn with_position<T: Into<GMVec2D>>(mut self, position: T) -> Self {
        let position = position.into();
        debug!("GMBitmapTextBuilder::with_position(), position: '{:?}'", position);

        self.bitmap_text.base.position = position;
        self
    }

    pub fn with_spacing<T: Into<GMVec2D>>(mut self, spacing: T) -> Self {
        let spacing = spacing.into();
        debug!("GMBitmapTextBuilder::with_spacing(), spacing: '{:?}'", spacing);

        self.bitmap_text.base.spacing = spacing;
        self
    }

    pub fn with_horizontal(mut self, horizontal: bool) -> Self {
        debug!("GMBitmapTextBuilder::with_horizontal(), horizontal: '{}'", horizontal);

        self.bitmap_text.base.horizontal = horizontal;
        self
    }

    pub fn with_align(mut self, align: GMAlign) -> Self {
        debug!("GMBitmapTextBuilder::with_align(), align: '{:?}'", align);

        self.bitmap_text.base.align = align;
        self
    }

    pub fn with_effect<T: 'static + GMTextEffectT>(mut self, effect: T) -> Self {
        debug!("GMBitmapTextBuilder::with_effect()");

        self.bitmap_text.effects.push(Box::new(effect));
        self
    }

    pub fn with_effect2(mut self, effect: Box<dyn GMTextEffectT>) -> Self {
        debug!("GMBitmapTextBuilder::with_effect2()");

        self.bitmap_text.effects.push(effect);
        self
    }

    pub fn with_effects(mut self, effects: Vec<Box<dyn GMTextEffectT>>) -> Self {
        debug!("GMBitmapTextBuilder::with_effects()");

        self.bitmap_text.effects.extend(effects);
        self
    }

    pub fn build(mut self) -> GMBitmapText {
        self.bitmap_text.base.reset_chars();
        self.bitmap_text
    }
}

// TODO: Add GMTextBlock

// TODO: Add GMTextList


