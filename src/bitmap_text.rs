

use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Debug;
use std::any::Any;

use log::debug;

use crate::bitmap_text_effects::GMTextEffectT;
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
    pub fn new(texture: Rc<GMTexture>, char_mapping: &str) -> Self {
        debug!("GMBitmapFont::new(), char_mapping: '{}'", char_mapping);

        let mut mapping = HashMap::new();

        for (i, c) in char_mapping.chars().enumerate() {
            mapping.insert(c, i as u32);
        }

        Self {
            texture,
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
}

impl GMBitmapChar {
    pub fn new(index: u32, position: GMVec2D, angle: f32) -> Self {
        Self {
            index,
            position,
            angle
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
            let bitmap_char = GMBitmapChar::new(index, position, 0.0);
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

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
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

    pub fn get_mut_chars(&mut self) -> &mut Vec<GMBitmapChar> {
        &mut self.chars
    }

}

#[derive(Debug)]
pub struct GMBitmapText {
    base: GMBitmapTextBase,
    effects: Vec<Box<dyn GMTextEffectT>>,
}

impl GMBitmapText {
    pub fn new(bitmap_font: &Rc<GMBitmapFont>) -> Self {
        Self {
            base: GMBitmapTextBase::new(bitmap_font),
            effects: Vec::new(),
        }
    }

    // Builder pattern
    pub fn with_text(mut self, text: &str) -> Self {
        self.base.text = text.to_string();
        self
    }

    pub fn with_position<T: Into<GMVec2D>>(mut self, position: T) -> Self {
        self.base.position = position.into();
        self
    }

    pub fn with_spacing<T: Into<GMVec2D>>(mut self, spacing: T) -> Self {
        self.base.spacing = spacing.into();
        self
    }

    pub fn with_horizontal(mut self, horizontal: bool) -> Self {
        self.base.horizontal = horizontal;
        self
    }

    pub fn with_align(mut self, align: GMAlign) -> Self {
        self.base.align = align;
        self
    }

    pub fn build(mut self) -> Self {
        self.base.reset_chars();
        self
    }

    // Sprite methods
    pub fn update(&mut self, context: &mut GMContext) {
        for effect in self.effects.iter_mut() {
            effect.update(&mut self.base, context);
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        for effect in self.effects.iter() {
            effect.draw(&self.base, context);
        }
    }

    pub fn get_base(&self) -> &GMBitmapTextBase {
        &self.base
    }

    pub fn get_mut_base(&mut self) -> &mut GMBitmapTextBase {
        &mut self.base
    }

    // Sprite effect methods
    pub fn add_effect<T: 'static + GMTextEffectT>(&mut self, effect: T) {
        self.effects.push(Box::new(effect));
    }

    pub fn remove_effect(&mut self, index: usize) {
        self.effects.remove(index);
    }

    pub fn swap_effects(&mut self, index1: usize, index2: usize) {
        self.effects.swap(index1, index2);
    }

    pub fn send_effect_message(&mut self, index: usize, message: &str, context: &mut GMContext) {
        self.effects[index].send_message(message, context)
    }

    pub fn send_effect_message_data(&mut self, index: usize, message: &str, data: Box<dyn Any>, context: &mut GMContext) {
        self.effects[index].send_message_data(message, data, context)
    }
}


// TODO: Add GMTextBlock

// TODO: Add GMTextList


