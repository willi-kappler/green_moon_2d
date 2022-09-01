

use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Debug;
use std::any::Any;
use std::f32::consts::TAU;
use log::debug;
use nanorand::{Rng, WyRand, SeedableRng};

use crate::texture::GMTexture;
use crate::context::GMContext;
use crate::util::{error_panic, extract_f32_value, extract_usize_value, GMAlign};

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
    pub font: Rc<GMBitmapFont>,
    pub text: String,
    pub base_x: f32,
    pub base_y: f32,
    pub spacing_x: f32,
    pub spacing_y: f32,
    pub horizontal: bool,
    pub chars: Vec<(u32, f32, f32, f32)>, // index, x, y, angle
    pub width: f32,
    pub height: f32,
    pub align: GMAlign,
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

        Self::new(&context.resources.get_font_clone(font), text, x, y)
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
}


// TODO: Add GMTextBlock

// TODO: Add GMTextList


pub trait GMTextEffectT: Debug {
    fn update(&mut self, _text: &mut GMBitmapText, _context: &mut GMContext) {
    }

    fn draw(&self, _text: &GMBitmapText, _context: &mut GMContext) {
    }

    fn send_message(&mut self, _message: &str, _data: Option<Box<dyn Any>>, _context: &mut GMContext) {
    }

    fn send_message_f32(&mut self, message: &str, data: f32, context: &mut GMContext) {
        self.send_message(message, Some(Box::new(data)), context);
    }

    fn send_message_multiple(&mut self, message1: &str, index: usize, message2: &'static str, data: Option<Box<dyn Any>>, context: &mut GMContext) {
        let data: Box<(usize, &str, Option<Box<dyn Any>>)> = Box::new((index, message2, data));
        self.send_message(message1, Some(data), context);
    }

    fn send_message_multiple_f32(&mut self, message1: &str, index: usize, message2: &'static str, data: f32, context: &mut GMContext) {
        let data: Box<(usize, &str, Option<Box<dyn Any>>)> = Box::new((index, message2, Some(Box::new(data))));
        self.send_message(message1, Some(data), context);
    }
}

#[derive(Debug)]
pub struct GMTextEffectEmpty {
}

impl GMTextEffectEmpty {
    pub fn new() -> Self {
        debug!("GMTextEffectEmpty::new()");

        Self { }
    }
}

impl GMTextEffectT for GMTextEffectEmpty {
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct GMTextEffectMultiple {
    pub text_effects: Vec<Box<dyn GMTextEffectT>>,
}

impl GMTextEffectMultiple {
    pub fn new() -> Self {
        debug!("GMTextEffectMultiple::new()");

        Self {
            text_effects: Vec::new(),
        }
    }

    pub fn add_text_effect<T: 'static + GMTextEffectT>(&mut self, effect: T) {
        self.text_effects.push(Box::new(effect));
    }

    pub fn remove_text_effect(&mut self, index: usize) {
        self.text_effects.remove(index);
    }

    pub fn clear(&mut self) {
        self.text_effects.clear();
    }
}

impl GMTextEffectT for GMTextEffectMultiple {
    fn update(&mut self, text: &mut GMBitmapText, context: &mut GMContext) {
        for text_effect in self.text_effects.iter_mut() {
            text_effect.update(text, context);
        }
    }

    fn draw(&self, text: &GMBitmapText, context: &mut GMContext) {
        for text_effect in self.text_effects.iter() {
            text_effect.draw(text, context);
        }
    }

    fn send_message(&mut self, message: &str, data: Option<Box<dyn Any>>, context: &mut GMContext) {
        match message {
            "add_text_effect" => {
                if let Some(data) = data {
                    if let Ok(value) = data.downcast::<Box<dyn GMTextEffectT>>() {
                        self.text_effects.push(*value);
                        return
                    }
                }

                error_panic(&format!("GMTextEffectMultiple::send_message(), expected Box<dyn GMTextEffectT>, message: {}", message))
            }
            "remove_text_effect" => {
                let index = extract_usize_value(message, data);
                self.text_effects.remove(index);
            }
            "clear" => {
                self.clear();
            }
            "send_message" => {
                if let Some(data) = data {
                    if let Ok(value) = data.downcast::<(usize, &str, Option<Box<dyn Any>>)>() {
                        let (index, message2, data2) = *value;
                        self.text_effects[index].send_message(message2, data2, context);
                        return
                    }
                }

                error_panic(&format!("GMTextEffectMultiple::send_message(), expected (usize, &str, Option<Box<dyn Any>>), message: {}", message))
            }
            _ => {
                error_panic(&format!("GMTextEffectMultiple::send_message(), unknown message: {}", message))
            }
        }
    }
}

#[derive(Debug)]
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
                *y = *y + (self.amplitude * (self.time + offset).sin());
                offset += self.offset;
            }
        } else {
            for (_, x, _, _) in text.chars.iter_mut() {
                *x = *x + (self.amplitude * (self.time + offset).sin());
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
            "set_amplitude" => {
                self.amplitude = extract_f32_value(message, data);
            }
            "add_amplitude" => {
                self.amplitude += extract_f32_value(message, data);
            }
            "set_speed" => {
                self.speed = extract_f32_value(message, data);
            }
            "add_speed" => {
                self.speed += extract_f32_value(message, data);
            }
            "set_offset" => {
                self.offset = extract_f32_value(message, data);
            }
            "add_offset" => {
                self.offset += extract_f32_value(message, data);
            }
            _ => {
                error_panic(&format!("GMTextEffectWave::send_message(), unknown message: {}", message))
            }
        }
    }
}

#[derive(Debug)]
pub struct GMTextEffectShake {
    pub radius: f32,
    pub speed: f32,
    pub time: f32,
    pub seed: u64,
    pub rng: WyRand,
}

impl GMTextEffectShake {
    pub fn new(radius: f32, speed: f32) -> Self {
        debug!("GMTextEffectShake::new(), radius: {}", radius);

        let seed = 1;
        let rng = WyRand::new();

        Self {
            radius,
            speed,
            time: 0.0,
            seed,
            rng,
        }
    }
}

impl GMTextEffectT for GMTextEffectShake {
    fn update(&mut self, text: &mut GMBitmapText, _context: &mut GMContext) {
        self.time += self.speed;
        self.rng.reseed(u64::to_ne_bytes(self.seed));

        for (_, x, y, _) in text.chars.iter_mut() {
            let dx = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;
            let dy = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;

            *x += dx;
            *y += dy;
        }

        if self.time > 1.0 {
            self.time = 0.0;
            self.seed += 1;
        }

    }

    fn send_message(&mut self, message: &str, data: Option<Box<dyn Any>>, _context: &mut GMContext) {
        match message {
            "set_speed" => {
                self.speed = extract_f32_value(message, data);
            }
            "add_speed" => {
                self.speed += extract_f32_value(message, data);
            }
            "set_radius" => {
                self.radius = extract_f32_value(message, data);
            }
            "add_radius" => {
                self.radius += extract_f32_value(message, data);
            }
            _ => {
                error_panic(&format!("GMTextEffectShake::send_message(), unknown message: {}", message))
            }
        }
    }
}

#[derive(Debug)]
pub struct GMTextEffectRotateChars {
    pub speed: f32,
    pub offset: f32,
    pub time: f32,
}

impl GMTextEffectRotateChars {
    pub fn new(speed: f32, offset: f32) -> Self {
        Self {
            speed,
            offset,
            time: 0.0,
        }
    }
}

impl GMTextEffectT for GMTextEffectRotateChars {
    fn update(&mut self, text: &mut GMBitmapText, _context: &mut GMContext) {
        let mut delta = 0.0;

        for (_, _, _, angle) in text.chars.iter_mut() {
            *angle = self.time + delta;
            delta += self.offset;
        }

        self.time += self.speed;
    }

    fn send_message(&mut self, message: &str, data: Option<Box<dyn Any>>, _context: &mut GMContext) {
        match message {
            "set_speed" => {
                self.speed = extract_f32_value(message, data);
            }
            "add_speed" => {
                self.speed += extract_f32_value(message, data);
            }
            "set_offset" => {
                self.offset = extract_f32_value(message, data);
            }
            "add_offset" => {
                self.offset += extract_f32_value(message, data);
            }
            _ => {
                error_panic(&format!("GMTextEffectRotateChars::send_message(), unknown message: {}", message))
            }
        }
    }
}
