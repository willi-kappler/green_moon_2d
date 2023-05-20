
use std::fmt;

use crate::context::GMContext;
use crate::curve::GMCurveT;
use crate::interpolation::GMInterpolateVec2D;
use crate::math::{GMVec2D, GMCircle};
use crate::message::GMMessage;
use crate::object_manager::GMObjectManager;
use crate::object::GMObjectT;
use crate::target::GMTarget;
use crate::util::{error_panic};
use crate::value::GMValue;

#[derive(Clone, Debug)]
pub struct GMMVVelocity {
    pub target: GMTarget,
    pub v: GMVec2D,
}

impl GMMVVelocity {
    pub fn new<T: Into<GMTarget>>(target: T, v: GMVec2D) -> Self {
        let target = target.into();

        Self{
            target,
            v,
        }
    }
}

impl GMObjectT for GMMVVelocity {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::GetTarget => {
                return self.target.clone().into();
            }
            GMMessage::SetTarget(target) => {
                self.target = target;
            }
            GMMessage::Custom0(name) if name == "get_velocity" => {
                return self.v.into();
            }
            GMMessage::Custom1(name, GMValue::Vec2D(v)) if name == "set_velocity" => {
                self.v = v;
            }
            GMMessage::Custom1(name, GMValue::Vec2D(v)) if name == "add_velocity" => {
                self.v += v;
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            _ => {
                error_panic(&format!("Wrong message for GMMVVelocity::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        object_manager.send_message(&self.target, GMMessage::AddPosition(self.v), context);
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
    pub fn new<T: Into<GMTarget>>(target: T, a: GMVec2D) -> Self {
        let target = target.into();

        Self {
            target,
            a,
        }
    }
}

impl GMObjectT for GMMVAcceleration {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::GetTarget => {
                return self.target.clone().into();
            }
            GMMessage::SetTarget(target) => {
                self.target = target;
            }
            GMMessage::Custom0(name) if name == "get_acceleration" => {
                return self.a.into();
            }
            GMMessage::Custom1(name, GMValue::Vec2D(a)) if name == "set_acceleration" => {
                self.a = a;
            }
            GMMessage::Custom1(name, GMValue::Vec2D(a)) if name == "add_acceleration" => {
                self.a += a;
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            _ => {
                error_panic(&format!("Wrong message for GMMVAcceleration::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        object_manager.send_custom_message1(&self.target, "add_velocity", self.a, context);
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
    pub fn new<T: Into<GMTarget>>(target: T, v: GMVec2D, a: GMVec2D) -> Self {
        let target = target.into();

        Self{
            target,
            v,
            a,
        }
    }
}

impl GMObjectT for GMMVVelAccel {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::GetTarget => {
                return self.target.clone().into();
            }
            GMMessage::SetTarget(target) => {
                self.target = target;
            }
            GMMessage::Custom0(name) if name == "get_velocity" => {
                return self.v.into();
            }
            GMMessage::Custom0(name) if name == "get_acceleration" => {
                return self.a.into();
            }
            GMMessage::Custom1(name, GMValue::Vec2D(v)) if name == "set_velocity" => {
                self.v = v;
            }
            GMMessage::Custom1(name, GMValue::Vec2D(v)) if name == "add_velocity" => {
                self.v += v;
            }
            GMMessage::Custom1(name, GMValue::Vec2D(a)) if name == "set_acceleration" => {
                self.a = a;
            }
            GMMessage::Custom1(name, GMValue::Vec2D(a)) if name == "add_acceleration" => {
                self.a += a;
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            _ => {
                error_panic(&format!("Wrong message for GMMVVelAccel::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        object_manager.send_message(&self.target, GMMessage::AddPosition(self.v), context);
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

        Self {
            target,
            circle,
            angle: 0.0,
            auto_update: true,
        }
    }
}

impl GMObjectT for GMMVCircle {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::GetPosition => {
                return self.circle.center.into()
            }
            GMMessage::GetX => {
                return self.circle.center.x.into()
            }
            GMMessage::GetY => {
                return self.circle.center.y.into()
            }
            GMMessage::AddPosition(pos) => {
                self.circle.center += pos;
            }
            GMMessage::AddX(x) => {
                self.circle.center.x += x;
            }
            GMMessage::AddY(y) => {
                self.circle.center.y += y;
            }
            GMMessage::SetPosition(pos) => {
                self.circle.center = pos;
            }
            GMMessage::SetX(x) => {
                self.circle.center.x = x;
            }
            GMMessage::SetY(y) => {
                self.circle.center.y = y;
            }
            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::SetTarget(target) => {
                self.target = target;
            }
            GMMessage::Update => {
                let new_pos = self.circle.position_from_deg(self.angle);
                object_manager.send_message(&self.target, GMMessage::SetPosition(new_pos), context);
            }
            GMMessage::Custom0(name) if name == "get_radius" => {
                return self.circle.radius.into()
            }
            GMMessage::Custom1(name, GMValue::F32(radius)) if name == "set_radius" => {
                self.circle.radius = radius;
            }
            GMMessage::Custom0(name) if name == "get_angle" => {
                return self.angle.into()
            }
            GMMessage::Custom1(name, GMValue::F32(angle)) if name == "set_angle" => {
                self.angle = angle;
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            _ => {
                error_panic(&format!("Wrong message for GMMVCircle::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        if self.auto_update {
            let new_pos = self.circle.position_from_deg(self.angle);
            object_manager.send_message(&self.target, GMMessage::SetPosition(new_pos), context);
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
    pub func: fn(value: Vec<GMVec2D>, context: &mut GMContext, object_manager: &GMObjectManager),
}

impl GMMVMultiCircle {
    pub fn new<T: Into<GMVec2D>>(center: T, radius: f32, angle_step: f32, count: usize, func: fn(value: Vec<GMVec2D>,
        context: &mut GMContext, object_manager: &GMObjectManager)) -> Self {
        let circle = GMCircle::new(center, radius);

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
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::GetPosition => {
                return self.circle.center.into()
            }
            GMMessage::GetMultiPosition => {
                return self.multi_pos().into()
            }
            GMMessage::GetX => {
                return self.circle.center.x.into()
            }
            GMMessage::GetY => {
                return self.circle.center.y.into()
            }
            GMMessage::AddPosition(pos) => {
                self.circle.center += pos;
            }
            GMMessage::AddX(x) => {
                self.circle.center.x += x;
            }
            GMMessage::AddY(y) => {
                self.circle.center.y += y;
            }
            GMMessage::SetPosition(pos) => {
                self.circle.center = pos;
            }
            GMMessage::SetX(x) => {
                self.circle.center.x = x;
            }
            GMMessage::SetY(y) => {
                self.circle.center.y = y;
            }
            GMMessage::Update => {
                let positions = self.multi_pos();
                (self.func)(positions, context, object_manager);
            }
            GMMessage::Custom0(name) if name == "get_radius" => {
                return self.circle.radius.into()
            }
            GMMessage::Custom1(name, GMValue::F32(radius)) if name == "set_radius" => {
                self.circle.radius = radius;
            }
            GMMessage::Custom0(name) if name == "get_angle" => {
                return self.angle.into()
            }
            GMMessage::Custom1(name, GMValue::F32(angle)) if name == "set_angle" => {
                self.angle = angle;
            }
            GMMessage::Custom0(name) if name == "get_angle_step" => {
                return self.angle_step.into()
            }
            GMMessage::Custom1(name, GMValue::F32(angle_step)) if name == "set_angle_step" => {
                self.angle_step = angle_step;
            }
            GMMessage::Custom0(name) if name == "get_count" => {
                return self.count.into()
            }
            GMMessage::Custom1(name, GMValue::USize(count)) if name == "set_count" => {
                self.count = count;
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_func" => {
                let func = *value.downcast::<fn(value: Vec<GMVec2D>, context: &mut GMContext,
                    object_manager: &GMObjectManager)>().unwrap();
                self.func = func;
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            _ => {
                error_panic(&format!("Wrong message for GMMVMultiCircle::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        if self.auto_update {
            let positions = self.multi_pos();
            (self.func)(positions, context, object_manager);
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
}

impl GMMVPath {
    pub fn new<T: Into<GMTarget>>(target: T, positions: Vec<(GMVec2D, f32)>) -> Self {
        assert!(positions.len() > 2, "GMMVPath::new, at least three positions required");

        let start = positions[0].0;
        let end = positions[1].0;
        let speed = positions[0].1;

        Self {
            target: target.into(),
            positions,
            interpolation: GMInterpolateVec2D::new(start, end, speed, 0.0),
            index: 0,
            auto_update: true,
            repeat: true,
        }
    }

    pub fn update_position(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        self.interpolation.update();
        let position = self.interpolation.get_current_value();
        object_manager.send_message(&self.target, GMMessage::SetPosition(position), context);

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
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Init => {
                let position = self.interpolation.get_current_value();
                object_manager.send_message(&self.target, GMMessage::SetPosition(position), context);
            }
            GMMessage::GetTarget => {
                return self.target.clone().into();
            }
            GMMessage::SetTarget(target) => {
                self.target = target;
            }
            GMMessage::Custom0(name) if name == "get_repeat" => {
                return self.repeat.into();
            }
            GMMessage::Custom1(name, GMValue::Bool(repeat)) if name == "set_repeat" => {
                self.repeat = repeat;
            }
            GMMessage::Custom0(name) if name == "toggle_repeat" => {
                self.repeat = !self.repeat;
            }
            GMMessage::Custom0(name) if name == "get_auto_update" => {
                return self.auto_update.into();
            }
            GMMessage::Custom1(name, GMValue::Bool(auto_update)) if name == "set_auto_update" => {
                self.auto_update = auto_update;
            }
            GMMessage::Custom0(name) if name == "toggle_auto_update" => {
                self.auto_update = !self.auto_update;
            }
            GMMessage::Custom0(name) if name == "get_index" => {
                return self.index.into();
            }
            GMMessage::Custom1(name, GMValue::USize(index)) if name == "set_index" => {
                self.index = index;
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_curve" => {
                let curve = (*value.downcast::<Box<dyn GMCurveT>>().unwrap()).clone();
                self.interpolation.curve = curve;
            }
            GMMessage::Custom1(name, GMValue::USize(index)) if name == "get_position_at" => {
                return self.positions[index].0.into();
            }
            GMMessage::Custom1(name, GMValue::USize(index)) if name == "get_speed_at" => {
                return self.positions[index].1.into();
            }
            GMMessage::Custom1(name, GMValue::USize(index)) if name == "get_tuple_at" => {
                let value = self.positions[index];
                let values = vec![GMValue::Vec2D(value.0), GMValue::F32(value.1)];
                return values.into();
            }
            GMMessage::Custom2(name, GMValue::Vec2D(position), GMValue::USize(index)) if name == "set_position_at" => {
                self.positions[index].0 = position;
            }
            GMMessage::Custom2(name, GMValue::F32(speed), GMValue::USize(index)) if name == "set_speed_at" => {
                self.positions[index].1 = speed;
            }
            GMMessage::Custom2(name, GMValue::Multiple(mut values), GMValue::USize(index)) if name == "set_tuple_at" => {
                let value1 = values.pop().unwrap();
                let value0 = values.pop().unwrap();

                if let (GMValue::Vec2D(position), GMValue::F32(speed)) = (value0, value1) {
                    self.positions[index].0 = position;
                    self.positions[index].1 = speed;
                }
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_positions" => {
                let positions = (*value.downcast::<Vec<(GMVec2D, f32)>>().unwrap()).clone();
                self.positions = positions;
            }
            GMMessage::Update =>{
                self.update_position(context, object_manager);
            }
            GMMessage::Custom0(name) if name == "is_finished" => {
                if self.repeat {
                    return false.into()
                } else {
                    let result =  self.index == self.positions.len() && self.interpolation.is_finished();
                    return result.into()
                }
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            _ => {
                error_panic(&format!("Wrong message for GMMVPath::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        if self.auto_update {
            self.update_position(context, object_manager);
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
        Self {
            target: target.into(),
            source: source.into(),
            interpolation: GMInterpolateVec2D::new(start.into(), (0.0, 0.0).into(), speed, 0.0),
        }
    }

    pub fn update_source(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        let value = object_manager.send_message(&self.source, GMMessage::GetPosition, context);

        if let GMValue::Vec2D(new_end) = value {
            let new_start = self.interpolation.get_current_value();
            self.interpolation.start = new_start;
            self.interpolation.end = new_end;
            self.interpolation.calculate_diff();
            self.interpolation.reset();
        }
    }
}

impl GMObjectT for GMMVFollow {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Init => {
                self.update_source(context, object_manager);
            }
            GMMessage::GetTarget => {
                return self.target.clone().into();
            }
            GMMessage::SetTarget(target) => {
                self.target = target;
            }
            GMMessage::Custom0(name) if name == "get_source" => {
                return self.source.clone().into();
            }
            GMMessage::Custom1(name, GMValue::Target(source)) if name == "set_source" => {
                self.source = source;
            }
            GMMessage::Custom0(name) if name == "get_speed" => {
                return self.interpolation.speed.into();
            }
            GMMessage::Custom1(name, GMValue::F32(speed)) if name == "set_speed" => {
                self.interpolation.speed = speed;
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_curve" => {
                let curve = (*value.downcast::<Box<dyn GMCurveT>>().unwrap()).clone();
                self.interpolation.curve = curve;
            }
            GMMessage::Custom0(name) if name == "update_source" => {
                self.update_source(context, object_manager);
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            _ => {
                error_panic(&format!("Wrong message for GMMVFollow::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        self.interpolation.update();
        let pos = self.interpolation.get_current_value();
        object_manager.send_message(&self.target, GMMessage::SetPosition(pos), context);
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

