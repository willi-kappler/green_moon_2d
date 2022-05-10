

use std::fmt::Debug;

use delegate::delegate;

use crate::{GMError, GMUpdateContext};
use crate::math::GMVec2D;
use crate::object::GMObjectT;
use crate::message::{GMMessage, GMMessageData, GMMessageFactory};
use crate::timer::GMTimer;


#[derive(Debug, Clone)]
pub struct GMParentCircular {
    active: bool,
    center: GMVec2D,
    radius: f32,
    angle: f32,
    angle_velocity: f32,
    child: Box<dyn GMObjectT>,
}

impl GMParentCircular {
    pub fn new(center: GMVec2D, radius: f32, start_angle: f32, angle_velocity: f32, child: Box<dyn GMObjectT>) -> Self {
        Self {
            active: true,
            center,
            radius,
            angle: start_angle,
            angle_velocity,
            child,
        }
    }

    pub fn calc_position(&self, angle: f32) -> GMVec2D {
        let x = self.center.x + (self.radius * angle.cos());
        let y = self.center.y + (self.radius * angle.sin());
        GMVec2D::new(x, y)
    }
}

impl GMObjectT for GMParentCircular {
    fn update(&mut self, context: &mut crate::GMUpdateContext) {
        if self.active {
            self.angle += self.angle_velocity;
            self.child.set_position(self.calc_position(self.angle));
        }

        self.child.update(context);
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }

    fn set_position(&mut self, position: GMVec2D) {
        self.center = position;
    }

    fn add_position(&mut self, position: &GMVec2D) {
        self.center.add2(position);
    }

    fn get_next_position(&self) -> GMVec2D {
        let current_pos = self.calc_position(self.angle);
        let new_pos = self.calc_position(self.angle + self.angle_velocity);
        let diff = new_pos - current_pos;
        self.child.get_next_position() + diff
    }

    fn get_child(&self) -> Option<Box<dyn GMObjectT>> {
        Some(self.child.clone_box())
    }

    fn set_child(&mut self, child: Box<dyn GMObjectT>) {
        self.child = child;
    }

    fn get_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    // Delegate methods:
    delegate! {
        to self.child {
            fn draw(&self, context: &mut crate::GMDrawContext);
            fn get_name(&self) -> &str;
            fn set_name(&self, name: &str);
            fn get_z_index(&self) -> i32;
            fn set_z_index(&mut self, z_index: i32);
            fn get_position(&self) -> GMVec2D;
            fn get_property(&self, name: &str) -> Option<&crate::property::GMValue>;
            fn has_property(&self, name: &str) -> bool;
            fn add_property(&mut self, name: &str, value: crate::property::GMValue);
            fn add_tag(&mut self, name: &str);
            fn remove_property(&mut self, name: &str);
        }
    }

    fn send_message(&mut self, message: GMMessage, context: &mut crate::GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        use GMMessageData::*;

        let message_factory = GMMessageFactory::new_sender_receiver(&self.child, &message.sender);

        match message.message_data {
            SetPosition(position) => {
                self.set_position(position);
                Ok(None)
            }
            AddPosition(position) => {
                self.add_position(&position);
                Ok(None)
            }
            // GetPosition is handled by child
            SetRadius(radius) => {
                self.radius = radius;
                Ok(None)
            }
            GetRadius => {
                Ok(Some(message_factory.create_data(Radius(self.radius))))
            }
            SetAngle(angle) => {
                self.angle = angle;
                Ok(None)
            }
            GetAngle => {
                Ok(Some(message_factory.create_data(Angle(self.radius))))
            }
            SetChild(child) => {
                self.set_child(child);
                Ok(None)
            }
            GetChildClone => {
                Ok(Some(message_factory.create_data(Child(self.get_child()))))
            }
            GetActive => {
                Ok(Some(message_factory.create_data(Active(self.active))))
            }
            SetActive(active) => {
                self.active = active;
                Ok(None)
            }
            MessageToChild(message_data) => {
                let child_message = GMMessage::new(
                    message.sender, message.receiver, *message_data);
                self.child.send_message(child_message, context)
            }
            _ => {
                self.child.send_message(message, context)
            }
        }
    }
}

#[derive(Clone)]
pub struct GMParentTimer {
    timer: GMTimer,
    looping: bool,
    child: Box<dyn GMObjectT>,
    action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> (),
}

impl GMParentTimer {
    pub fn new(duration: f32, child: Box<dyn GMObjectT>, action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> ()) -> Self {
        Self {
            timer: GMTimer::new(duration),
            looping: false,
            child,
            action,
        }
    }

    pub fn new_looping(duration: f32, child: Box<dyn GMObjectT>, action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> ()) -> Self {
        let mut result = Self::new(duration, child, action);
        result.looping = true;
        result
    }
}

impl GMObjectT for GMParentTimer {
    fn update(&mut self, context: &mut crate::GMUpdateContext) {
        if self.timer.finished() {
            (self.action)(&mut self.child, context);

            if self.looping {
                self.timer.start();
            }
        }

        self.child.update(context);
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }

    fn get_child(&self) -> Option<Box<dyn GMObjectT>> {
        Some(self.child.clone_box())
    }

    fn set_child(&mut self, child: Box<dyn GMObjectT>) {
        self.child = child;
    }

    fn get_active(&self) -> bool {
        self.timer.get_active()
    }

    fn set_active(&mut self, active: bool) {
        self.timer.set_active(active);
    }

// Delegate methods:
    delegate! {
        to self.child {
            fn draw(&self, context: &mut crate::GMDrawContext);
            fn get_name(&self) -> &str;
            fn set_name(&self, name: &str);
            fn get_z_index(&self) -> i32;
            fn set_z_index(&mut self, z_index: i32);
            fn get_position(&self) -> GMVec2D;
            fn set_position(&mut self, position: GMVec2D);
            fn add_position(&mut self, position: &GMVec2D);
            fn get_property(&self, name: &str) -> Option<&crate::property::GMValue>;
            fn has_property(&self, name: &str) -> bool;
            fn add_property(&mut self, name: &str, value: crate::property::GMValue);
            fn add_tag(&mut self, name: &str);
            fn remove_property(&mut self, name: &str);
        }
    }

    fn send_message(&mut self, message: GMMessage, context: &mut crate::GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        use GMMessageData::*;

        let message_factory = GMMessageFactory::new_sender_receiver(&self.child, &message.sender);

        match message.message_data {
            SetChild(child) => {
                self.set_child(child);
                Ok(None)
            }
            GetChildClone => {
                Ok(Some(message_factory.create_data(Child(self.get_child()))))
            }
            SetDuration(duration) => {
                self.timer.set_duration(duration);
                Ok(None)
            }
            GetDuration => {
                Ok(Some(message_factory.create_data(Duration(self.timer.get_duration()))))
            }
            GetActive => {
                Ok(Some(message_factory.create_data(Active(self.get_active()))))
            }
            SetActive(active) => {
                self.set_active(active);
                Ok(None)
            }
            MessageToChild(message_data) => {
                let child_message = GMMessage::new(
                    message.sender, message.receiver, *message_data);
                self.child.send_message(child_message, context)
            }
            _ => {
                self.child.send_message(message, context)
            }
        }
    }
}

impl Debug for GMParentTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GMParentTimer, timer: '{:?}', looping: '{}', child: '{:?}'", self.timer, self.looping, self.child)
    }
}
