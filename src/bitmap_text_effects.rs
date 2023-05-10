
use std::f32::consts::TAU;

use log::debug;
use nanorand::{Rng, WyRand, SeedableRng};

use crate::context::GMContext;
use crate::object::{GMObjectT};
use crate::message::GMMessage;
use crate::value::GMValue;
use crate::target::GMTarget;
use crate::object_manager::GMObjectManager;
use crate::util::error_panic;
use crate::math::GMVec2D;

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
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }
            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            GMMessage::Custom0(name) if name == "get_ampitude" => {
                return self.amplitude.into()
            }
            GMMessage::Custom0(name) if name == "get_speed" => {
                return self.speed.into()
            }
            GMMessage::Custom0(name) if name == "get_offset" => {
                return self.offset.into()
            }
            GMMessage::Custom1(name, GMValue::F32(amplitude)) if name == "set_amplitude" => {
                self.amplitude = amplitude
            }
            GMMessage::Custom1(name, GMValue::F32(speed)) if name == "set_speed" => {
                self.speed = speed
            }
            GMMessage::Custom1(name, GMValue::F32(offset)) if name == "set_offset" => {
                self.offset = offset
            }
            GMMessage::Custom1(name, GMValue::F32(amplitude)) if name == "add_amplitude" => {
                self.amplitude += amplitude
            }
            GMMessage::Custom1(name, GMValue::F32(speed)) if name == "add_speed" => {
                self.speed += speed
            }
            GMMessage::Custom1(name, GMValue::F32(offset)) if name == "add_offset" => {
                self.offset += offset
            }
            _ => {
                error_panic(&format!("Wrong message for GMTEWave::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        let message1 = GMMessage::Custom0("get_horizontal".to_string());
        let message2 = GMMessage::Custom0("get_char_count".to_string());

        let result = object_manager.send_message(&self.target, (message1, message2).into(), context);

        if let GMValue::Multiple(values) = result {
            if let GMValue::Bool(horizontal) = values[0] {
                if let GMValue::USize(num_of_chars) = values[1] {
                    let mut offset = 0.0;

                    if horizontal {
                        let mut new_positions = Vec::with_capacity(num_of_chars);

                        for _ in 0..num_of_chars {
                            let new_y = (self.time + offset).sin() * self.amplitude;
                            new_positions.push(GMValue::F32(new_y));
                            offset += self.offset;
                        }
                        let add_chars_y = ("add_chars_y", new_positions.into()).into();
                        object_manager.send_message(&self.target, add_chars_y, context);
                    } else {
                        let mut new_positions = Vec::with_capacity(num_of_chars);

                        for _ in 0..num_of_chars {
                            let new_x = (self.time + offset).sin() * self.amplitude;
                            new_positions.push(GMValue::F32(new_x));
                            offset += self.offset;
                        }
                        let add_chars_x = ("add_chars_x", new_positions.into()).into();
                        object_manager.send_message(&self.target, add_chars_x, context);
                    }

                    self.time += self.speed;
                    if self.time > TAU {
                        self.time -= TAU;
                    }
                }
            }
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
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }
            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            GMMessage::Custom0(name) if name == "get_radius" => {
                return self.radius.into()
            }
            GMMessage::Custom0(name) if name == "get_speed" => {
                return self.speed.into()
            }
            GMMessage::Custom1(name, GMValue::F32(radius)) if name == "set_radius" => {
                self.radius = radius;
            }
            GMMessage::Custom1(name, GMValue::F32(speed)) if name == "set_speed" => {
                self.speed = speed;
            }
            GMMessage::Custom1(name, GMValue::F32(radius)) if name == "add_radius" => {
                self.radius += radius;
            }
            GMMessage::Custom1(name, GMValue::F32(speed)) if name == "add_speed" => {
                self.speed += speed;
            }
            _ => {
                error_panic(&format!("Wrong message for GMTEShake::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        let result = object_manager.send_message(&self.target, GMMessage::Custom0("get_char_count".to_string()), context);

        if let GMValue::USize(num_of_chars) = result {
            self.time += self.speed;
            self.rng.reseed(u64::to_ne_bytes(self.seed));

            let mut new_positions = Vec::with_capacity(num_of_chars);

            for _ in 0..num_of_chars {
                let dx = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;
                let dy = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;
                new_positions.push(GMValue::Vec2D(GMVec2D::new(dx, dy)));
            }

            object_manager.send_custom_message1(&self.target, "add_chars_position", new_positions, context);
        }

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
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }
            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            GMMessage::Custom0(name) if name == "get_speed" => {
                return self.speed.into()
            }
            GMMessage::Custom0(name) if name == "get_offset" => {
                return self.offset.into()
            }
            GMMessage::Custom1(name, GMValue::F32(speed)) if name == "set_speed" => {
                self.speed = speed;
            }
            GMMessage::Custom1(name, GMValue::F32(offset)) if name == "set_offset" => {
                self.offset = offset;
            }
            GMMessage::Custom1(name, GMValue::F32(speed)) if name == "add_speed" => {
                self.speed += speed;
            }
            GMMessage::Custom1(name, GMValue::F32(offset)) if name == "add_offset" => {
                self.offset += offset;
            }
            _ => {
                error_panic(&format!("Wrong message for GMTERotateChars::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        let result = object_manager.send_message(&self.target, GMMessage::Custom0("get_char_count".to_string()), context);

        if let GMValue::USize(num_of_chars) = result {
            let mut delta = 0.0;
            let mut new_angles = Vec::with_capacity(num_of_chars);

            for _ in 0..num_of_chars {
                new_angles.push(GMValue::F32(self.time + delta));
                delta += self.offset;
            }

            object_manager.send_custom_message1(&self.target, "set_chars_angle", new_angles, context);
        }

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
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }
            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            GMMessage::Custom0(name) if name == "get_amplitude" => {
                return self.amplitude.into()
            }
            GMMessage::Custom0(name) if name == "get_base" => {
                return self.base.into()
            }
            GMMessage::Custom0(name) if name == "get_speed" => {
                return self.speed.into()
            }
            GMMessage::Custom0(name) if name == "get_offset" => {
                return self.offset.into()
            }
            GMMessage::Custom1(name, GMValue::F32(amplitude)) if name == "set_amplitude" => {
                self.amplitude = amplitude;
            }
            GMMessage::Custom1(name, GMValue::F32(base)) if name == "set_base" => {
                self.base = base;
            }
            GMMessage::Custom1(name, GMValue::F32(speed)) if name == "set_speed" => {
                self.speed = speed;
            }
            GMMessage::Custom1(name, GMValue::F32(offset)) if name == "set_offset" => {
                self.offset = offset;
            }
            GMMessage::Custom1(name, GMValue::F32(amplitude)) if name == "add_amplitude" => {
                self.amplitude += amplitude;
            }
            GMMessage::Custom1(name, GMValue::F32(base)) if name == "add_base" => {
                self.base += base;
            }
            GMMessage::Custom1(name, GMValue::F32(speed)) if name == "add_speed" => {
                self.speed += speed;
            }
            GMMessage::Custom1(name, GMValue::F32(offset)) if name == "add_offset" => {
                self.offset += offset;
            }
            _ => {
                error_panic(&format!("Wrong message for GMTEScale::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        let result = object_manager.send_message(&self.target, GMMessage::Custom0("get_char_count".to_string()), context);

        if let GMValue::USize(num_of_chars) = result {
            let mut offset = 0.0;
            let mut new_scales = Vec::with_capacity(num_of_chars);

            for _ in 0..num_of_chars {
                new_scales.push(GMValue::F32(self.base + (self.amplitude * (self.time + offset).sin())));
                offset += self.offset;
            }

            object_manager.send_custom_message1(&self.target, "set_chars_scale", new_scales, context);
        }

        self.time += self.speed;
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
