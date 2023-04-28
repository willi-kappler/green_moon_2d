
use std::f32::consts::TAU;

use log::debug;

use crate::context::GMContext;
use crate::object::{GMObjectT, GMMessage, GMValue, GMTarget, GMObjectManager};
use crate::util::error_panic;

#[derive(Debug, Clone)]
pub struct GMTEWave {
    pub target: GMTarget,
    pub amplitude: f32,
    pub speed: f32,
    pub offset: f32,
    time: f32,
}

impl GMTEWave {
    pub fn new<T:Into<GMTarget>>(target: T, amplitude: f32, speed: f32, offset: f32) -> Self {
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
            GMMessage::Custom1(name) if name == "get_ampitude" => {
                return self.amplitude.into()
            }
            GMMessage::Custom1(name) if name == "get_speed" => {
                return self.speed.into()
            }
            GMMessage::Custom1(name) if name == "get_offset" => {
                return self.offset.into()
            }
            GMMessage::Custom2(name, GMValue::F32(value)) if name == "set_amplitude" => {
                self.amplitude = value
            }
            GMMessage::Custom2(name, GMValue::F32(value)) if name == "set_speed" => {
                self.speed = value
            }
            GMMessage::Custom2(name, GMValue::F32(value)) if name == "set_offset" => {
                self.offset = value
            }
            _ => {
                error_panic(&format!("Wrong message for GMTEWave::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        let mut offset = 0.0;

        let message1 = GMMessage::Custom1("get_horizontal".to_string());
        let message2 = GMMessage::GetChildCount;

        let result = object_manager.send_message(&self.target, (message1, message2).into(), context);

        if let GMValue::Tuple2(value1, value2) = result {
            if let GMValue::Bool(horizontal) = *value1 {
                if let GMValue::USize(num_of_chars) = *value2 {
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
