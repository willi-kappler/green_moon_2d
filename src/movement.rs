
use crate::object::GMObjectT;
use crate::message::GMMessage;
use crate::value::GMValue;
use crate::target::GMTarget;
use crate::object_manager::GMObjectManager;
use crate::math::{GMVec2D, GMCircle};
use crate::context::GMContext;
use crate::util::error_panic;

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

}

impl GMMVTwoPoints {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl GMObjectT for GMMVTwoPoints {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
