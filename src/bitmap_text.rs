

use std::collections::{HashMap};
use std::rc::Rc;
use std::fmt::Debug;

use log::debug;

use crate::texture::{GMTexture};
use crate::util::{error_panic, GMAlign};
use crate::math::{GMVec2D, GMSize};
use crate::context::GMContext;
use crate::object::{GMObjectT, GMMessage, GMValue, GMSetProperty, GMGetProperty, GMObjectManager};


#[derive(Debug, Clone)]
pub struct GMBitmapFont {
    mapping: HashMap<char, u32>,
    texture: Rc<GMTexture>,
}

impl GMBitmapFont {
    pub fn new(texture: Rc<GMTexture>, char_mapping: &str) -> Self {
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
}

#[derive(Debug, Clone)]
pub struct GMBitmapChar {
    pub angle: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub index: u32,
    pub position: GMVec2D,
    pub scale: f32,
    // TODO: alpha value
}

impl GMBitmapChar {
    pub fn new(index: u32, position: GMVec2D) -> Self {
        Self {
            angle: 0.0,
            flip_x: false,
            flip_y: false,
            index,
            position,
            scale: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMBitmapText {
    pub align: GMAlign,
    pub chars: Vec<GMBitmapChar>,
    pub font: Rc<GMBitmapFont>,
    pub horizontal: bool,
    pub position: GMVec2D,
    pub spacing: GMVec2D,
    pub text: String,
    size: GMSize,
}

impl GMBitmapText {
    pub fn new<T: Into<GMVec2D>, S: Into<String>>(font: &Rc<GMBitmapFont>, position: T, text: S) -> Self {
        let position = position.into();
        let text = text.into();

        debug!("GMBitmapText::new(), position: '{}', text: '{}'", position, text);

        let mut text = Self {
            align: GMAlign::BottomLeft,
            chars: Vec::new(),
            font: font.clone(),
            horizontal: true,
            position,
            size: GMSize::new(0.0, 0.0),
            spacing: GMVec2D::new(0.0, 0.0),
            text,
        };

        text.reset_chars();

        text
    }


    pub fn get_size(&self) -> GMSize {
        self.size
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

impl GMObjectT for GMBitmapText {
    fn send_message(&mut self, message: GMMessage, _context: &mut GMContext, _object_manager: &GMObjectManager) -> Option<GMValue> {

        match message {
            GMMessage::AddPosition(vec) => {
                self.position += vec;
            }
            GMMessage::AddSpacing(vec) => {
                self.spacing += vec;
            }
            GMMessage::AddSpacingX(x) => {
                self.spacing.x += x;
            }
            GMMessage::AddSpacingXY(x, y) => {
                self.spacing.x += x;
                self.spacing.y += y;
            }
            GMMessage::AddSpacingY(y) => {
                self.spacing.y += y;
            }
            GMMessage::AddX(x) => {
                self.position.x += x;
            }
            GMMessage::AddXY(x, y) => {
                self.position.x += x;
                self.position.y += y;
            }
            GMMessage::AddY(y) => {
                self.position.y += y;
            }
            GMMessage::ResetChars => {
                self.reset_chars();
            }

            GMMessage::ResetPosition => {
                self.reset_positions();
            }
            GMMessage::ToggleHorizontal => {
                self.horizontal = !self.horizontal;
            }
            _ => {
                error_panic(&format!("Wrong message for GMBitmapText::send_message: {:?}", message))
            }
        }

        None
    }

    fn set_property(&mut self, property: GMSetProperty) {
        match property {
            GMSetProperty::Align(align) => {
                self.align = align;
            }
            GMSetProperty::Font(font) => {
                self.font = font;
            }
            GMSetProperty::Horizontal(horizontal) => {
                self.horizontal = horizontal;
            }
            GMSetProperty::Position(vec) => {
                self.position = vec;
            }
            GMSetProperty::Spacing(spacing) => {
                self.spacing = spacing;
            }
            GMSetProperty::SpacingX(x) => {
                self.spacing.x = x;
            }
            GMSetProperty::SpacingXY(x, y) => {
                self.spacing.x = x;
                self.spacing.y = y;
            }
            GMSetProperty::SpacingY(y) => {
                self.spacing.y = y;
            }
            GMSetProperty::Text(text) => {
                self.text = text;
            }
            GMSetProperty::X(x) => {
                self.position.x = x;
            }
            GMSetProperty::XY(x, y) => {
                self.position.x = x;
                self.position.y = y;
            }
            GMSetProperty::Y(y) => {
                self.position.y = y;
            }
            _ => {
                error_panic(&format!("Wrong property for GMBitmapText::set_property: {:?}", property))
            }
        }
    }

    fn get_property(&self, property: &GMGetProperty) -> GMValue {
        match property {
            GMGetProperty::Align => {
                GMValue::Align(self.align)
            }
            GMGetProperty::Font => {
                GMValue::Font(self.font.clone())
            }
            GMGetProperty::Horizontal => {
                GMValue::Bool(self.horizontal)
            }
            GMGetProperty::NumElements => {
                GMValue::USize(self.chars.len())
            }
            GMGetProperty::Position => {
                GMValue::Vec2D(self.position)
            }
            GMGetProperty::Size => {
                GMValue::Size(self.size)
            }
            GMGetProperty::Spacing => {
                GMValue::Vec2D(self.spacing)
            }
            GMGetProperty::SpacingX => {
                GMValue::F32(self.spacing.x)
            }
            GMGetProperty::SpacingXY => {
                GMValue::F32F32(self.spacing.x, self.spacing.y)
            }
            GMGetProperty::SpacingY => {
                GMValue::F32(self.spacing.y)
            }
            GMGetProperty::Text => {
                GMValue::String(self.text.clone())
            }
            GMGetProperty::X => {
                GMValue::F32(self.position.x)
            }
            GMGetProperty::XY => {
                GMValue::F32F32(self.position.x, self.position.y)
            }
            GMGetProperty::Y => {
                GMValue::F32(self.position.y)
            }
            _ => {
                error_panic(&format!("Wrong property for GMBitmapText::get_property: {:?}", property))
            }
        }
    }

    fn draw(&self, context: &mut GMContext) {
        for c in self.chars.iter() {
            let dx = self.position.x + c.position.x;
            let dy = self.position.y + c.position.y;

            self.font.texture.draw_opt(dx, dy, c.index, c.angle, c.scale, c.flip_x, c.flip_y, context);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
