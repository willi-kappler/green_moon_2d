use std::fmt::Debug;
// use std::rc::Rc;
use std::f32::consts::TAU;

use log::debug;
use nanorand::{Rng, WyRand, SeedableRng};

use crate::context::GMContext;
use crate::util::{error_panic, parse_f32};
use crate::bitmap_text::{GMBitmapTextBase};
// use crate::sprite::GMSprite;
// use crate::sprite_effect::GMSpriteEffectT;



pub trait GMTextEffectT: Debug {
    fn update(&mut self, _text: &mut GMBitmapTextBase, _context: &mut GMContext) {
    }

    fn draw(&self, _text: &GMBitmapTextBase, _context: &mut GMContext) {
    }

    fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
    }

    fn clone_box(&self) -> Box<dyn GMTextEffectT>;
}

impl Clone for Box<dyn GMTextEffectT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct GMTextEffectReset {
}

impl GMTextEffectReset {
    pub fn new() -> Self {
        debug!("GMTextEffectReset::new()");

        Self { }
    }
}

impl GMTextEffectT for GMTextEffectReset {
    fn update(&mut self, text: &mut GMBitmapTextBase, _context: &mut GMContext) {
        text.reset_chars();
    }

    fn clone_box(&self) -> Box<dyn GMTextEffectT> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMTextEffectWave {
    amplitude: f32,
    speed: f32,
    offset: f32,
    time: f32,
}

impl GMTextEffectWave {
    pub fn new(amplitude: f32, speed: f32, offset: f32) -> Self {
        debug!("GMTextEffectWave::new(), amplitude: '{}', speed: '{}', offset: '{}'", amplitude, speed, offset);

        Self {
            amplitude,
            speed,
            offset,
            time: 0.0,
        }
    }
}

impl GMTextEffectT for GMTextEffectWave {
    fn update(&mut self, text: &mut GMBitmapTextBase, _context: &mut GMContext) {
        let mut offset = 0.0;

        if text.get_horizontal() {
            for bitmap_char in text.get_chars_mut().iter_mut() {
                bitmap_char.position.y += self.amplitude * (self.time + offset).sin();
                offset += self.offset;
            }
        } else {
            for bitmap_char in text.get_chars_mut().iter_mut() {
                bitmap_char.position.x += self.amplitude * (self.time + offset).sin();
                offset += self.offset;
            }
        }

        self.time += self.speed;

        if self.time > TAU {
            self.time -= TAU;
        }
    }

    fn send_message(&mut self, message: &str, _context: &mut GMContext) {
        let (name, data) = parse_f32(message);

        match name {
            "set_amplitude" => {
                self.amplitude = data[0];
            }
            "add_amplitude" => {
                self.amplitude += data[0];
            }
            "set_speed" => {
                self.speed = data[0];
            }
            "add_speed" => {
                self.speed += data[0];
            }
            "set_offset" => {
                self.offset = data[0];
            }
            "add_offset" => {
                self.offset += data[0];
            }
            _ => {
                error_panic(&format!("GMTextEffectWave::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMTextEffectT> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMTextEffectShake {
    radius: f32,
    speed: f32,
    time: f32,
    seed: u64,
    rng: WyRand,
}

impl GMTextEffectShake {
    pub fn new(radius: f32, speed: f32) -> Self {
        debug!("GMTextEffectShake::new(), radius: '{}'", radius);

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
    fn update(&mut self, text: &mut GMBitmapTextBase, _context: &mut GMContext) {
        self.time += self.speed;
        self.rng.reseed(u64::to_ne_bytes(self.seed));

        for bitmap_char in text.get_chars_mut().iter_mut() {
            let dx = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;
            let dy = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;

            let position = &mut bitmap_char.position;
            position.x += dx;
            position.y += dy;
        }

        if self.time > 1.0 {
            self.time = 0.0;
            self.seed += 1;
        }

    }

    fn send_message(&mut self, message: &str, _context: &mut GMContext) {
        let (name, data) = parse_f32(message);

        match name {
            "set_speed" => {
                self.speed = data[0];
            }
            "add_speed" => {
                self.speed += data[0];
            }
            "set_radius" => {
                self.radius = data[0];
            }
            "add_radius" => {
                self.radius += data[0];
            }
            _ => {
                error_panic(&format!("GMTextEffectShake::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMTextEffectT> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMTextEffectRotateChars {
    speed: f32,
    offset: f32,
    time: f32,
}

impl GMTextEffectRotateChars {
    pub fn new(speed: f32, offset: f32) -> Self {
        debug!("GMTextEffectRotateChars::new(), speed: '{}', offset: '{}'", speed, offset);

        Self {
            speed,
            offset,
            time: 0.0,
        }
    }
}

impl GMTextEffectT for GMTextEffectRotateChars {
    fn update(&mut self, text: &mut GMBitmapTextBase, _context: &mut GMContext) {
        let mut delta = 0.0;

        for bitmap_char in text.get_chars_mut().iter_mut() {
            bitmap_char.angle = self.time + delta;
            delta += self.offset;
        }

        self.time += self.speed;
    }

    fn send_message(&mut self, message: &str, _context: &mut GMContext) {
        let (name, data) = parse_f32(message);

        match name {
            "set_speed" => {
                self.speed = data[0];
            }
            "add_speed" => {
                self.speed += data[0];
            }
            "set_offset" => {
                self.offset = data[0];
            }
            "add_offset" => {
                self.offset += data[0];
            }
            _ => {
                error_panic(&format!("GMTextEffectRotateChars::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMTextEffectT> {
        Box::new(self.clone())
    }
}


#[derive(Debug, Clone)]
pub struct GMTextEffectScale {
    factor_min: f32,
    factor_max: f32,
    speed: f32,
    offset: f32,
    factor: f32,
}

impl GMTextEffectScale {
    pub fn new(factor_min: f32, factor_max: f32, speed: f32, offset: f32) -> Self {
        debug!("GMTextEffectScale::new(), factor_min: '{}', factor_max: '{}', speed: '{}', offset: '{}'", factor_min, factor_max, speed, offset);

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
    fn update(&mut self, _text: &mut GMBitmapTextBase, _context: &mut GMContext) {
        // TODO:
        todo!();
    }

    fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
        // TODO:
        todo!();
    }

    fn clone_box(&self) -> Box<dyn GMTextEffectT> {
        Box::new(self.clone())
    }
}
