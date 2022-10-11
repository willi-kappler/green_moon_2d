use std::fmt::Debug;
// use std::rc::Rc;
use std::f32::consts::TAU;

use log::debug;
use nanorand::{Rng, WyRand, SeedableRng};

use crate::context::GMContext;
use crate::bitmap_text::{GMBitmapTextBase};
use crate::data::GMData;
use crate::util::{error_panic};
use crate::effect::GMEffectT;

// use crate::sprite::GMSprite;
// use crate::sprite_effect::GMSpriteEffectT;


#[derive(Debug, Clone)]
pub struct GMTEReset {
    active: bool,
}

impl GMTEReset {
    pub fn new() -> Self {
        debug!("GMTEReset::new()");

        Self {
            active: true,
        }
    }
}

impl GMEffectT<GMBitmapTextBase> for GMTEReset {
    fn update(&mut self, text: &mut GMBitmapTextBase, _context: &mut GMContext) {
        if self.active {
            text.reset_chars();
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMBitmapTextBase>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMTEWave {
    amplitude: f32,
    speed: f32,
    offset: f32,
    time: f32,
    active: bool,
}

impl GMTEWave {
    pub fn new(amplitude: f32, speed: f32, offset: f32) -> Self {
        debug!("GMTEWave::new(), amplitude: '{}', speed: '{}', offset: '{}'", amplitude, speed, offset);

        Self {
            amplitude,
            speed,
            offset,
            time: 0.0,
            active: true,
        }
    }
}

impl GMEffectT<GMBitmapTextBase> for GMTEWave {
    fn update(&mut self, text: &mut GMBitmapTextBase, _context: &mut GMContext) {
        if self.active {
            let mut offset = 0.0;

            if text.horizontal {
                for bitmap_char in text.chars.iter_mut() {
                    bitmap_char.position.y += self.amplitude * (self.time + offset).sin();
                    offset += self.offset;
                }
            } else {
                for bitmap_char in text.chars.iter_mut() {
                    bitmap_char.position.x += self.amplitude * (self.time + offset).sin();
                    offset += self.offset;
                }
            }

            self.time += self.speed;

            if self.time > TAU {
                self.time -= TAU;
            }
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {

        match message {
            "set_amplitude" => {
                self.amplitude = data.into();
            }
            "add_amplitude" => {
                let data: f32 = data.into();
                self.amplitude += data;
            }
            "set_speed" => {
                self.speed = data.into();
            }
            "add_speed" => {
                let data: f32 = data.into();
                self.speed += data;
            }
            "set_offset" => {
                self.offset = data.into();
            }
            "add_offset" => {
                let data: f32 = data.into();
                self.offset += data;
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMTEWave::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMBitmapTextBase>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMTEShake {
    radius: f32,
    speed: f32,
    time: f32,
    seed: u64,
    rng: WyRand,
    active: bool,
}

impl GMTEShake {
    pub fn new(radius: f32, speed: f32) -> Self {
        debug!("GMTEShake::new(), radius: '{}'", radius);

        let seed = 1;
        let rng = WyRand::new();

        Self {
            radius,
            speed,
            time: 0.0,
            seed,
            rng,
            active: true,
        }
    }
}

impl GMEffectT<GMBitmapTextBase> for GMTEShake {
    fn update(&mut self, text: &mut GMBitmapTextBase, _context: &mut GMContext) {
        if self.active {
            self.time += self.speed;
            self.rng.reseed(u64::to_ne_bytes(self.seed));

            for bitmap_char in text.chars.iter_mut() {
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
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_speed" => {
                self.speed = data.into();
            }
            "add_speed" => {
                let data: f32 = data.into();
                self.speed += data;
            }
            "set_radius" => {
                self.radius = data.into();
            }
            "add_radius" => {
                let data: f32 = data.into();
                self.radius += data;
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMTEShake::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMBitmapTextBase>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMTERotateChars {
    speed: f32,
    offset: f32,
    time: f32,
    active: bool,
}

impl GMTERotateChars {
    pub fn new(speed: f32, offset: f32) -> Self {
        debug!("GMTERotateChars::new(), speed: '{}', offset: '{}'", speed, offset);

        Self {
            speed,
            offset,
            time: 0.0,
            active: true,
        }
    }
}

impl GMEffectT<GMBitmapTextBase> for GMTERotateChars {
    fn update(&mut self, text: &mut GMBitmapTextBase, _context: &mut GMContext) {
        if self.active {
            let mut delta = 0.0;

            for bitmap_char in text.chars.iter_mut() {
                bitmap_char.angle = self.time + delta;
                delta += self.offset;
            }

            self.time += self.speed;
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_speed" => {
                self.speed = data.into();
            }
            "add_speed" => {
                let data: f32 = data.into();
                self.speed += data;
            }
            "set_offset" => {
                self.offset = data.into();
            }
            "add_offset" => {
                let data: f32 = data.into();
                self.offset += data;
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMTERotateChars::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMBitmapTextBase>> {
        Box::new(self.clone())
    }
}


#[derive(Debug, Clone)]
pub struct GMTEScale {
    amplitude: f32,
    base: f32,
    speed: f32,
    offset: f32,
    time: f32,
    active: bool,
}

impl GMTEScale {
    pub fn new(amplitude: f32, base: f32, speed: f32, offset: f32) -> Self {
        debug!("GMTEScale::new(), amplitude: '{}', base: '{}', speed: '{}', offset: '{}'", amplitude, base, speed, offset);

        Self {
            amplitude,
            base,
            speed,
            offset,
            time: 0.0,
            active: true,
        }
    }
}

impl GMEffectT<GMBitmapTextBase> for GMTEScale {
    fn update(&mut self, text: &mut GMBitmapTextBase, _context: &mut GMContext) {
        let mut offset = 0.0;

        if self.active {
            for bitmap_char in text.chars.iter_mut() {
                bitmap_char.scale = self.base + (self.amplitude * (self.time + offset).sin());
                offset += self.offset;
            }

            self.time += self.speed;
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_amplitude" => {
                self.amplitude = data.into();
            }
            "add_amplitude" => {
                let data: f32 = data.into();
                self.amplitude += data;
            }
            "set_base" => {
                self.base = data.into();
            }
            "add_base" => {
                let data: f32 = data.into();
                self.base += data;
            }
            "set_speed" => {
                self.speed = data.into();
            }
            "add_speed" => {
                let data: f32 = data.into();
                self.speed += data;
            }
            "set_offset" => {
                self.offset = data.into();
            }
            "add_offset" => {
                let data: f32 = data.into();
                self.offset += data;
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMTEScale::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMBitmapTextBase>> {
        Box::new(self.clone())
    }
}

pub struct GMTECircle {
    // TODO: chars move around circle, optionally rotated to radial direction
}
