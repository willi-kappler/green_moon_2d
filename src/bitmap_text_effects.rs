use std::fmt::Debug;
use std::any::Any;
use std::rc::Rc;
use std::f32::consts::TAU;

use log::debug;
use nanorand::{Rng, WyRand, SeedableRng};

use crate::context::GMContext;
use crate::util::{error_panic, extract_f32_value, extract_usize_value};
use crate::bitmap_text::{GMBitmapText, GMBitmapFont};
// use crate::sprite::GMSprite;
// use crate::sprite_effect::GMSpriteEffectT;



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
    text_effects: Vec<Box<dyn GMTextEffectT>>,
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
    amplitude: f32,
    speed: f32,
    offset: f32,
    time: f32,
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

        if text.get_horizontal() {
            for (_, _, y, _) in text.get_mut_chars().iter_mut() {
                *y = *y + (self.amplitude * (self.time + offset).sin());
                offset += self.offset;
            }
        } else {
            for (_, x, _, _) in text.get_mut_chars().iter_mut() {
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
    radius: f32,
    speed: f32,
    time: f32,
    seed: u64,
    rng: WyRand,
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

        for (_, x, y, _) in text.get_mut_chars().iter_mut() {
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
    speed: f32,
    offset: f32,
    time: f32,
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

        for (_, _, _, angle) in text.get_mut_chars().iter_mut() {
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


#[derive(Debug)]
pub struct GMTextEffectScale {
    factor_min: f32,
    factor_max: f32,
    speed: f32,
    offset: f32,
    factor: f32,
}

impl GMTextEffectScale {
    pub fn new(factor_min: f32, factor_max: f32, speed: f32, offset: f32) -> Self {
        Self {
            factor_min,
            factor_max,
            speed,
            offset,
            factor: factor_min,
        }
    }
}

impl GMTextEffectT for GMTextEffectScale {
    fn update(&mut self, _text: &mut GMBitmapText, _context: &mut GMContext) {
        // TODO:
        todo!();
    }

    fn send_message(&mut self, _message: &str, _data: Option<Box<dyn Any>>, _context: &mut GMContext) {
        // TODO:
        todo!();
    }
}

#[derive(Debug)]
pub struct GMTextEffectMenuText {
    text1: GMBitmapText,
    text1_effect: Box<dyn GMTextEffectT>,
    text1_offset: f32,
    text1_offset_min: f32,
    text1_offset_max: f32,
    text2: GMBitmapText,
    text2_effect: Box<dyn GMTextEffectT>,
    text2_offset: f32,
    text2_offset_min: f32,
    text2_offset_max: f32,
    speed: f32,
}

impl GMTextEffectMenuText {
    pub fn new(text1: &str, text2: &str, font: Rc<GMBitmapFont>) -> Self {
        Self {
            text1: GMBitmapText::new(&font, text1, 0.0, 0.0),
            text1_effect: Box::new(GMTextEffectEmpty::new()),
            text1_offset: 50.0,
            text1_offset_min: 5.0,
            text1_offset_max: 50.0,
            text2: GMBitmapText::new(&font, text2, 0.0, 0.0),
            text2_effect: Box::new(GMTextEffectEmpty::new()),
            text2_offset: 50.0,
            text2_offset_min: 5.0,
            text2_offset_max: 50.0,
            speed: 1.0,
        }
    }
}

impl GMTextEffectT for GMTextEffectMenuText {
    fn update(&mut self, text: &mut GMBitmapText, context: &mut GMContext) {
        self.text1_effect.update(&mut self.text1, context);
        self.text2_effect.update(&mut self.text2, context);

        self.text1_offset -= self.speed;
        if self.text1_offset < self.text1_offset_min {
            self.text1_offset = self.text1_offset_max;
        }

        self.text2_offset -= self.speed;
        if self.text2_offset < self.text2_offset_min {
            self.text2_offset = self.text2_offset_max;
        }

        if text.get_horizontal() {

        } else {

        }
    }

    fn draw(&self, _text: &GMBitmapText, context: &mut GMContext) {
        self.text1.draw(context);
        self.text1_effect.draw(&self.text1, context);
        self.text2.draw(context);
        self.text2_effect.draw(&self.text2, context);
    }

    fn send_message(&mut self, _message: &str, _data: Option<Box<dyn Any>>, _context: &mut GMContext) {
    }
}
