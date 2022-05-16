

use std::fmt::Debug;

use crate::math::GMVec2D;
use crate::object::{GMObjectT, GMObjectAction};
use crate::message::{GMMessage, GMMessageData, GMSender, GMReceiver};
use crate::timer::GMTimer;
use crate::context::{GMUpdateContext, GMDrawContext};
use crate::error::GMError;


pub trait GMParentActionT : Debug {
    // Must be implemented:
    fn clone_box(&self) -> Box<dyn GMParentActionT>;

    // May be implemented:
    fn update(&mut self, child: &mut Box<dyn GMObjectT>, context: &mut GMUpdateContext) {
        child.update(context);
    }

    fn draw(&self, child: &Box<dyn GMObjectT>, context: &mut GMDrawContext) {
        child.draw(context);
    }

    fn send_message(&mut self, child: &mut Box<dyn GMObjectT>, message: GMMessage, context: &mut GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        child.send_message(message, context)
    }
}

impl Clone for Box<dyn GMParentActionT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}


#[derive(Clone, Debug)]
pub struct GMParentBase {
    active: bool,
    child: Box<dyn GMObjectT>,
    action: Box<dyn GMParentActionT>,
}

impl GMParentBase {
    pub fn new(child: Box<dyn GMObjectT>, action: Box<dyn GMParentActionT>) -> Self {
        Self {
            active: true,
            child,
            action,
        }
    }
}

impl GMObjectT for  GMParentBase {
    fn update(&mut self, context: &mut GMUpdateContext) {
        if self.active {
            self.action.update(&mut self.child, context);
        }
    }

    fn draw(&self, context: &mut GMDrawContext) {
        if self.active {
            self.action.draw(&self.child, context);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }

    fn get_child_ref(&self) -> Option<&Box<dyn GMObjectT>> {
        Some(&self.child)
    }

    fn get_child_mut(&mut self) -> Option<&mut Box<dyn GMObjectT>> {
        Some(&mut self.child)
    }

    fn send_message(&mut self, message: GMMessage, context: &mut GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        use GMMessageData::*;

        let sender = GMSender::CurrentObject;
        let receiver = message.sender.as_receiver();

        match message.message_data {
            SetChild(child) => {
                self.child = child;
            }
            GetChildClone => {
                return Ok(Some(GMMessage::new(sender, receiver, Child(Some(self.child.clone())))))
            }
            GetActive => {
                return Ok(Some(GMMessage::new(sender, receiver, Active(self.active))))
            }
            SetActive(active) => {
                self.active = active;
            }
            SetParentAction(action) => {
                self.action = action;
            }
            MessageToChild(message_data) => {
                let child_message = GMMessage::new(
                    message.sender, message.receiver, *message_data);
                return self.child.send_message(child_message, context)
            }
            _ => {
                return self.action.send_message(&mut self.child, message, context)
            }
        }

        Ok(None)
    }
}

#[derive(Clone)]
pub struct GMPActionFn {
    action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> (),
}

impl GMPActionFn {
    pub fn new(child: Box<dyn GMObjectT>, action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> ()) -> GMParentBase {
        let action_fn = Self {
            action
        };

        GMParentBase::new(child, Box::new(action_fn))
    }
}

impl Debug for GMPActionFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GMPActionFn")
    }
}

impl GMParentActionT for GMPActionFn {
    fn clone_box(&self) -> Box<dyn GMParentActionT> {
        Box::new(self.clone())
    }

    fn update(&mut self, child: &mut Box<dyn GMObjectT>, context: &mut GMUpdateContext) {
        (self.action)(child, context)
    }

    fn send_message(&mut self, child: &mut Box<dyn GMObjectT>, message: GMMessage, context: &mut GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        use GMMessageData::*;

        match message.message_data {
            SetObjectAction(GMObjectAction{ action }) => {
                self.action = action;
            }
            _ => {
                return child.send_message(message, context)
            }
        }

        Ok(None)
    }
}





#[derive(Debug, Clone)]
pub struct GMPCircular {
    active: bool,
    center: GMVec2D,
    radius: f32,
    angle: f32,
    angle_velocity: f32,
    child: Box<dyn GMObjectT>,
}

impl GMPCircular {
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

impl GMObjectT for GMPCircular {
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

    fn get_child_ref(&self) -> Option<&Box<dyn GMObjectT>> {
        Some(&self.child)
    }

    fn get_child_mut(&mut self) -> Option<&mut Box<dyn GMObjectT>> {
        Some(&mut self.child)
    }

    fn send_message(&mut self, message: GMMessage, context: &mut crate::GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        use GMMessageData::*;

        let sender = GMSender::CurrentObject;
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
                self.child = child;
            }
            GetChildClone => {
                return Ok(Some(GMMessage::new(sender, receiver, Child(Some(self.child.clone())))))
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
pub struct GMPTimer {
    timer: GMTimer,
    looping: bool,
    action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> (),
}

impl GMPTimer {
    pub fn new(duration: f32, child: Box<dyn GMObjectT>, action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> ()) -> GMParentBase {
        let timer = Self {
            timer: GMTimer::new(duration),
            looping: false,
            action,
        };

        GMParentBase::new(child, Box::new(timer))
    }

    pub fn new_looping(duration: f32, child: Box<dyn GMObjectT>, action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> ()) -> GMParentBase {
        let timer = Self {
            timer: GMTimer::new(duration),
            looping: true,
            action,
        };

        GMParentBase::new(child, Box::new(timer))
    }
}

impl GMParentActionT for GMPTimer {
    fn clone_box(&self) -> Box<dyn GMParentActionT> {
        Box::new(self.clone())
    }

    fn update(&mut self, child: &mut Box<dyn GMObjectT>, context: &mut GMUpdateContext) {
        if self.timer.finished() {
            (self.action)(child, context);

            if self.looping {
                self.timer.start();
            }
        }
    }

    fn send_message(&mut self, child: &mut Box<dyn GMObjectT>, message: GMMessage, context: &mut GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        use GMMessageData::*;

        let sender = GMSender::CurrentObject;
        let receiver = message.sender.as_receiver();

        match message.message_data {
            SetDuration(duration) => {
                self.timer.set_duration(duration);
            }
            GetDuration => {
                return Ok(Some(GMMessage::new(sender, receiver, Duration(self.timer.get_duration()))))
            }
            SetObjectAction(GMObjectAction{ action }) => {
                self.action = action;
            }
            _ => {
                return child.send_message(message, context)
            }
        }

        Ok(None)
    }
}

impl Debug for GMPTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GMParentTimer, timer: '{:?}', looping: '{}'", self.timer, self.looping)
    }
}




// TODO: Simplify
#[derive(Clone)]
pub struct GMPAnimationFinished {
    active: bool,
    child: Box<dyn GMObjectT>,
    action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> (),
}

impl GMPAnimationFinished {
    pub fn new(child: Box<dyn GMObjectT>, action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> ()) -> Self {
        Self {
            active: true,
            child,
            action,
        }
    }
}

impl GMObjectT for GMPAnimationFinished {
    fn update(&mut self, context: &mut GMUpdateContext) {
        if self.active {
            let result = self.child.send_message(GMMessage {
                sender: GMSender::ParentObject,
                receiver: GMReceiver::ChildObject,
                message_data: GMMessageData::GetAnimationStatus }, context);

            match result {
                Ok(Some(GMMessage { message_data: GMMessageData::AnimationStatus(status), .. })) => {
                    if status {
                        (self.action)(&mut self.child, context);
                    }
                }
                _ => {
                    panic!("GMPAnimationFinished::update(), unexpected message from child: {:?}", result);
                }
            }
        }

        self.child.update(context);
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }

    fn get_child_ref(&self) -> Option<&Box<dyn GMObjectT>> {
        Some(&self.child)
    }

    fn get_child_mut(&mut self) -> Option<&mut Box<dyn GMObjectT>> {
        Some(&mut self.child)
    }

    fn send_message(&mut self, message: GMMessage, context: &mut GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        use GMMessageData::*;

        let sender = GMSender::CurrentObject;
        let receiver = message.sender.as_receiver();

        match message.message_data {
            SetChild(child) => {
                self.child = child;
            }
            GetChildClone => {
                return Ok(Some(GMMessage::new(sender, receiver, Child(Some(self.child.clone())))))
            }
            GetActive => {
                return Ok(Some(GMMessage::new(sender, receiver, Active(self.active))))
            }
            SetActive(active) => {
                self.active = active;
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

impl Debug for GMPAnimationFinished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GMPAnimationFinished")
    }
}

