
use std::fmt;

use log::debug;

use crate::context::GMContext;
use crate::curve::GMCurveT;
use crate::interpolation::GMInterpolateVec2D;
use crate::math::{GMVec2D, GMCircle};
use crate::message::{GMMessage, msgt1v, msgt0v};
use crate::object_manager::GMObjectManager;
use crate::object::GMObjectT;
use crate::target::GMTarget;
use crate::util::{error_panic, send_message_f32, send_message_bool, send_message_usize};
use crate::value::GMValue;

#[derive(Clone, Debug)]
pub struct GMMVVelocity {
    pub target: GMTarget,
    pub v: GMVec2D,
}

impl GMMVVelocity {
    pub fn new<T: Into<GMTarget>, V: Into<GMVec2D>>(target: T, v: V) -> Self {
        let target = target.into();
        let v = v.into();
        debug!("GMMVVelocity::new(), target: '{:?}', v: '{:?}'", target, v);

        Self{
            target,
            v,
        }
    }
}

impl GMObjectT for GMMVVelocity {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method;
        let value = message.value;

        match tag.as_str() {
            "target" => {
                return self.target.send_message(&method, value);
            }
            "velocity" => {
                return self.v.send_message(&method, value);
            }
            _ => {
                error_panic(&format!("GMMVVelocity::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        object_manager.send_message(&self.target, msgt1v("position", "add", self.v));
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMVAcceleration {
    pub target: GMTarget,
    pub a: GMVec2D,
}

impl GMMVAcceleration {
    pub fn new<T: Into<GMTarget>, A: Into<GMVec2D>>(target: T, a: A) -> Self {
        let target = target.into();
        let a = a.into();
        debug!("GMMVAcceleration::new(), target: '{:?}', a: '{:?}'", target, a);

        Self {
            target,
            a,
        }
    }
}

impl GMObjectT for GMMVAcceleration {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method;
        let value = message.value;

        match tag.as_str() {
            "target" => {
                return self.target.send_message(&method, value);
            }
            "acceleration" => {
                return self.a.send_message(&method, value);
            }
            _ => {
                error_panic(&format!("GMMVAcceleration::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        object_manager.send_message(&self.target, msgt1v("velocity", "add", self.a));
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMVVelAccel {
    pub target: GMTarget,
    pub v: GMVec2D,
    pub a: GMVec2D,
}

impl GMMVVelAccel {
    pub fn new<T: Into<GMTarget>, U: Into<GMVec2D>, V: Into<GMVec2D>>(target: T, v: U, a: V) -> Self {
        let target = target.into();
        let v = v.into();
        let a = a.into();
        debug!("GMMVAcceleration::new(), target: '{:?}', v: '{:?}', a: '{:?}'", target, v, a);

        Self{
            target,
            v,
            a,
        }
    }
}

impl GMObjectT for GMMVVelAccel {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method;
        let value = message.value;

        match tag.as_str() {
            "target" => {
                return self.target.send_message(&method, value);
            }
            "velocity" => {
                return self.v.send_message(&method, value);
            }
            "acceleration" => {
                return self.a.send_message(&method, value);
            }
            _ => {
                error_panic(&format!("GMMVVelAccel::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        object_manager.send_message(&self.target, msgt1v("position", "add", self.v));
        self.v += self.a;
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMVCircle {
    pub target: GMTarget,
    pub circle: GMCircle,
    pub angle: f32,
    pub auto_update: bool,
}

impl GMMVCircle {
    pub fn new<T: Into<GMTarget>, U: Into<GMVec2D>>(target: T, center: U, radius: f32) -> Self {
        let target = target.into();
        let circle = GMCircle::new(center, radius);
        debug!("GMMVCircle::new(), target: '{:?}', center: '{:?}', radius: '{:?}'", target, circle.center, circle.radius);

        Self {
            target,
            circle,
            angle: 0.0,
            auto_update: true,
        }
    }
}

impl GMObjectT for GMMVCircle {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value.clone();

        match tag.as_str() {
            "" => {
                match method {
                    "update" => {
                        let new_pos = self.circle.position_from_deg(self.angle);
                        object_manager.send_message(&self.target, msgt1v("position", "set", new_pos));
                    }
                    _ => {
                        error_panic(&format!("GMMVCircle::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, value);
            }
            "circle" => {
                return self.circle.send_message(message);
            }
            "angle" => {
                return send_message_f32(&mut self.angle, method, value);
            }
            "auto_update" => {
                return send_message_bool(&mut self.auto_update, method, value);
            }
            _ => {
                error_panic(&format!("GMMVCircle::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        if self.auto_update {
            let new_pos = self.circle.position_from_deg(self.angle);
            object_manager.send_message(&self.target, msgt1v("position", "set", new_pos));
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct GMMVMultiCircle {
    pub circle: GMCircle,
    pub angle: f32,
    pub angle_step: f32,
    pub count: usize,
    pub auto_update: bool,
    pub func: fn(value: Vec<GMVec2D>, object_manager: &GMObjectManager),
}

impl GMMVMultiCircle {
    pub fn new<T: Into<GMVec2D>>(center: T, radius: f32, angle_step: f32, count: usize, func: fn(value: Vec<GMVec2D>,
        object_manager: &GMObjectManager)) -> Self {
        let circle = GMCircle::new(center, radius);
        debug!("GMMVMultiCircle::new(), center: '{:?}', radius: '{:?}', angle_step: '{:?}', count: '{:?}'", circle.center, circle.radius, angle_step, count);

        Self {
            circle,
            angle: 0.0,
            angle_step,
            count,
            auto_update: true,
            func,
        }
    }

    pub fn multi_pos(&self) -> Vec<GMVec2D> {
        let mut result = Vec::with_capacity(self.count);
        let mut angle = self.angle;

        for _ in 0..self.count {
            result.push(self.circle.position_from_deg(angle));
            angle += self.angle_step;
        }

        result
    }
}

impl fmt::Debug for GMMVMultiCircle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMMVMultiCircle, center: '{}', radius: '{}', angle_step: '{}', count: '{}'", self.circle.center, self.circle.radius, self.angle_step, self.count)
    }
}

impl GMObjectT for GMMVMultiCircle {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value.clone();

        match tag.as_str() {
            "" => {
                match method {
                    "update" => {
                        let positions = self.multi_pos();
                        (self.func)(positions, object_manager);
                    }
                    "set_func" => {
                        self.func = value.into_generic::<fn(value: Vec<GMVec2D>, object_manager: &GMObjectManager)>();
                    }
                    _ => {
                        error_panic(&format!("GMMVCircle::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "circle" => {
                return self.circle.send_message(message);
            }
            "angle" => {
                return send_message_f32(&mut self.angle, method, value);
            }
            "angle_step" => {
                return send_message_f32(&mut self.angle_step, method, value);
            }
            "count" => {
                return send_message_usize(&mut self.count, method, value);
            }
            "auto_update" => {
                return send_message_bool(&mut self.auto_update, method, value);
            }
            _ => {
                error_panic(&format!("GMMVCircle::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        if self.auto_update {
            let positions = self.multi_pos();
            (self.func)(positions, object_manager);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMVPath {
    pub target: GMTarget,
    pub positions: Vec<(GMVec2D, f32)>,
    pub interpolation: GMInterpolateVec2D,
    pub index: usize,
    pub auto_update: bool,
    pub repeat: bool,
    // TODO: Add tag and method
}

impl GMMVPath {
    pub fn new<T: Into<GMTarget>>(target: T, positions: Vec<(GMVec2D, f32)>) -> Self {
        assert!(positions.len() > 2, "GMMVPath::new, at least three positions required");

        let start = positions[0].0;
        let end = positions[1].0;
        let speed = positions[0].1;
        let target = target.into();

        debug!("GMMVPath::new(), start: '{:?}', end: '{:?}', speed: '{:?}', target: '{:?}'", start, end, speed, target);

        Self {
            target,
            positions,
            interpolation: GMInterpolateVec2D::new(start, end, speed, 0.0),
            index: 0,
            auto_update: true,
            repeat: true,
        }
    }

    pub fn update_position(&mut self, object_manager: &GMObjectManager) {
        self.interpolation.update();
        let position = self.interpolation.get_current_value();
        object_manager.send_message(&self.target, msgt1v("position", "set", position));

        if self.interpolation.is_finished() {
            self.index += 1;

            if self.index >= self.positions.len() {
                if self.repeat {
                    self.index = 0;
                } else {
                    self.index = self.positions.len();
                    return
                }
            }

            let start = self.positions[self.index].0;
            let speed = self.positions[self.index].1;

            let end = if self.index < self.positions.len() - 1 {
                self.positions[self.index + 1].0
            } else {
                self.positions[0].0
            };

            self.interpolation.start = start;
            self.interpolation.end = end;
            self.interpolation.speed = speed;
            self.interpolation.calculate_diff();
            self.interpolation.reset();

        }
    }
}

impl GMObjectT for GMMVPath {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value.clone();

        match tag.as_str() {
            "" => {
                match method {
                    "update" => {
                        self.update_position(object_manager);
                    }
                    "init" => {
                        let position = self.interpolation.get_current_value();
                        object_manager.send_message(&self.target, msgt1v("position", "set", position));
                    }
                    "get_tuple_at" => {
                        let index = value.into_usize();
                        let position: GMValue = self.positions[index].0.into();
                        let speed: GMValue = self.positions[index].1.into();

                        return position.chain(speed);
                    }
                    "set_tuple_at" => {
                        let mut values = value.to_vec_deque();
                        let index = values.pop_front().unwrap().into_usize();
                        let position = values.pop_front().unwrap().into_vec2d();
                        let speed = values.pop_front().unwrap().into_f32();

                        self.positions[index] = (position, speed);
                    }
                    "set_positions" => {
                        self.positions = value.into_generic::<Vec<(GMVec2D, f32)>>();
                    }
                    "set_curve" => {
                        self.interpolation.curve = value.into_generic::<Box<dyn GMCurveT>>();
                    }
                    "is_finished" => {
                        if self.repeat {
                            return false.into()
                        } else {
                            let result =  self.index == self.positions.len() && self.interpolation.is_finished();
                            return result.into()
                        }

                    }
                    _ => {
                        error_panic(&format!("GMMVCircle::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, value);
            }
            "index" => {
                return send_message_usize(&mut self.index, method, value);
            }
            "auto_update" => {
                return send_message_bool(&mut self.auto_update, method, value);
            }
            "repeat" => {
                return send_message_bool(&mut self.repeat, method, value);
            }
            "position" => {
                match value {
                    GMValue::USize(index) => {
                        // No other value needed
                        return self.positions[index].0.send_message(method, GMValue::None);
                    }
                    GMValue::Multiple(mut values) => {
                        let index = values.pop_front().unwrap().into_usize();
                        let new_value = values.pop_front().unwrap();

                        return self.positions[index].0.send_message(method, new_value);
                    }
                    _ => {
                        error_panic(&format!("GMPath::send_message, tag: 'position', invalid value: '{:?}'", value));
                    }
                }
            }
            "speed" => {
                match value {
                    GMValue::USize(index) => {
                        // No other value needed
                        return send_message_f32(&mut self.positions[index].1, method, GMValue::None);
                    }
                    GMValue::Multiple(mut values) => {
                        let index = values.pop_front().unwrap().into_usize();
                        let new_value = values.pop_front().unwrap();

                        return send_message_f32(&mut self.positions[index].1, method, new_value);
                    }
                    _ => {
                        error_panic(&format!("GMPath::send_message, tag: 'speed', invalid value: '{:?}'", value));
                    }
                }
            }
            _ => {
                error_panic(&format!("GMMVCircle::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        if self.auto_update {
            self.update_position(object_manager);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMVFollow {
    pub target: GMTarget,
    pub source: GMTarget,
    pub interpolation: GMInterpolateVec2D,
}

impl GMMVFollow {
    pub fn new<E: Into<GMTarget>, F: Into<GMTarget>, U: Into<GMVec2D>>(target: E, source: F, speed: f32, start: U) -> Self {
        let target = target.into();
        let source = source.into();
        let start = start.into();
        debug!("GMMVFollow::new(), target: '{:?}', source: '{:?}', speed: '{:?}', start: '{:?}'", target, source, speed, start);

        Self {
            target,
            source,
            interpolation: GMInterpolateVec2D::new(start, (0.0, 0.0).into(), speed, 0.0),
        }
    }

    pub fn update_source(&mut self, object_manager: &GMObjectManager) {
        let value = object_manager.send_message(&self.source, msgt0v("position", "get"));
        let new_end = value.into_vec2d();

        let new_start = self.interpolation.get_current_value();
        self.interpolation.start = new_start;
        self.interpolation.end = new_end;
        self.interpolation.calculate_diff();
        self.interpolation.reset();
    }
}

impl GMObjectT for GMMVFollow {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value.clone();

        match tag.as_str() {
            "" => {
                match method {
                    "init" => {
                        self.update_source(object_manager);
                    }
                    "update_source" => {
                        self.update_source(object_manager);
                    }
                    _ => {
                        error_panic(&format!("GMMVCircle::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, value);
            }
            "source" => {
                return self.source.send_message(method, value);
            }
            "interpolation" => {
                return self.interpolation.send_message(message);
            }
            _ => {
                error_panic(&format!("GMMVCircle::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        self.interpolation.update();
        let pos = self.interpolation.get_current_value();
        object_manager.send_message(&self.target, msgt1v("position", "set", pos));
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

