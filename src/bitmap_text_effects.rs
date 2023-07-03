
use std::f32::consts::TAU;

use log::debug;
use nanorand::{Rng, WyRand, SeedableRng};

use crate::bitmap_text::GMBitmapText;
use crate::object::{GMObjectT, GMObjectBox};
use crate::value::GMValue;
use crate::target::GMTarget;
use crate::object_manager::GMObjectManager;
use crate::util::{error_panic, send_message_f32};
use crate::math::GMVec2D;
use crate::message::{GMMessage, msg0v, msgt0v, msgt1v};
use crate::context::GMContext;

#[derive(Debug, Clone)]
pub struct GMTEWave {
    pub target: GMTarget,
    pub amplitude: f32,
    pub speed: f32,
    pub offset: f32,
    time: f32,
}

impl GMTEWave {
    pub fn new<T: Into<GMTarget>>(target: T, amplitude: f32, speed: f32, offset: f32) -> Self {
        let target = target.into();
        debug!("GMTEWave::new(), target: {:?}, amplitude: '{}', speed: '{}', offset: '{}'", target, amplitude, speed, offset);

        Self {
            target,
            amplitude,
            speed,
            offset,
            time: 0.0,
        }
    }

    pub fn update_inner(&mut self, horizontal: bool, num_of_chars: usize) -> GMMessage {
        let mut offset = 0.0;

        let message = if horizontal {
            let mut new_positions = Vec::with_capacity(num_of_chars);

            for _ in 0..num_of_chars {
                let new_y = (self.time + offset).sin() * self.amplitude;
                new_positions.push(new_y);
                offset += self.offset;
            }
            msgt1v("chars", "add_y", GMValue::from_any(new_positions))
        } else {
            let mut new_positions = Vec::with_capacity(num_of_chars);

            for _ in 0..num_of_chars {
                let new_x = (self.time + offset).sin() * self.amplitude;
                new_positions.push(new_x);
                offset += self.offset;
            }
            msgt1v("chars", "add_x", GMValue::from_any(new_positions))
        };

        self.time += self.speed;
        if self.time > TAU {
            self.time -= TAU;
        }

        message
    }

    pub fn update_text(&mut self, text: &mut GMBitmapText, object_manager: &GMObjectManager) {
        let horizontal = text.horizontal;
        let num_of_chars = text.get_char_count();
        let message = self.update_inner(horizontal, num_of_chars);
        text.send_message(message, object_manager);
    }

    pub fn update_object(&mut self, object: &mut GMObjectBox, object_manager: &GMObjectManager) {
        let messages = vec![msgt0v("horizontal", "get"), msg0v("get_char_count")];

        let mut values = object.send_message_multiple(messages, object_manager);
        let horizontal = values.pop_front().unwrap().into_bool();
        let num_of_chars = values.pop_front().unwrap().into_usize();

        let message = self.update_inner(horizontal, num_of_chars);
        object.send_message(message, object_manager);
    }

}

impl GMObjectT for GMTEWave {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "update_inner" => {
                        let (horizontal, num_of_chars) = value.into_generic::<(bool, usize)>();
                        let message = self.update_inner(horizontal, num_of_chars);
                        return message.into();
                    }
                    _ => {
                        error_panic(&format!("GMTEWave::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, value);
            }
            "amplitude" => {
                return send_message_f32(&mut self.amplitude, method, value);
            }
            "speed" => {
                return send_message_f32(&mut self.speed, method, value);
            }
            "offset" => {
                return send_message_f32(&mut self.offset, method, value);
            }
            _ => {
                error_panic(&format!("GMTEWave::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        let messages = vec![msgt0v("horizontal", "get"), msg0v("get_char_count")];

        let result = object_manager.send_message_multiple(&self.target, messages);
        let mut values = result.to_vec_deque();
        let horizontal = values.pop_front().unwrap().into_bool();
        let num_of_chars = values.pop_front().unwrap().into_usize();

        let message = self.update_inner(horizontal, num_of_chars);
        object_manager.send_message(&self.target, message);
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMTEShake {
    pub target: GMTarget,
    pub radius: f32,
    pub speed: f32,
    time: f32,
    seed: u64,
    rng: WyRand,
}

impl GMTEShake {
    pub fn new<T: Into<GMTarget>>(target: T, radius: f32, speed: f32) -> Self {
        let target = target.into();
        debug!("GMTEShake::new(), target: '{:?}', radius: '{}', speed: '{}'", target, radius, speed);

        let seed = 1;
        let rng = WyRand::new();

        Self {
            target,
            radius,
            speed,
            time: 0.0,
            seed,
            rng,
        }
    }

    pub fn update_inner(&mut self, num_of_chars: usize) -> GMMessage {
        self.time += self.speed;
        self.rng.reseed(u64::to_ne_bytes(self.seed));

        let mut new_positions = Vec::with_capacity(num_of_chars);

        for _ in 0..num_of_chars {
            let dx = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;
            let dy = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;
            new_positions.push(GMVec2D::new(dx, dy));
        }

        if self.time > 1.0 {
            self.time = 0.0;
            self.seed += 1;
        }

        msgt1v("chars", "add_position", GMValue::from_any(new_positions))
    }

    pub fn update_text(&mut self, text: &mut GMBitmapText, object_manager: &GMObjectManager) {
        let num_of_chars = text.get_char_count();

        let message = self.update_inner(num_of_chars);
        text.send_message(message, object_manager);
    }

    pub fn update_object(&mut self, object: &mut GMObjectBox, object_manager: &GMObjectManager) {
        let result = object.send_message(msg0v("get_char_count"), object_manager);
        let num_of_chars = result.into_usize();

        let message = self.update_inner(num_of_chars);
        object.send_message(message, object_manager);
    }
}

impl GMObjectT for GMTEShake {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "update_inner" => {
                        let num_of_chars = value.into_usize();
                        let message = self.update_inner(num_of_chars);
                        return message.into();
                    }
                    _ => {
                        error_panic(&format!("GMTEShake::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, value);
            }
            "radius" => {
                return send_message_f32(&mut self.radius, method, value);
            }
            "speed" => {
                return send_message_f32(&mut self.speed, method, value);
            }
            _ => {
                error_panic(&format!("GMTEShake::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        let result = object_manager.send_message(&self.target, msg0v("get_char_count"));
        let num_of_chars = result.into_usize();

        let message = self.update_inner(num_of_chars);
        object_manager.send_message(&self.target, message);
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMTERotateChars {
    pub target: GMTarget,
    pub speed: f32,
    pub offset: f32,
    time: f32,
}

impl GMTERotateChars {
    pub fn new<T: Into<GMTarget>>(target: T, speed: f32, offset: f32) -> Self {
        let target = target.into();
        debug!("GMTERotateChars::new(), target: '{:?}' speed: '{}', offset: '{}'", target, speed, offset);

        Self {
            target,
            speed,
            offset,
            time: 0.0,
        }
    }

    pub fn update_inner(&mut self, num_of_chars: usize) -> GMMessage {
        let mut delta = 0.0;
        let mut new_angles = Vec::with_capacity(num_of_chars);

        for _ in 0..num_of_chars {
            new_angles.push(self.time + delta);
            delta += self.offset;
        }

        self.time += self.speed;

        msgt1v("chars", "set_angle", GMValue::from_any(new_angles))
    }

    pub fn update_text(&mut self, text: &mut GMBitmapText, object_manager: &GMObjectManager) {
        let num_of_chars = text.get_char_count();

        let message = self.update_inner(num_of_chars);
        text.send_message(message, object_manager);
    }

    pub fn update_object(&mut self, object: &mut GMObjectBox, object_manager: &GMObjectManager) {
        let result = object.send_message(msg0v("get_char_count"), object_manager);
        let num_of_chars = result.into_usize();

        let message = self.update_inner(num_of_chars);
        object.send_message(message, object_manager);
    }
}

impl GMObjectT for GMTERotateChars {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "update_inner" => {
                        let num_of_chars = value.into_usize();
                        let message = self.update_inner(num_of_chars);
                        return message.into();
                    }
                    _ => {
                        error_panic(&format!("GMTERotateChars::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, value);
            }
            "speed" => {
                return send_message_f32(&mut self.speed, method, value);
            }
            "offset" => {
                return send_message_f32(&mut self.offset, method, value);
            }
            _ => {
                error_panic(&format!("GMTERotateChars::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        let result = object_manager.send_message(&self.target, msg0v("get_char_count"));
        let num_of_chars = result.into_usize();

        let message = self.update_inner(num_of_chars);
        object_manager.send_message(&self.target, message);
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMTEScale {
    pub target: GMTarget,
    pub amplitude: f32,
    pub base: f32,
    pub speed: f32,
    pub offset: f32,
    time: f32,
}

impl GMTEScale {
    pub fn new<T: Into<GMTarget>>(target: T, amplitude: f32, base: f32, speed: f32, offset: f32) -> Self {
        let target = target.into();
        debug!("GMTEScale::new(), target: '{:?}', amplitude: '{}', base: '{}', speed: '{}', offset: '{}'", target, amplitude, base, speed, offset);

        Self {
            target,
            amplitude,
            base,
            speed,
            offset,
            time: 0.0,
        }
    }

    pub fn update_inner(&mut self, num_of_chars: usize) -> GMMessage {
        let mut offset = 0.0;
        let mut new_scales = Vec::with_capacity(num_of_chars);

        for _ in 0..num_of_chars {
            new_scales.push(self.base + (self.amplitude * (self.time + offset).sin()));
            offset += self.offset;
        }

        self.time += self.speed;

        msgt1v("chars", "set_scale", GMValue::from_any(new_scales))
    }

    pub fn update_text(&mut self, text: &mut GMBitmapText, object_manager: &GMObjectManager) {
        let num_of_chars = text.get_char_count();

        let message = self.update_inner(num_of_chars);
        text.send_message(message, object_manager);
    }

    pub fn update_object(&mut self, object: &mut GMObjectBox, object_manager: &GMObjectManager) {
        let result = object.send_message(msg0v("get_char_count"), object_manager);
        let num_of_chars = result.into_usize();

        let message = self.update_inner(num_of_chars);
        object.send_message(message, object_manager);
    }
}

impl GMObjectT for GMTEScale {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "update_inner" => {
                        let num_of_chars = value.into_usize();
                        let message = self.update_inner(num_of_chars);
                        return message.into();
                    }
                    _ => {
                        error_panic(&format!("GMTEScale::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, value);
            }
            "amplitude" => {
                return send_message_f32(&mut self.amplitude, method, value);
            }
            "base" => {
                return send_message_f32(&mut self.base, method, value);
            }
            "speed" => {
                return send_message_f32(&mut self.speed, method, value);
            }
            "offset" => {
                return send_message_f32(&mut self.offset, method, value);
            }
            _ => {
                error_panic(&format!("GMTEScale::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        let result = object_manager.send_message(&self.target, msg0v("get_char_count"));
        let num_of_chars = result.into_usize();

        let message = self.update_inner(num_of_chars);
        object_manager.send_message(&self.target, message);
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}
