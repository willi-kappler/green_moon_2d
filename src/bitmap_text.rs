

use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Debug;
use std::any::Any;
use std::f32::consts::TAU;
use log::debug;

use crate::texture::GMTexture;
use crate::context::GMContext;
use crate::util::{error_panic, extract_f32_value};

#[derive(Debug, Clone)]
pub struct GMBitmapFont {
    texture: Rc<GMTexture>,
    mapping: HashMap<char, u32>,
}

impl GMBitmapFont {
    pub fn new(texture: Rc<GMTexture>, char_mapping: &str) -> Self {
        debug!("GMBitmapFont::new(), char_mapping: {}", char_mapping);

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
    pub font: Rc<GMBitmapFont>,
    pub text: String,
    pub base_x: f32,
    pub base_y: f32,
    pub spacing_x: f32,
    pub spacing_y: f32,
    pub horizontal: bool,
    pub chars: Vec<(u32, f32, f32, f32)>, // index, x, y, angle
}

impl GMBitmapText {
    pub fn new(font: Rc<GMBitmapFont>, text: &str, x: f32, y: f32) -> Self {
        debug!("GMBitmapText::new(), text: {}, x: {}, y: {}", text, x, y);

        let mut text = Self {
            font,
            text: text.to_string(),
            base_x: x,
            base_y: y,
            spacing_x: 0.0,
            spacing_y: 0.0,
            horizontal: true,
            chars: Vec::new(),
        };

        text.reset_chars();

        text
    }

    pub fn new2(font: &str, text: &str, x: f32, y: f32, context: &GMContext) -> Self {
        debug!("GMBitmapText::new2(), font: {}, text: {}, x: {}, y: {}", font, text, x, y);

        Self::new(context.resources.get_font_clone(font), text, x, y)
    }

    pub fn reset_chars(&mut self) {
        if self.horizontal {
            let mut x = self.base_x;
            let (dx, _) = self.font.get_char_dimensions();
            self.chars.clear();

            for c in self.text.chars() {
                let index = self.font.get_index(c);
                self.chars.push((index, x, self.base_y, 0.0));
                x += dx + self.spacing_x;
            }
        } else {
            let mut y = self.base_y;
            let (_, dy) = self.font.get_char_dimensions();
            self.chars.clear();

            for c in self.text.chars() {
                let index = self.font.get_index(c);
                self.chars.push((index, self.base_x, y, 0.0));
                y += dy + self.spacing_y;
            }
        }
    }
}

pub trait GMTextEffectT {
    fn update(&mut self, _text: &mut GMBitmapText, _context: &mut GMContext) {
    }

    fn draw(&self, _text: &GMBitmapText, _context: &mut GMContext) {
    }

    fn send_message(&mut self, _message: &str, _data: Option<Box<dyn Any>>, _context: &mut GMContext) {
    }
}

pub struct GMTextEffectDraw {
}

impl GMTextEffectDraw {
    pub fn new() -> Self {
        debug!("GMTextEffectDraw::new()");

        Self { }
    }
}

impl GMTextEffectT for GMTextEffectDraw {
    fn draw(&self, text: &GMBitmapText, context: &mut GMContext) {
        for (index, x, y, angle) in text.chars.iter() {
            text.font.draw_opt(*index, *x, *y, *angle, false, false, context);
        }
    }

}

pub struct GMTextEffectReset {
}

impl GMTextEffectReset {
    pub fn new() -> Self {
        debug!("GMTextEffectReset::new()");

        Self { }
    }
}

impl GMTextEffectT for GMTextEffectReset {
    fn update(&mut self, text: &mut GMBitmapText, _context: &mut GMContext) {
        text.reset_chars();
    }
}

pub struct GMTextEffectWave {
    pub amplitude: f32,
    pub speed: f32,
    pub offset: f32,
    pub time: f32,
}

impl GMTextEffectWave {
    pub fn new(amplitude: f32, speed: f32, offset: f32) -> Self {
        debug!("GMTextEffectWave::new(), amplitude: {}, speed: {}, offset: {}", amplitude, speed, offset);

        Self {
            amplitude,
            speed,
            offset,
            time: 0.0,
        }
    }
}

impl GMTextEffectT for GMTextEffectWave {
    fn update(&mut self, text: &mut GMBitmapText, _context: &mut GMContext) {
        let mut offset = 0.0;

        if text.horizontal {
            for (_, _, y, _) in text.chars.iter_mut() {
                *y = text.base_y + (self.amplitude * (self.time + offset).sin());
                offset += self.offset;
            }
        } else {
            for (_, x, _, _) in text.chars.iter_mut() {
                *x = text.base_x + (self.amplitude * (self.time + offset).sin());
                offset += self.offset;
            }
        }

        self.time += self.speed;

        if self.time > TAU {
            self.time -= TAU;
        }
    }

    fn send_message(&mut self, message: &str, data: Option<Box<dyn Any>>, _context: &mut GMContext) {
        match message {
            "amplitude" => {
                self.amplitude = extract_f32_value(message, data);
            }
            "speed" => {
                self.speed = extract_f32_value(message, data);
            }
            "offset" => {
                self.offset = extract_f32_value(message, data);
            }
            _ => {
                error_panic(&format!("GMTextEffectWave::send_message(), unknown message: {}", message))
            }
        }
    }
}
