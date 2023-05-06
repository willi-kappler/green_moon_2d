
use std::rc::Rc;

use crate::context::GMContext;
use crate::interpolation::{GMInterpolateVec2D, GMInterpolateF32, GMCurveT};
use crate::math::{GMVec2D, GMCircle};
use crate::message::GMMessage;
use crate::object_manager::GMObjectManager;
use crate::object::GMObjectT;
use crate::target::GMTarget;
use crate::util::{error_panic, GMRepetition};
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
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
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
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
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
            _ => {
                error_panic(&format!("Wrong message for GMMVAcceleration::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        object_manager.send_custom_message1(&self.target, "add_velocity", self.a.into(), context);
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
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
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
pub struct GMMVTwoPoints {
    pub target: GMTarget,
    pub interpolation: GMInterpolateVec2D,
    pub auto_update: bool,
}

impl GMMVTwoPoints {
    pub fn new<T: Into<GMTarget>>(target: T, pos1: GMVec2D, pos2: GMVec2D, speed: f32) -> Self {
        let target = target.into();
        let interpolation = GMInterpolateVec2D::new(pos1, pos2, speed, 0.0);

        Self {
            target,
            interpolation,
            auto_update: true,
        }
    }
}

impl GMObjectT for GMMVTwoPoints {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::GetTarget => {
                return self.target.clone().into();
            }
            GMMessage::SetTarget(target) => {
                self.target = target;
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            GMMessage::GetChildCount => {
                return GMValue::USize(2)
            }
            GMMessage::ToChild(n, child_message) => {
                match *child_message {
                    GMMessage::GetPosition => {
                        if n == 0 {
                            return self.interpolation.start.into()
                        } else {
                            return self.interpolation.end.into()
                        }
                    }
                    GMMessage::SetPosition(position) => {
                        if n == 0 {
                            self.interpolation.start = position;
                        } else {
                            self.interpolation.end = position;
                        }
                    }
                    GMMessage::AddPosition(position) => {
                        if n == 0 {
                            self.interpolation.start += position;
                        } else {
                            self.interpolation.end += position;
                        }
                    }
                    GMMessage::GetX => {
                        if n == 0 {
                            return self.interpolation.start.x.into()
                        } else {
                            return self.interpolation.end.x.into()
                        }
                    }
                    GMMessage::GetY => {
                        if n == 0 {
                            return self.interpolation.start.y.into()
                        } else {
                            return self.interpolation.end.y.into()
                        }
                    }
                    GMMessage::SetX(x) => {
                        if n == 0 {
                            self.interpolation.start.x = x;
                        } else {
                            self.interpolation.end.x = x;
                        }
                    }
                    GMMessage::SetY(y) => {
                        if n == 0 {
                            self.interpolation.start.y = y;
                        } else {
                            self.interpolation.end.y = y;
                        }
                    }
                    GMMessage::AddX(x) => {
                        if n == 0 {
                            self.interpolation.start.x += x;
                        } else {
                            self.interpolation.end.x += x;
                        }
                    }
                    GMMessage::AddY(y) => {
                        if n == 0 {
                            self.interpolation.start.y += y;
                        } else {
                            self.interpolation.end.y += y;
                        }
                    }
                    _ => {
                        error_panic(&format!("Wrong message for GMMVTwoPoints::send_message: child message: {:?}", child_message))
                    }
                }
            }
            GMMessage::ToAllChildren(child_message) => {
                match *child_message {
                    GMMessage::GetPosition => {
                        let positions = vec![self.interpolation.start, self.interpolation.end];
                        return positions.into()
                    }
                    GMMessage::AddPosition(position) => {
                        self.interpolation.start += position;
                        self.interpolation.end += position;
                    }
                    GMMessage::GetX => {
                        let x_values = vec![self.interpolation.start.x, self.interpolation.end.x];
                        return x_values.into()
                    }
                    GMMessage::GetY => {
                        let y_values = vec![self.interpolation.start.y, self.interpolation.end.y];
                        return y_values.into()
                    }
                    GMMessage::SetX(x) => {
                        self.interpolation.start.x = x;
                        self.interpolation.end.x = x;
                    }
                    GMMessage::SetY(y) => {
                        self.interpolation.start.y = y;
                        self.interpolation.end.y = y;
                    }
                    GMMessage::AddX(x) => {
                        self.interpolation.start.x += x;
                        self.interpolation.end.x += x;
                    }
                    GMMessage::AddY(y) => {
                        self.interpolation.start.y += y;
                        self.interpolation.end.y += y;
                    }
                    _ => {
                        error_panic(&format!("Wrong message for GMMVTwoPoints::send_message: child message: {:?}", child_message))
                    }
                }
            }
            GMMessage::Custom0(name) if name == "get_start" => {
                return self.interpolation.start.into()
            }
            GMMessage::Custom0(name) if name == "get_end" => {
                return self.interpolation.end.into()
            }
            GMMessage::Custom0(name) if name == "get_speed" => {
                return self.interpolation.speed.into()
            }
            GMMessage::Custom0(name) if name == "get_step" => {
                return self.interpolation.current_step.into()
            }
            GMMessage::Custom0(name) if name == "get_value" => {
                return self.interpolation.get_current_value().into()
            }
            GMMessage::Custom0(name) if name == "get_repetition" => {
                return GMValue::Any(Rc::new(self.interpolation.repetition))
            }
            GMMessage::Custom0(name) if name == "get_curve" => {
                return GMValue::Any(Rc::new(self.interpolation.curve.clone()))
            }
            GMMessage::Custom0(name) if name == "reset" => {
                self.interpolation.reset();
            }
            GMMessage::Custom0(name) if name == "is_finished" => {
                return self.interpolation.is_finished().into()
            }
            GMMessage::Custom0(name) if name == "update" => {
                self.interpolation.update();
                let new_pos = self.interpolation.get_current_value();
                object_manager.send_message(&self.target, GMMessage::SetPosition(new_pos), context);
            }
            GMMessage::Custom1(name, GMValue::Vec2D(start)) if name == "set_start" => {
                self.interpolation.start = start;
            }
            GMMessage::Custom1(name, GMValue::Vec2D(end)) if name == "set_end" => {
                self.interpolation.end = end;
            }
            GMMessage::Custom1(name, GMValue::F32(speed)) if name == "set_speed" => {
                self.interpolation.speed = speed;
            }
            GMMessage::Custom1(name, GMValue::F32(step)) if name == "set_step" => {
                self.interpolation.current_step = step;
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_repetition" => {
                let repetition = value.downcast::<GMRepetition>().unwrap();
                self.interpolation.repetition = (*repetition).clone();
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_curve" => {
                let curve = value.downcast::<Box<dyn GMCurveT>>().unwrap();
                self.interpolation.curve = (*curve).clone();
            }
            _ => {
                error_panic(&format!("Wrong message for GMMVTwoPoints::send_message: {:?}", message))
            }
        }
        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        if self.auto_update {
            self.interpolation.update();
            let new_pos = self.interpolation.get_current_value();
            object_manager.send_message(&self.target, GMMessage::SetPosition(new_pos), context);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMVCircle {
    pub target: GMTarget,
    pub interpolation: GMInterpolateF32,
    pub circle: GMCircle,
    pub auto_update: bool,
    pub child: Option<Box<dyn GMObjectT>>,
}

impl GMMVCircle {
    pub fn new<T: Into<GMTarget>>(target: T, center: GMVec2D, radius: f32) -> Self {
        let target = target.into();
        let interpolation = GMInterpolateF32::new(0.0, 360.0, 1.0, 0.0);
        let circle = GMCircle::new(center, radius);

        Self {
            target,
            interpolation,
            circle,
            auto_update: true,
            child: None,
        }
    }

    pub fn set_child(&mut self) {

    }
}

impl GMObjectT for GMMVCircle {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            _ => {
                error_panic(&format!("Wrong message for GMMVCircle::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        if self.auto_update {
            self.interpolation.update();
            let angle = self.interpolation.get_current_value();
            let new_position = self.circle.position_from_deg(angle);
            let message = GMMessage::SetPosition(new_position);

            if let Some(child) = &mut self.child {
                child.send_message(message, context, object_manager);
            } else {
                object_manager.send_message(&self.target, message, context);
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
