

use std::fmt::Debug;

use crate::{GMError, GMUpdateContext};
use crate::math::GMVec2D;
use crate::object::{GMObjectT, GMObjectAction};
use crate::message::{GMMessage, GMMessageData, GMSender, GMReceiver};
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

    fn get_child_ref(&self) -> Option<&Box<dyn GMObjectT>> {
        Some(&self.child)
    }

    fn get_child_mut(&mut self) -> Option<&mut Box<dyn GMObjectT>> {
        Some(&mut self.child)
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

    fn send_message(&mut self, message: GMMessage, context: &mut crate::GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        use GMMessageData::*;

        let sender = self.child.as_sender();
        let receiver = message.sender.as_receiver();

        match message.message_data {
            SetPosition(position) => {
                self.set_position(position);
            }
            AddPosition(position) => {
                self.add_position(&position);
            }
            // GetPosition is handled by child
            SetRadius(radius) => {
                self.radius = radius;
            }
            GetRadius => {
                return Ok(Some(GMMessage::new(sender, receiver, Radius(self.radius))))
            }
            SetAngle(angle) => {
                self.angle = angle;
            }
            GetAngle => {
                return Ok(Some(GMMessage::new(sender, receiver, Angle(self.radius))))
            }
            SetChild(child) => {
                self.set_child(child);
            }
            GetChildClone => {
                return Ok(Some(GMMessage::new(sender, receiver, Child(self.get_child()))))
            }
            GetActive => {
                return Ok(Some(GMMessage::new(sender, receiver, Active(self.active))))
            }
            SetActive(active) => {
                self.active = active;
            }
            MessageToChild(message_data) => {
                let child_message = GMMessage::new(
                    message.sender, message.receiver, *message_data);
                return self.child.send_message(child_message, context)
            }
            _ => {
                return self.child.send_message(message, context)
            }
        }

        Ok(None)
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

    fn get_child_ref(&self) -> Option<&Box<dyn GMObjectT>> {
        Some(&self.child)
    }

    fn get_child_mut(&mut self) -> Option<&mut Box<dyn GMObjectT>> {
        Some(&mut self.child)
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

    fn send_message(&mut self, message: GMMessage, context: &mut crate::GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        use GMMessageData::*;

        let sender = self.child.as_sender();
        let receiver = message.sender.as_receiver();

        match message.message_data {
            SetChild(child) => {
                self.set_child(child);
            }
            GetChildClone => {
                return Ok(Some(GMMessage::new(sender, receiver, Child(self.get_child()))))
            }
            SetDuration(duration) => {
                self.timer.set_duration(duration);
            }
            GetDuration => {
                return Ok(Some(GMMessage::new(sender, receiver, Duration(self.timer.get_duration()))))
            }
            GetActive => {
                return Ok(Some(GMMessage::new(sender, receiver, Active(self.get_active()))))
            }
            SetActive(active) => {
                self.set_active(active);
            }
            SetObjectAction(GMObjectAction{ action }) => {
                self.action = action;
            }
            MessageToChild(message_data) => {
                let child_message = GMMessage::new(
                    message.sender, message.receiver, *message_data);
                return self.child.send_message(child_message, context)
            }
            _ => {
                return self.child.send_message(message, context)
            }
        }

        Ok(None)
    }
}

impl Debug for GMParentTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GMParentTimer, timer: '{:?}', looping: '{}', child: '{:?}'", self.timer, self.looping, self.child)
    }
}

#[derive(Clone)]
pub struct GMParentAnimationFinished {
    active: bool,
    child: Box<dyn GMObjectT>,
    action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> (),
}

impl GMParentAnimationFinished {
    pub fn new(child: Box<dyn GMObjectT>, action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> ()) -> Self {
        Self {
            active: true,
            child,
            action,
        }
    }
}

impl GMObjectT for GMParentAnimationFinished {
    fn update(&mut self, context: &mut GMUpdateContext) {
        if self.active {
            let result = self.child.send_message(GMMessage {
                sender: GMSender::ObjectParent,
                receiver: GMReceiver::ObjectChild,
                message_data: GMMessageData::GetAnimationStatus }, context);

            match result {
                Ok(Some(GMMessage { message_data: GMMessageData::AnimationStatus(status), .. })) => {
                    if status {
                        (self.action)(&mut self.child, context);
                    }
                }
                _ => {
                    panic!("GMParentAnimationFinished::update(), unexpected message from child: {:?}", result);
                }
            }
        }

        self.child.update(context);
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn get_active(&self) -> bool {
        self.active
    }

    fn set_child(&mut self, child: Box<dyn GMObjectT>) {
        self.child = child;
    }

    fn get_child(&self) -> Option<Box<dyn GMObjectT>> {
        Some(self.child.clone_box())
    }

    fn get_child_ref(&self) -> Option<&Box<dyn GMObjectT>> {
        Some(&self.child)
    }

    fn get_child_mut(&mut self) -> Option<&mut Box<dyn GMObjectT>> {
        Some(&mut self.child)
    }

    fn send_message(&mut self, message: GMMessage, context: &mut GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        use GMMessageData::*;

        let sender = self.child.as_sender();
        let receiver = message.sender.as_receiver();

        match message.message_data {
            SetChild(child) => {
                self.set_child(child);
            }
            GetChildClone => {
                return Ok(Some(GMMessage::new(sender, receiver, Child(self.get_child()))))
            }
            GetActive => {
                return Ok(Some(GMMessage::new(sender, receiver, Active(self.get_active()))))
            }
            SetActive(active) => {
                self.set_active(active);
            }
            SetObjectAction(GMObjectAction{ action }) => {
                self.action = action;
            }
            MessageToChild(message_data) => {
                let child_message = GMMessage::new(
                    message.sender, message.receiver, *message_data);
                return self.child.send_message(child_message, context)
            }
            _ => {
                return self.child.send_message(message, context)
            }
        }

        Ok(None)
    }
}

impl Debug for GMParentAnimationFinished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GMParentAnimationFinished, child: '{:?}'", self.child)
    }
}
