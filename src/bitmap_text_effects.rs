
use std::f32::consts::TAU;
use std::collections::VecDeque;

use log::debug;
use nanorand::{Rng, WyRand, SeedableRng};

use crate::object::{GMObjectT};
use crate::value::GMValue;
use crate::target::GMTarget;
use crate::object_manager::GMObjectManager;
use crate::util::{error_panic, send_message_f32};
use crate::math::GMVec2D;
use crate::message::{GMMessage, msg0, msg1};

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
}

impl GMObjectT for GMTEWave {
    fn send_message(&mut self, message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.tag.as_str();
        let method = message.method.as_str();
        let value = message.value;

        match tag {
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

    fn update(&mut self, object_manager: &GMObjectManager) {
        let messages = vec![msg0("get_horizontal"), msg0("get_char_count")];

        let result = object_manager.send_message_multiple(&self.target, messages);
        let mut values = result.to_vec_deque();
        let horizontal = values.pop_front().unwrap().into_bool();
        let num_of_chars = values.pop_front().unwrap().into_usize();

        let mut offset = 0.0;

        if horizontal {
            let mut new_positions = VecDeque::with_capacity(num_of_chars);

            for _ in 0..num_of_chars {
                let new_y = (self.time + offset).sin() * self.amplitude;
                new_positions.push_back(GMValue::F32(new_y));
                offset += self.offset;
            }
            object_manager.send_message(&self.target, msg1("add_chars_y", new_positions));
        } else {
            let mut new_positions = VecDeque::with_capacity(num_of_chars);

            for _ in 0..num_of_chars {
                let new_x = (self.time + offset).sin() * self.amplitude;
                new_positions.push_back(GMValue::F32(new_x));
                offset += self.offset;
            }
            object_manager.send_message(&self.target, msg1("add_chars_x", new_positions));
        }

        self.time += self.speed;
        if self.time > TAU {
            self.time -= TAU;
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
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
}

impl GMObjectT for GMTEShake {
    fn send_message(&mut self, message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.tag.as_str();
        let method = message.method.as_str();
        let value = message.value;

        match tag {
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

    fn update(&mut self, object_manager: &GMObjectManager) {
        let result = object_manager.send_message(&self.target, msg0("get_char_count"));
        let num_of_chars = result.into_usize();

        self.time += self.speed;
        self.rng.reseed(u64::to_ne_bytes(self.seed));

        let mut new_positions = VecDeque::with_capacity(num_of_chars);

        for _ in 0..num_of_chars {
            let dx = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;
            let dy = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;
            new_positions.push_back(GMValue::Vec2D(GMVec2D::new(dx, dy)));
        }

        object_manager.send_message(&self.target, msg1("add_chars_position", new_positions));

        if self.time > 1.0 {
            self.time = 0.0;
            self.seed += 1;
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
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
}

impl GMObjectT for GMTERotateChars {
    fn send_message(&mut self, message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.tag.as_str();
        let method = message.method.as_str();
        let value = message.value;

        match tag {
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

    fn update(&mut self, object_manager: &GMObjectManager) {
        let result = object_manager.send_message(&self.target, msg0("get_char_count"));
        let num_of_chars = result.into_usize();

        let mut delta = 0.0;
        let mut new_angles = VecDeque::with_capacity(num_of_chars);

        for _ in 0..num_of_chars {
            new_angles.push_back(GMValue::F32(self.time + delta));
            delta += self.offset;
        }

        object_manager.send_message(&self.target, msg1("set_chars_angle", new_angles));

        self.time += self.speed;
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
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
}

impl GMObjectT for GMTEScale {
    fn send_message(&mut self, message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.tag.as_str();
        let method = message.method.as_str();
        let value = message.value;

        match tag {
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

    fn update(&mut self, object_manager: &GMObjectManager) {
        let result = object_manager.send_message(&self.target, msg0("get_char_count"));
        let num_of_chars = result.into_usize();

        let mut offset = 0.0;
        let mut new_scales = VecDeque::with_capacity(num_of_chars);

        for _ in 0..num_of_chars {
            new_scales.push_back(GMValue::F32(self.base + (self.amplitude * (self.time + offset).sin())));
            offset += self.offset;
        }

        object_manager.send_message(&self.target, msg1("set_chars_scale", new_scales));

        self.time += self.speed;
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
