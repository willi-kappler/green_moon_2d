

use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Debug;

use log::debug;

use crate::texture::GMTexture;
use crate::context::GMContext;
use crate::util::{error_panic, GMAlign};

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
pub struct GMBitmapText {
    font: Rc<GMBitmapFont>,
    text: String,
    base_x: f32,
    base_y: f32,
    spacing_x: f32,
    spacing_y: f32,
    horizontal: bool,
    chars: Vec<(u32, f32, f32, f32)>, // index, x, y, angle
    width: f32,
    height: f32,
    align: GMAlign,
}

impl GMBitmapText {
    pub fn new(font: &Rc<GMBitmapFont>, text: &str, x: f32, y: f32) -> Self {
        debug!("GMBitmapText::new(), text: '{}', x: {}, y: {}", text, x, y);

        let mut text = Self {
            font: font.clone(),
            text: text.to_string(),
            base_x: x,
            base_y: y,
            spacing_x: 0.0,
            spacing_y: 0.0,
            horizontal: true,
            chars: Vec::new(),
            width: 0.0,
            height: 0.0,
            align: GMAlign::TopLeft,
        };

        text.reset_chars();

        text
    }

    pub fn new2(font: &str, text: &str, x: f32, y: f32, context: &GMContext) -> Self {
        debug!("GMBitmapText::new2(), font: {}, text: {}, x: {}, y: {}", font, text, x, y);

        Self::new(&context.get_resources().get_font_clone(font), text, x, y)
    }

    pub fn reset_chars(&mut self) {
        // Remove all the characters and recreate them
        self.chars.clear();

        for c in self.text.chars() {
            let index = self.font.get_index(c);
            self.chars.push((index, 0.0, 0.0, 0.0));
        }

        self.reset_chars2();
    }

    pub fn reset_chars2(&mut self) {
        // Keep characters, just change position
        let (dx, dy) = self.font.get_char_dimensions();
        let num_of_chars = self.chars.len() as f32;
        let mut x;
        let mut y;
        let mut dx2 = dx + self.spacing_x;
        let mut dy2 = dy + self.spacing_y;

        if self.horizontal {
            self.width = (dx * num_of_chars) + (self.spacing_x * (num_of_chars - 1.0));
            self.height = dy;
            dy2 = 0.0;
        } else {
            self.width = dx;
            self.height = (dy * num_of_chars) + (self.spacing_y * (num_of_chars - 1.0));
            dx2 = 0.0;
        }

        match self.align {
            GMAlign::TopLeft => {
                x = self.base_x;
                y = self.base_y;
            }
            GMAlign::TopCenter => {
                x = self.base_x - (self.width / 2.0);
                y = self.base_y;
            }
            GMAlign::TopRight => {
                x = self.base_x - self.width;
                y = self.base_y;
            }
            GMAlign::MiddleLeft => {
                x = self.base_x;
                y = self.base_y - (self.height / 2.0);
            }
            GMAlign::MiddleCenter => {
                x = self.base_x - (self.width / 2.0);
                y = self.base_y - (self.height / 2.0);
            }
            GMAlign::MiddleRight => {
                x = self.base_x - self.width;
                y = self.base_y - (self.height / 2.0);
            }
            GMAlign::BottomLeft => {
                x = self.base_x;
                y = self.base_y - self.height;
            }
            GMAlign::BottomCenter => {
                x = self.base_x - (self.width / 2.0);
                y = self.base_y - self.height;
            }
            GMAlign::BottomRight => {
                x = self.base_x - self.width;
                y = self.base_y - self.height;
            }
        }

        for (_, cx, cy, angle)in self.chars.iter_mut() {
            *cx = x;
            *cy = y;
            *angle = 0.0;

            x += dx2;
            y += dy2;
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        for (index, x, y, angle) in self.chars.iter() {
            self.font.draw_opt(*index, *x, *y, *angle, false, false, context);
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

    pub fn set_base_x(&mut self, base_x: f32) {
        if self.base_x != base_x {
            let dx = base_x - self.base_x;
            self.base_x = base_x;

            for char in self.chars.iter_mut() {
                char.1 += dx;
            }
        }
    }

    pub fn set_base_y(&mut self, base_y: f32) {
        if self.base_y != base_y {
            let dy = base_y - self.base_y;
            self.base_y = base_y;

            for char in self.chars.iter_mut() {
                char.2 += dy;
            }
        }
    }

    pub fn set_base_xy(&mut self, base_x: f32, base_y: f32) {
        self.set_base_x(base_x);
        self.set_base_y(base_y);
    }

    pub fn set_spacing_x(&mut self, spacing_x: f32) {
        if self.spacing_x != spacing_x {
            self.spacing_x = spacing_x;
            if self.horizontal {
                self.reset_chars2();
            }
        }
    }

    pub fn set_spacing_y(&mut self, spacing_y: f32) {
        if self.spacing_y != spacing_y {
            self.spacing_y = spacing_y;
            if !self.horizontal {
                self.reset_chars2();
            }
        }
    }

    pub fn set_spacing_xy(&mut self, spacing_x: f32, spacing_y: f32) {
        self.set_spacing_x(spacing_x);
        self.set_spacing_y(spacing_y);
    }

    pub fn set_horizontal(&mut self, horizontal: bool) {
        if self.horizontal != horizontal {
            self.horizontal = horizontal;
            self.reset_chars2();
        }
    }

    pub fn align(&mut self, align: GMAlign) {
        if self.align != align {
            self.align = align;
            self.reset_chars2();
        }
    }

    pub fn get_horizontal(&self) -> bool {
        self.horizontal
    }

    pub fn get_mut_chars(&mut self) -> &mut Vec<(u32, f32, f32, f32)> {
        &mut self.chars
    }

}


// TODO: Add GMTextBlock

// TODO: Add GMTextList


