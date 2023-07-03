
use std::fmt;

use log::debug;

use crate::curve::GMCurveT;
use crate::interpolation::GMInterpolateVec2D;
use crate::math::{GMVec2D, GMCircle};
use crate::message::{GMMessage, msgt1v, msgt0v};
use crate::object_manager::GMObjectManager;
use crate::object::{GMObjectT, GMObjectBox};
use crate::target::GMTarget;
use crate::util::{error_panic, send_message_f32, send_message_bool, send_message_usize};
use crate::value::GMValue;
use crate::context::GMContext;

#[derive(Clone, Debug)]
pub struct GMMVVelocity {
    pub target: GMTarget,
    pub v: GMVec2D,
    pub message: GMMessage,
}

impl GMMVVelocity {
    pub fn new<T: Into<GMTarget>, V: Into<GMVec2D>>(target: T, v: V) -> Self {
        let target = target.into();
        let v = v.into();
        debug!("GMMVVelocity::new(), target: '{:?}', v: '{:?}'", target, v);

        Self {
            target,
            v,
            message: msgt0v("position", "set"),
        }
    }

    pub fn update_object(&mut self, object: &mut GMObjectBox, object_manager: &GMObjectManager) {
        self.message.set_value(self.v);
        object.send_message(self.message.clone(), object_manager);
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
            "message" => {
                let new_message = value.into_message();
                return self.message.send_message(new_message);
            }
            _ => {
                error_panic(&format!("GMMVVelocity::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        self.message.set_value(self.v);
        object_manager.send_message(&self.target, self.message.clone());
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMVAcceleration {
    pub target: GMTarget,
    pub a: GMVec2D,
    pub message: GMMessage,
}

impl GMMVAcceleration {
    pub fn new<T: Into<GMTarget>, A: Into<GMVec2D>>(target: T, a: A) -> Self {
        let target = target.into();
        let a = a.into();
        debug!("GMMVAcceleration::new(), target: '{:?}', a: '{:?}'", target, a);

        Self {
            target,
            a,
            message: msgt0v("veclocity", "add"),
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
            "message" => {
                let new_message = value.into_message();
                return self.message.send_message(new_message);
            }
            _ => {
                error_panic(&format!("GMMVAcceleration::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        self.message.set_value(self.a);
        object_manager.send_message(&self.target, self.message.clone());
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMVVelAccel {
    pub target: GMTarget,
    pub v: GMVec2D,
    pub a: GMVec2D,
    pub message: GMMessage,
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
            message: msgt0v("position", "add"),
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
            "message" => {
                let new_message = value.into_message();
                return self.message.send_message(new_message);
            }
            _ => {
                error_panic(&format!("GMMVVelAccel::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        self.message.set_value(self.v);
        object_manager.send_message(&self.target, self.message.clone());
        self.v += self.a;
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMCircleBase {
    pub circle: GMCircle,
    pub angle: f32,
    pub auto_update: bool,
}

impl GMObjectT for GMCircleBase {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "circle" => {
                return self.circle.send_message(message);
            }
            "angle" => {
                return send_message_f32(&mut self.angle, method, message.value);
            }
            "auto_update" => {
                return send_message_bool(&mut self.auto_update, method, message.value);
            }
            _ => {
                error_panic(&format!("GMCircleBase::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMVCircle {
    pub target: GMTarget,
    pub message: GMMessage,
    pub base: GMCircleBase,
}

impl GMMVCircle {
    pub fn new<T: Into<GMTarget>, U: Into<GMVec2D>>(target: T, center: U, radius: f32) -> Self {
        let target = target.into();
        let circle = GMCircle::new(center, radius);
        debug!("GMMVCircle::new(), target: '{:?}', center: '{:?}', radius: '{:?}'", target, circle.center, circle.radius);

        Self {
            target,
            message: msgt0v("position", "set"),
            base: GMCircleBase { circle, angle: 0.0, auto_update: true },
        }
    }

    pub fn update_pos(&mut self, object_manager: &GMObjectManager) {
        let new_pos = self.base.circle.position_from_deg(self.base.angle);
        self.message.set_value(new_pos);
        object_manager.send_message(&self.target, self.message.clone());
    }
}

impl GMObjectT for GMMVCircle {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "" => {
                match method {
                    "update" => {
                        self.update_pos(object_manager);
                    }
                    _ => {
                        error_panic(&format!("GMMVCircle::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, message.value);
            }
            "message" => {
                let new_message = message.value.into_message();
                return self.message.send_message(new_message);
            }
            _ => {
                message.pre_tag(tag);
                return self.base.send_message(message, object_manager);
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        if self.base.auto_update {
            self.update_pos(object_manager);
        }
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct GMMVCircleFunc {
    pub base: GMCircleBase,
    pub func: fn(position: GMVec2D, object_manager: &GMObjectManager),
}

impl GMMVCircleFunc {
    pub fn new<U: Into<GMVec2D>>(center: U, radius: f32, func: fn(position: GMVec2D, object_manager: &GMObjectManager)) -> Self {
        let circle = GMCircle::new(center, radius);
        debug!("GMMVCircleFunc::new(), enter: '{:?}', radius: '{:?}'", circle.center, circle.radius);

        Self {
            base: GMCircleBase { circle, angle: 0.0, auto_update: true },
            func,
        }
    }

    pub fn update_pos(&self, object_manager: &GMObjectManager) {
        let new_pos = self.base.circle.position_from_deg(self.base.angle);
        (self.func)(new_pos, object_manager);
    }
}

impl fmt::Debug for GMMVCircleFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMMVMultiCircleFunc, center: '{}', radius: '{}'", self.base.circle.center, self.base.circle.radius)
    }
}

impl GMObjectT for GMMVCircleFunc {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "" => {
                match method {
                    "update" => {
                        self.update_pos(object_manager);
                    }
                    "set_func" => {
                        self.func = message.value.into_generic::<fn(position: GMVec2D, object_manager: &GMObjectManager)>();
                    }
                    _ => {
                        error_panic(&format!("GMMVCircleFunc::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            _ => {
                message.pre_tag(tag);
                return self.base.send_message(message, object_manager);
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        if self.base.auto_update {
            self.update_pos(object_manager);
        }
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct GMMVMultiCircle {
    pub angle_step: f32,
    pub count: usize,
    pub base: GMCircleBase,
    pub func: fn(value: Vec<GMVec2D>, object_manager: &GMObjectManager),
}

impl GMMVMultiCircle {
    pub fn new<T: Into<GMVec2D>>(center: T, radius: f32, angle_step: f32, count: usize, func: fn(value: Vec<GMVec2D>,
        object_manager: &GMObjectManager)) -> Self {
        let circle = GMCircle::new(center, radius);
        debug!("GMMVMultiCircle::new(), center: '{:?}', radius: '{:?}', angle_step: '{:?}', count: '{:?}'", circle.center, circle.radius, angle_step, count);

        Self {
            angle_step,
            count,
            base: GMCircleBase { circle, angle: 0.0, auto_update: true },
            func,
        }
    }

    pub fn multi_pos(&self) -> Vec<GMVec2D> {
        let mut result = Vec::with_capacity(self.count);
        let mut angle = self.base.angle;

        for _ in 0..self.count {
            result.push(self.base.circle.position_from_deg(angle));
            angle += self.angle_step;
        }

        result
    }

    pub fn update_pos(&self, object_manager: &GMObjectManager) {
        let positions = self.multi_pos();
        (self.func)(positions, object_manager);
    }
}

impl fmt::Debug for GMMVMultiCircle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMMVMultiCircle, center: '{}', radius: '{}', angle_step: '{}', count: '{}'",
        self.base.circle.center, self.base.circle.radius, self.angle_step, self.count)
    }
}

impl GMObjectT for GMMVMultiCircle {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "" => {
                match method {
                    "update" => {
                        self.update_pos(object_manager);
                    }
                    "set_func" => {
                        self.func = message.value.into_generic::<fn(value: Vec<GMVec2D>, object_manager: &GMObjectManager)>();
                    }
                    _ => {
                        error_panic(&format!("GMMVMultiCircle::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "angle_step" => {
                return send_message_f32(&mut self.angle_step, method, message.value);
            }
            "count" => {
                return send_message_usize(&mut self.count, method, message.value);
            }
            _ => {
                message.pre_tag(tag);
                return self.base.send_message(message, object_manager);
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        if self.base.auto_update {
            self.update_pos(object_manager);
        }
    }

    fn clone_box(&self) -> GMObjectBox {
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
    pub message: GMMessage,
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
            message: msgt0v("position", "set"),
        }
    }

    pub fn update_position(&mut self, object_manager: &GMObjectManager) {
        self.interpolation.update();
        let position = self.interpolation.get_current_value();
        self.message.set_value(position);
        object_manager.send_message(&self.target, self.message.clone());

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
                        let index = message.value.into_usize();
                        let position: GMValue = self.positions[index].0.into();
                        let speed: GMValue = self.positions[index].1.into();

                        return position.chain(speed);
                    }
                    "set_tuple_at" => {
                        let (index, position, speed) = message.value.into_generic::<(usize, GMVec2D, f32)>();
                        self.positions[index] = (position, speed);
                    }
                    "set_positions" => {
                        self.positions = message.value.into_generic::<Vec<(GMVec2D, f32)>>();
                    }
                    "set_curve" => {
                        self.interpolation.curve = message.value.into_generic::<Box<dyn GMCurveT>>();
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
                        error_panic(&format!("GMMVPath::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, message.value);
            }
            "index" => {
                return send_message_usize(&mut self.index, method, message.value);
            }
            "auto_update" => {
                return send_message_bool(&mut self.auto_update, method, message.value);
            }
            "repeat" => {
                return send_message_bool(&mut self.repeat, method, message.value);
            }
            "position" => {
                match message.value {
                    GMValue::USize(index) => {
                        // No other value needed
                        return self.positions[index].0.send_message(method, GMValue::None);
                    }
                    GMValue::Any(any) => {
                        let (index, new_value) = any.downcast_ref::<(usize, GMValue)>().unwrap().clone();

                        return self.positions[index].0.send_message(method, new_value);
                    }
                    _ => {
                        error_panic(&format!("GMPath::send_message, tag: 'position', invalid value: '{:?}'", message.value));
                    }
                }
            }
            "speed" => {
                match message.value {
                    GMValue::USize(index) => {
                        // No other value needed
                        return send_message_f32(&mut self.positions[index].1, method, GMValue::None);
                    }
                    GMValue::Any(any) => {
                        let (index, new_value) = any.downcast_ref::<(usize, GMValue)>().unwrap().clone();

                        return send_message_f32(&mut self.positions[index].1, method, new_value);
                    }
                    _ => {
                        error_panic(&format!("GMPath::send_message, tag: 'speed', invalid value: '{:?}'", message.value));
                    }
                }
            }
            "message" => {
                let new_message = message.value.into_message();
                return self.message.send_message(new_message);
            }
            _ => {
                error_panic(&format!("GMMVPath::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        if self.auto_update {
            self.update_position(object_manager);
        }
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMVFollow {
    pub target: GMTarget,
    pub source: GMTarget,
    pub interpolation: GMInterpolateVec2D,
    pub message: GMMessage,
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
            message: msgt0v("position", "set"),
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
                        error_panic(&format!("GMMVFollow::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, message.value);
            }
            "source" => {
                return self.source.send_message(method, message.value);
            }
            "interpolation" => {
                return self.interpolation.send_message(message);
            }
            "message" => {
                let new_message = message.value.into_message();
                return self.message.send_message(new_message);
            }
            _ => {
                error_panic(&format!("GMMVFollow::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        self.interpolation.update();
        let pos = self.interpolation.get_current_value();
        self.message.set_value(pos);
        object_manager.send_message(&self.target, self.message.clone());
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}
