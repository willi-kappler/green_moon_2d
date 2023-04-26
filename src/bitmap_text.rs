

use std::collections::{HashMap};
use std::rc::Rc;
use std::fmt::Debug;

use log::debug;

use crate::texture::{GMTexture};
use crate::util::{error_panic, GMAlign};
use crate::math::{GMVec2D, GMSize};
use crate::context::GMContext;
use crate::object::{GMObjectT, GMMessage, GMValue, GMObjectManager};


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
            GMMessage::GetChildCount => {
                return GMValue::USize(self.chars.len())
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
            GMMessage::Tuple2(m1, m2) => {
                return self.send_tuple2_message(*m1, *m2, context, object_manager)
            }
            GMMessage::Tuple3(m1, m2, m3) => {
                return self.send_tuple3_message(*m1, *m2, *m3, context, object_manager)
            }
            GMMessage::Tuple4(m1, m2, m3, m4) => {
                return self.send_tuple4_message(*m1, *m2, *m3, *m4, context, object_manager)
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            GMMessage::Custom1(name) if name == "reset_chars" => {
                self.reset_chars();
            }
            GMMessage::Custom1(name) if name == "reset_position" => {
                self.reset_positions();
            }
            GMMessage::Custom1(name) if name == "toggle_horizontal" => {
                self.horizontal = !self.horizontal;
            }
            GMMessage::Custom1(name) if name == "get_align" => {
                // let value = Box::new(GMValue::Any(Rc::new(self.align)));
                // return GMValue::Custom2("align".to_string(), value)
                let value = GMValue::Any(Rc::new(self.align));
                return ("align", value).into()
            }
            GMMessage::Custom1(name) if name == "get_font" => {
                // let value = Box::new(GMValue::Any(self.font.clone()));
                // return GMValue::Custom2("font".to_string(), value)
                let value = GMValue::Any(self.font.clone());
                return ("font", value).into()
            }
            GMMessage::Custom1(name) if name == "get_horizontal" => {
                // let value = Box::new(GMValue::Bool(self.horizontal));
                // return GMValue::Custom2("horizontal".to_string(), value)
                let value = self.horizontal.into();
                return ("horizontal", value).into()
            }
            GMMessage::Custom1(name) if name == "get_spacing" => {
                // let value = Box::new(GMValue::Vec2D(self.spacing));
                // return GMValue::Custom2("spacing".to_string(), value)
                let value = self.spacing.into();
                return ("spacing", value).into()
            }
            GMMessage::Custom1(name) if name == "get_spacing_x" => {
                // let value = Box::new(GMValue::F32(self.spacing.x));
                // return GMValue::Custom2("spacing_x".to_string(), value)
                let value = self.spacing.x.into();
                return ("spacing_x", value).into()
            }
            GMMessage::Custom1(name) if name == "get_spacing_y" => {
                // let value = Box::new(GMValue::F32(self.spacing.y));
                // return GMValue::Custom2("spacing_y".to_string(), value)
                let value = self.spacing.y.into();
                return ("spacing_y", value).into()
            }
            GMMessage::Custom1(name) if name == "get_text" => {
                // let value = Box::new(GMValue::String(self.text.clone()));
                // return GMValue::Custom2("text".to_string(), value)
                let value = self.text.clone().into();
                return ("text", value).into()
            }
            GMMessage::Custom2(name, GMValue::Vec2D(value)) if name == "add_spacing" => {
                self.spacing += value;
            }
            GMMessage::Custom2(name, GMValue::F32(value)) if name == "add_spacing_x" => {
                self.spacing.x += value;
            }
            GMMessage::Custom2(name, GMValue::F32(value)) if name == "add_spacing_y" => {
                self.spacing.y += value;
            }
            GMMessage::Custom2(name, GMValue::Any(value)) if name == "set_align" => {
                let align = value.downcast::<GMAlign>().unwrap();
                self.align = *align;
            }
            GMMessage::Custom2(name, GMValue::Any(value)) if name == "set_font" => {
                let font = value.downcast::<GMBitmapFont>().unwrap();
                self.font = font;
            }
            GMMessage::Custom2(name, GMValue::String(value)) if name == "set_font_name" => {
                let font = context.resources.get_font(&value);
                self.font = font.clone();
            }
            GMMessage::Custom2(name, GMValue::Bool(value)) if name == "set_horizontal" => {
                self.horizontal = value;
            }
            GMMessage::Custom2(name, GMValue::Vec2D(value)) if name == "set_spacing" => {
                self.spacing = value;
            }
            GMMessage::Custom2(name, GMValue::F32(value)) if name == "set_spacing_x" => {
                self.spacing.x = value;
            }
            GMMessage::Custom2(name, GMValue::F32(value)) if name == "set_spacing_y" => {
                self.spacing.y = value;
            }
            GMMessage::Custom2(name, GMValue::String(value)) if name == "set_text" => {
                self.text = value;
            }
            // Messages for character manipulation:
            GMMessage::Custom2(name, GMValue::Multiple(values)) if name == "add_chars_position" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::Vec2D(position) = values[i] {
                        c.position += position;
                    }
                }
            }
            GMMessage::Custom2(name, GMValue::Multiple(values)) if name == "set_chars_position" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::Vec2D(position) = values[i] {
                        c.position = position;
                    }
                }
            }
            GMMessage::Custom2(name, GMValue::Multiple(values)) if name == "add_chars_x" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(x) = values[i] {
                        c.position.x += x;
                    }
                }
            }
            GMMessage::Custom2(name, GMValue::Multiple(values)) if name == "add_chars_y" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(y) = values[i] {
                        c.position.y += y;
                    }
                }
            }
            GMMessage::Custom2(name, GMValue::Multiple(values)) if name == "set_chars_x" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(x) = values[i] {
                        c.position.x = x;
                    }
                }
            }
            GMMessage::Custom2(name, GMValue::Multiple(values)) if name == "set_chars_y" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(y) = values[i] {
                        c.position.y = y;
                    }
                }
            }
            GMMessage::Custom2(name, GMValue::Multiple(values)) if name == "set_chars_angle" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(angle) = values[i] {
                        c.angle = angle;
                    }
                }
            }
            GMMessage::Custom2(name, GMValue::Multiple(values)) if name == "set_chars_scale" => {
                for (i, c) in self.chars.iter_mut().enumerate() {
                    if let GMValue::F32(scale) = values[i] {
                        c.scale = scale;
                    }
                }
            }
            _ => {
                error_panic(&format!("Wrong message for GMBitmapText::send_message: {:?}", message))
            }
        }

        GMValue::None
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
