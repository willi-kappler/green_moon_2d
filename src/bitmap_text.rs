

use std::collections::{HashMap};
use std::sync::Arc;
use std::fmt::Debug;

use log::debug;

use crate::texture::{GMTexture, GMTextureT};
use crate::util::{error_panic, GMAlign, GMDrawT, GMUpdateT, GMVisibleT, GMFlipXYT, GMSizeT};
use crate::math::{GMVec2D, GMSize, GMFlipXY};
use crate::context::GMContext;
use crate::movement::{GMPositionT, GMRotationT, GMScaleT};

use crate::{gen_impl_position,
    gen_impl_rotation, gen_impl_scale, gen_impl_flipxy, gen_impl_visible,
    gen_impl_texture, gen_impl_size};

#[derive(Debug, Clone)]
pub struct GMBitmapFont {
    texture: Arc<GMTexture>,
    mapping: HashMap<char, u32>,
}

impl GMBitmapFont {
    pub fn new(texture: Arc<GMTexture>, char_mapping: &str) -> Self {
        debug!("GMBitmapFont::new(), char_mapping: '{}'", char_mapping);
        // Maybe split texture into smaller char sized textures...

        let mut mapping = HashMap::new();

        for (i, c) in char_mapping.chars().enumerate() {
            mapping.insert(c, i as u32);
        }

        Self {
            texture: texture,
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

    pub fn set_mapping(&mut self, mapping: HashMap<char, u32>) {
        self.mapping = mapping;
    }

    pub fn get_mapping(&self) -> &HashMap<char, u32> {
        &self.mapping
    }
}

gen_impl_texture!(GMBitmapFont);

#[derive(Debug, Clone)]
pub struct GMBitmapChar {
    index: u32,
    position: GMVec2D,
    angle: f32,
    scale: f32,
    flip_xy: GMFlipXY,
    visible: bool,
    // TODO: alpha value
}

impl GMBitmapChar {
    pub fn new(index: u32, position: GMVec2D) -> Self {
        Self {
            index,
            position,
            angle: 0.0,
            scale: 1.0,
            flip_xy: GMFlipXY::new(false, false),
            visible: true,
        }
    }

    pub fn set_index(&mut self, index: u32) {
        self.index = index;
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }
}

gen_impl_position!(GMBitmapChar);

gen_impl_rotation!(GMBitmapChar);

gen_impl_scale!(GMBitmapChar);

gen_impl_flipxy!(GMBitmapChar);

gen_impl_visible!(GMBitmapChar);

#[derive(Debug, Clone)]
pub struct GMBitmapText {
    font: Arc<GMBitmapFont>,
    position: GMVec2D,
    text: String,
    spacing: GMVec2D,
    horizontal: bool,
    align: GMAlign,
    size: GMSize,
    chars: Vec<GMBitmapChar>,
    visible: bool,
}

impl GMBitmapText {
    pub fn new<T: Into<GMVec2D>, S: Into<String>>(font: &Arc<GMBitmapFont>, position: T, text: S) -> Self {
        let position = position.into();
        let text = text.into();

        debug!("GMBitmapText::new(), position: '{}', text: '{}'", position, text);

        let mut text = Self {
            font: font.clone(),
            position,
            text,
            spacing: GMVec2D::new(0.0, 0.0),
            horizontal: true,
            align: GMAlign::BottomLeft,
            size: GMSize::new(0.0, 0.0),
            chars: Vec::new(),
            visible: true,
        };

        text.reset_chars();

        text
    }

    pub fn set_font(&mut self, font: &Arc<GMBitmapFont>) {
        self.font = font.clone();
    }

    pub fn set_font2(&mut self, font: &Arc<GMBitmapFont>) {
        self.set_font(font);
        self.reset_chars();
    }

    pub fn get_font(&self) -> &Arc<GMBitmapFont> {
        &self.font
    }

    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
    }

    pub fn set_text2<T: Into<String>>(&mut self, text: T) {
        self.set_text(text);
        self.reset_chars();
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_spacing<T: Into<GMVec2D>>(&mut self, spacing: T) {
        self.spacing = spacing.into();
    }

    pub fn set_spacing2<T: Into<GMVec2D>>(&mut self, spacing: T) {
        self.set_spacing(spacing);
        self.reset_positions();
    }

    pub fn get_spacing(&self) -> GMVec2D{
        self.spacing
    }

    pub fn set_horizontal(&mut self, horizontal: bool) {
        self.horizontal = horizontal;
    }

    pub fn set_horizontal2(&mut self, horizontal: bool) {
        self.set_horizontal(horizontal);
        self.reset_positions();
    }

    pub fn get_horizontal(&self) -> bool {
        self.horizontal
    }

    pub fn toggle_horizontal(&mut self) {
        self.horizontal = !self.horizontal;
    }

    pub fn toggle_horizontal2(&mut self) {
        self.toggle_horizontal();
        self.reset_positions();
    }

    pub fn set_align(&mut self, align: GMAlign) {
        self.align = align;
    }

    pub fn set_align2(&mut self, align: GMAlign) {
        self.set_align(align);
        self.reset_positions();
    }

    pub fn get_align(&self) -> GMAlign {
        self.align
    }

    pub fn get_size(&self) -> GMSize {
        self.size
    }

    pub fn get_chars(&self) -> &Vec<GMBitmapChar> {
        &self.chars
    }

    pub fn get_chars_mut(&mut self) -> &mut Vec<GMBitmapChar> {
        &mut self.chars
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

        self.reset_positions();
    }

    pub fn reset_positions(&mut self) {
        // Keep characters, just change position
        let (dx, dy) = self.font.get_char_dimensions();
        let num_of_chars = self.chars.len() as f32;
        let mut x = 0.0;
        let mut y = 0.0;
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
            }
            GMAlign::TopCenter => {
                x = -(self.size.width / 2.0);
            }
            GMAlign::TopRight => {
                x = -self.size.width;
            }
            GMAlign::MiddleLeft => {
                y = -(self.size.height / 2.0);
            }
            GMAlign::MiddleCenter => {
                x = -(self.size.width / 2.0);
                y = -(self.size.height / 2.0);
            }
            GMAlign::MiddleRight => {
                x = -self.size.width;
                y = -(self.size.height / 2.0);
            }
            GMAlign::BottomLeft => {
                y = -self.size.height;
            }
            GMAlign::BottomCenter => {
                x = -(self.size.width / 2.0);
                y = -self.size.height;
            }
            GMAlign::BottomRight => {
                x = -self.size.width;
                y = -self.size.height;
            }
        }

        for c in self.chars.iter_mut() {
            c.position.x = x;
            c.position.y = y;
            c.angle = 0.0;

            x += dx2;
            y += dy2;
        }
    }
}

impl GMDrawT for GMBitmapText {
    fn draw(&self, context: &mut GMContext) {
        if self.visible {
            for c in self.chars.iter() {
                if c.visible {
                    let dx = self.position.x + c.position.x;
                    let dy = self.position.y + c.position.y;
                    let flip_x = c.flip_xy.flip_x;
                    let flip_y = c.flip_xy.flip_y;

                    self.font.texture.draw_opt(dx, dy, c.index, c.angle, c.scale, flip_x, flip_y, context);
                }
            }
        }
    }
}

impl GMUpdateT for GMBitmapText {}

gen_impl_position!(GMBitmapText);

gen_impl_visible!(GMBitmapText);

gen_impl_size!(GMBitmapText);
