

use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefMut;
use std::fmt::Debug;

use log::debug;

use crate::texture::{GMTexture};
use crate::util::{error_panic, GMAlign};
use crate::math::{GMVec2D, GMSize};
use crate::context::GMContext;
use crate::object::GMObjectT;
use crate::value::GMValue;
use crate::object_manager::GMObjectManager;


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

    pub fn set_align(&mut self, align: GMAlign) {
        self.align = align;
        self.reset_positions();
    }

    pub fn set_font(&mut self, font: &Rc<GMBitmapFont>) {
        self.font = font.clone();
        self.reset_chars();
    }

    pub fn set_horizontal(&mut self, horizontal: bool) {
        self.horizontal = horizontal;
        self.reset_positions();
    }

    pub fn set_position(&mut self, position: GMVec2D) {
        self.position = position;
        self.reset_positions();
    }

    pub fn set_spacing(&mut self, spacing: GMVec2D) {
        self.spacing = spacing;
        self.reset_positions();
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.reset_chars();
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
    /*
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {

        match message {
            GMMessage::AddPosition(vec) => {
                self.position += vec;
            }
            GMMessage::AddX(x) => {
                self.position.x += x;
            }
            GMMessage::AddY(y) => {
                self.position.y += y;
            }
            GMMessage::SetPosition(vec) => {
                self.position = vec;
            }
            GMMessage::SetX(x) => {
                self.position.x = x;
            }
            GMMessage::SetY(y) => {
                self.position.y = y;
            }
            GMMessage::GetPosition => {
                return GMValue::Position(self.position)
            }
            GMMessage::GetSize => {
                return GMValue::Size(self.size)
            }
            GMMessage::GetX => {
                return GMValue::F32(self.position.x)
            }
            GMMessage::GetY => {
                return GMValue::F32(self.position.y)
            }
            GMMessage::Multiple(messages) => {
                return self.send_message_multiple(messages, context, object_manager)
            }
            GMMessage::Custom0(name) if name == "get_char_count" => {
                return GMValue::USize(self.chars.len())
            }
            GMMessage::Custom0(name) if name == "reset_chars" => {
                self.reset_chars();
            }
            GMMessage::Custom0(name) if name == "reset_positions" => {
                self.reset_positions();
            }
            GMMessage::Custom0(name) if name == "toggle_horizontal" => {
                self.horizontal = !self.horizontal;
            }
            GMMessage::Custom0(name) if name == "toggle_horizontal2" => {
                self.horizontal = !self.horizontal;
                self.reset_positions();
            }
            GMMessage::Custom0(name) if name == "get_align" => {
                let value = GMValue::Any(Rc::new(self.align));
                return value
            }
            GMMessage::Custom0(name) if name == "get_font" => {
                let value = GMValue::Any(self.font.clone());
                return value
            }
            GMMessage::Custom0(name) if name == "get_horizontal" => {
                let value = self.horizontal.into();
                return value
            }
            GMMessage::Custom0(name) if name == "get_spacing" => {
                let value = self.spacing.into();
                return value
            }
            GMMessage::Custom0(name) if name == "get_spacing_x" => {
                let value = self.spacing.x.into();
                return value
            }
            GMMessage::Custom0(name) if name == "get_spacing_y" => {
                let value = self.spacing.y.into();
                return value
            }
            GMMessage::Custom0(name) if name == "get_text" => {
                let value = self.text.clone().into();
                return value
            }
            GMMessage::Custom1(name, GMValue::Vec2D(value)) if name == "add_spacing" => {
                debug!("GMBitmapText::send_message(), add_spacing: '{}'", value);
                self.spacing += value;
            }
            GMMessage::Custom1(name, GMValue::Vec2D(value)) if name == "add_spacing2" => {
                debug!("GMBitmapText::send_message(), add_spacing: '{}'", value);
                self.spacing += value;
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == "add_spacing_x" => {
                debug!("GMBitmapText::send_message(), add_spacing_x: '{}'", value);
                self.spacing.x += value;
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == "add_spacing_x2" => {
                debug!("GMBitmapText::send_message(), add_spacing_x: '{}'", value);
                self.spacing.x += value;
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == "add_spacing_y" => {
                debug!("GMBitmapText::send_message(), add_spacing_y: '{}#", value);
                self.spacing.y += value;
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == "add_spacing_y2" => {
                debug!("GMBitmapText::send_message(), add_spacing_y: '{}'", value);
                self.spacing.y += value;
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_align" => {
                let align = value.downcast::<GMAlign>().unwrap();
                self.align = *align;
                debug!("GMBitmapText::send_message(), set_align: '{:?}'", self.align);
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_align2" => {
                let align = value.downcast::<GMAlign>().unwrap();
                self.align = *align;
                debug!("GMBitmapText::send_message(), set_align: '{:?}'", self.align);
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_font" => {
                debug!("GMBitmapText::send_message(), set_font");
                let font = value.downcast::<GMBitmapFont>().unwrap();
                self.font = font;
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_font2" => {
                debug!("GMBitmapText::send_message(), set_font2");
                let font = value.downcast::<GMBitmapFont>().unwrap();
                self.font = font;
                self.reset_chars()
            }
            GMMessage::Custom1(name, GMValue::String(value)) if name == "set_font" => {
                debug!("GMBitmapText::send_message(), set_font, font_name: '{}'", value);
                let font = context.resources.get_font(&value);
                self.font = font.clone();
            }
            GMMessage::Custom1(name, GMValue::String(value)) if name == "set_font2" => {
                debug!("GMBitmapText::send_message(), set_font2, font_name: '{}'", value);
                let font = context.resources.get_font(&value);
                self.font = font.clone();
                self.reset_chars()
            }
            GMMessage::Custom1(name, GMValue::Bool(value)) if name == "set_horizontal" => {
                debug!("GMBitmapText::send_message(), set_horizontal: '{}'", value);
                self.horizontal = value;
            }
            GMMessage::Custom1(name, GMValue::Bool(value)) if name == "set_horizontal2" => {
                debug!("GMBitmapText::send_message(), set_horizontal: '{}'", value);
                self.horizontal = value;
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::Vec2D(value)) if name == "set_spacing" => {
                debug!("GMBitmapText::send_message(), set_spacing: '{}'", value);
                self.spacing = value;
            }
            GMMessage::Custom1(name, GMValue::Vec2D(value)) if name == "set_spacing2" => {
                debug!("GMBitmapText::send_message(), set_spacing: '{}'", value);
                self.spacing = value;
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == "set_spacing_x" => {
                debug!("GMBitmapText::send_message(), set_spacing_x: '{}'", value);
                self.spacing.x = value;
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == "set_spacing_x2" => {
                debug!("GMBitmapText::send_message(), set_spacing_x: '{}'", value);
                self.spacing.x = value;
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == "set_spacing_y" => {
                debug!("GMBitmapText::send_message(), set_spacing_y: '{}'", value);
                self.spacing.y = value;
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == "set_spacing_y2" => {
                debug!("GMBitmapText::send_message(), set_spacing_y: '{}'", value);
                self.spacing.y = value;
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::String(value)) if name == "set_text" => {
                self.text = value;
            }
            GMMessage::Custom1(name, GMValue::String(value)) if name == "set_text2" => {
                self.text = value;
                self.reset_chars();
            }
            // Messages for character manipulation:
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "add_chars_position" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::Vec2D(position) = values[i] {
                        c.position += position;
                    }
                }
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "add_chars_position2" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::Vec2D(position) = values[i] {
                        c.position += position;
                    }
                }
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "set_chars_position" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::Vec2D(position) = values[i] {
                        c.position = position;
                    }
                }
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "set_chars_position2" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::Vec2D(position) = values[i] {
                        c.position = position;
                    }
                }
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "add_chars_x" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(x) = values[i] {
                        c.position.x += x;
                    }
                }
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "add_chars_x2" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(x) = values[i] {
                        c.position.x += x;
                    }
                }
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "add_chars_y" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(y) = values[i] {
                        c.position.y += y;
                    }
                }
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "add_chars_y2" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(y) = values[i] {
                        c.position.y += y;
                    }
                }
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "set_chars_x" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(x) = values[i] {
                        c.position.x = x;
                    }
                }
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "set_chars_x2" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(x) = values[i] {
                        c.position.x = x;
                    }
                }
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "set_chars_y" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(y) = values[i] {
                        c.position.y = y;
                    }
                }
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "set_chars_y2" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(y) = values[i] {
                        c.position.y = y;
                    }
                }
                self.reset_positions();
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "set_chars_angle" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(angle) = values[i] {
                        c.angle = angle;
                    }
                }
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "set_chars_scale" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(scale) = values[i] {
                        c.scale = scale;
                    }
                }
            }
            _ => {
                error_panic(&format!("Wrong message for GMBitmapText::send_message: '{:?}'", message))
            }
        }

        GMValue::None
    }
*/

    fn draw(&self, context: &mut RefMut<&mut GMContext>) {
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
