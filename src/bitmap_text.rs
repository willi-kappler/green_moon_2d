

use std::collections::{HashMap};
use std::rc::Rc;
use std::fmt::Debug;

use log::debug;
use hecs::Entity;



use crate::texture::GMTexture;
// use crate::context::GMContext;
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
}

#[derive(Debug, Clone)]
pub struct GMBitmapText {
    pub font: Rc<GMBitmapFont>,
    pub text: String,
    pub spacing: GMVec2D,
    pub horizontal: bool,
    pub align: GMAlign,
    pub size: GMSize,
    pub chars: Vec<Entity>,
}

impl GMBitmapText {
    pub fn new<S: Into<String>, T: Into<GMVec2D>>(font: &Rc<GMBitmapFont>, text: S, spacing: T, horizontal: bool, align: GMAlign) -> Self {
        Self {
            font: font.clone(),
            text: text.into(),
            spacing: spacing.into(),
            horizontal,
            align,
            size: GMSize::new(0.0, 0.0),
            chars: Vec::new(),
        }
    }
}

/*
#[derive(Debug, Clone)]
pub struct GMBitmapTextBase {
    pub font: Rc<GMBitmapFont>,
    pub text: String,

    pub position: GMVec2D,
    pub spacing: GMVec2D,
    pub horizontal: bool,
    pub size: GMSize,
    pub align: GMAlign,
    pub active: bool,
    pub visible: bool,
    pub name: String,
    pub groups: HashSet<String>,

    pub chars: Vec<GMBitmapChar>,
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
            active: true,
            visible: true,
            name: "".to_string(),
            groups: HashSet::new(),
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

    pub fn set_horizontal(&mut self, horizontal: bool) {
        if self.horizontal != horizontal {
            self.horizontal = horizontal;
            self.reset_chars2();
        }
    }

    pub fn set_align(&mut self, align: GMAlign) {
        if self.align != align {
            self.align = align;
            self.reset_chars2();
        }
    }
}

impl GMObjectBaseT for GMBitmapTextBase {
    fn draw(&self, context: &mut GMContext) {
        for bitmap_char in self.chars.iter() {
            let position = bitmap_char.position;
            self.font.draw_opt(bitmap_char.index, position.x, position.y, bitmap_char.angle, bitmap_char.scale,
                false, false, context);
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_font" => {
                todo!();
            }
            "set_font2" => {
                todo!();
            }
            "set_text" => {
                let text: String = data.into();
                self.set_text(text);
            }
            "set_text2" => {
                self.text = data.into();
            }
            "set_position" => {
                let position: GMVec2D = data.into();
                self.set_position(position);
            }
            "set_position2" => {
                self.position = data.into();
            }
            "set_spacing" => {
                let spacing: GMVec2D = data.into();
                self.set_spacing(spacing);
            }
            "set_spacing2" => {
                self.spacing = data.into();
            }
            "set_horizontal" => {
                let horizontal: bool = data.into();
                self.set_horizontal(horizontal);
            }
            "set_horizontal2" => {
                self.horizontal = data.into();
            }
            "set_align" => {
                todo!();
            }
            "set_align2" => {
                todo!();
            }
            "set_active" => {
                self.active = data.into();
            }
            "set_visible" => {
                self.visible = data.into();
            }
            "set_name" => {
                self.name = data.into();
            }
            "add_group" => {
                self.groups.insert(data.into());
            }
            "remove_group" => {
                let group: String = data.into();
                self.groups.remove(&group);
            }
            "clear_group" => {
                self.groups.clear();
            }
            _ => {
                error_panic(&format!("GMBitmapTextBase::send_message(), unknown message: '{}'", message))
            }
        }
    }

    return_name_and_groups!();
}

pub type GMBitmapText = GMObjectManager<GMBitmapTextBase>;

impl GMBitmapText {
    pub fn new(bitmap_font: &Rc<GMBitmapFont>) -> Self {
        Self {
            base: GMBitmapTextBase::new(bitmap_font),
            effects: GMEffectManager::new(),
        }
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

    create_builder_methods!(GMBitmapTextBuilder, GMBitmapTextBase, bitmap_text);

    pub fn build(mut self) -> GMBitmapText {
        self.bitmap_text.base.reset_chars();
        self.bitmap_text
    }
}

// TODO: Add GMTextBlock

// TODO: Add GMTextList
*/
