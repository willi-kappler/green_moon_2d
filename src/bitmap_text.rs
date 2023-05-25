

use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefMut;
use std::fmt::Debug;

use log::debug;

use crate::texture::{GMTexture};
use crate::util::{error_panic, GMAlign, send_message_f32, send_message_bool};
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
    fn send_message(&mut self, mut message: crate::message::GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "get_char_count" => {
                        return self.chars.len().into();
                    }
                    "reset_chars" => {
                        self.reset_chars();
                    }
                    "reset_positions" => {
                        self.reset_positions();
                    }
                    "get_align" => {
                        return self.align.clone().into();
                    }
                    "set_align" => {
                        self.align = value.into_align();
                    }
                    "set_align2" => {
                        self.align = value.into_align();
                        self.reset_positions();
                    }
                    "get_font" => {
                        return self.font.clone().into();
                    }
                    "set_font" => {
                        self.font = value.into_font();
                    }
                    "set_font2" => {
                        self.font = value.into_font();
                        self.reset_chars();
                    }
                    "get_text" => {
                        return self.text.clone().into();
                    }
                    "set_text" => {
                        self.text = value.into_string();
                    }
                    "set_text2" => {
                        self.text = value.into_string();
                        self.reset_chars();
                    }
                    _ => {
                        error_panic(&format!("GMBitmapText::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "position" => {
                return self.position.send_message(method, value);
            }
            "position2" => {
                let result = self.position.send_message(method, value);

                if result.is_none() {
                    self.reset_positions();
                }

                return result;
            }
            "size" => {
                if method == "get" {
                    return self.size.into();
                } else {
                    error_panic(&format!("GMBitmapText::send_message, unknown method: '{}', tag: 'size'", method));
                }
            }
            "horizontal" => {
                return send_message_bool(&mut self.horizontal, method, value);
            }
            "horizontal2" => {
                let result = send_message_bool(&mut self.horizontal, method, value);

                if result.is_none() {
                    self.reset_positions();
                }

                return result
            }
            "spacing" => {
                return self.spacing.send_message(method, value);
            }
            "spacing2" => {
                let result =  self.spacing.send_message(method, value);

                if result.is_none() {
                    self.reset_positions();
                }

                return result
            }
            "chars" => {
                let mut values = value.to_vec_deque();

                match method {
                    "add_position" => {
                        for c in self.chars.iter_mut() {
                            let position = values.pop_front().unwrap().into_vec2d();
                            c.position += position;
                        }
                    }
                    "add_position2" => {
                        for c in self.chars.iter_mut() {
                            let position = values.pop_front().unwrap().into_vec2d();
                            c.position += position;
                        }

                        self.reset_positions();
                    }
                    "set_position" => {
                        for c in self.chars.iter_mut() {
                            let position = values.pop_front().unwrap().into_vec2d();
                            c.position = position;
                        }
                    }
                    "set_position2" => {
                        for c in self.chars.iter_mut() {
                            let position = values.pop_front().unwrap().into_vec2d();
                            c.position = position;
                        }

                        self.reset_positions();
                    }
                    "add_x" => {
                        for c in self.chars.iter_mut() {
                            let x = values.pop_front().unwrap().into_f32();
                            c.position.x += x;
                        }
                    }
                    "add_x2" => {
                        for c in self.chars.iter_mut() {
                            let x = values.pop_front().unwrap().into_f32();
                            c.position.x += x;
                        }

                        self.reset_positions();
                    }
                    "add_y" => {
                        for c in self.chars.iter_mut() {
                            let y = values.pop_front().unwrap().into_f32();
                            c.position.y += y;
                        }
                    }
                    "add_y2" => {
                        for c in self.chars.iter_mut() {
                            let y = values.pop_front().unwrap().into_f32();
                            c.position.y += y;
                        }

                        self.reset_positions();
                    }
                    "set_x" => {
                        for c in self.chars.iter_mut() {
                            let x = values.pop_front().unwrap().into_f32();
                            c.position.x = x;
                        }
                    }
                    "set_x2" => {
                        for c in self.chars.iter_mut() {
                            let x = values.pop_front().unwrap().into_f32();
                            c.position.x = x;
                        }

                        self.reset_positions();
                    }
                    "set_y" => {
                        for c in self.chars.iter_mut() {
                            let y = values.pop_front().unwrap().into_f32();
                            c.position.y = y;
                        }
                    }
                    "set_y2" => {
                        for c in self.chars.iter_mut() {
                            let y = values.pop_front().unwrap().into_f32();
                            c.position.y = y;
                        }

                        self.reset_positions();
                    }
                    "add_angle" => {
                        for c in self.chars.iter_mut() {
                            let angle = values.pop_front().unwrap().into_f32();
                            c.angle += angle;
                        }
                    }
                    "set_angle" => {
                        for c in self.chars.iter_mut() {
                            let angle = values.pop_front().unwrap().into_f32();
                            c.angle = angle;
                        }
                    }
                    "add_scale" => {
                        for c in self.chars.iter_mut() {
                            let scale = values.pop_front().unwrap().into_f32();
                            c.scale += scale;
                        }
                    }
                    "set_scale" => {
                        for c in self.chars.iter_mut() {
                            let scale = values.pop_front().unwrap().into_f32();
                            c.scale = scale;
                        }
                    }
                    _ => {
                        error_panic(&format!("GMBitmapText::send_message, unknown method: '{}', tag: 'chars'", method));
                    }
                }
            }
            _ => {
                error_panic(&format!("GMBitmapText::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

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
