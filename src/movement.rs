

use crate::context::GMContext;
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
pub struct GMMVCircle {
    pub target: GMTarget,
    pub circle: GMCircle,
    pub angle: f32,
}

impl GMMVCircle {
    pub fn new<T: Into<GMTarget>>(target: T, center: GMVec2D, radius: f32) -> Self {
        let target = target.into();
        let circle = GMCircle::new(center, radius);

        Self {
            target,
            circle,
            angle: 0.0,
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
            GMMessage::Custom0(name) if name == "update" => {
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
            _ => {
                error_panic(&format!("Wrong message for GMMVCircle::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

pub struct GMMVPath {
    pub target: GMTarget,
}

