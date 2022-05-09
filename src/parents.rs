

use delegate::delegate;

use crate::math::GMVec2D;
use crate::object::{GMObjectT, GMObjectBase};
use crate::message::{GMMessage, GMMessageData, GMSender, GMReceiver, GMMessageFactory};
use crate::timer::GMTimer;


#[derive(Debug, Clone)]
pub struct GMParentCircular {
    center: GMVec2D,
    radius: f32,
    angle: f32,
    angle_velocity: f32,
    child: Box<dyn GMObjectT>,
    message_factory: GMMessageFactory,
}

impl GMParentCircular {
    pub fn new(center: GMVec2D, radius: f32, start_angle: f32, angle_velocity: f32, child: Box<dyn GMObjectT>) -> Self {
        let name = child.get_name().to_string();
        let sender = GMSender::Object(name);

        Self {
            center,
            radius,
            angle: start_angle,
            angle_velocity,
            child,
            message_factory: GMMessageFactory::new(sender),
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
        self.angle += self.angle_velocity;
        self.child.set_position(self.calc_position(self.angle));
        self.child.update(context);
    }

    fn send_message(&mut self, message: GMMessage, context: &mut crate::GMUpdateContext) {
        use GMMessageData::*;

        match message.message_data {
            SetPosition(position) => {
                self.center = position;
            }
            // GetPosition is handled by child
            SetRadius(radius) => {
                self.radius = radius;
            }
            GetRadius => {
                let receiver = message.receiver;
                let message_data = Radius(self.radius);
                self.message_factory.sender_from_object(&self.child);
                self.message_factory.send_to(receiver, message_data, context);
            }
            SetAngle(angle) => {
                self.angle = angle;
            }
            GetAngle => {
                let receiver = message.receiver;
                let message_data = Angle(self.radius);
                self.message_factory.sender_from_object(&self.child);
                self.message_factory.send_to(receiver, message_data, context);
            }
            SetChild(child) => {
                self.set_child(child);
            }
            GetChildClone => {
                let receiver = message.receiver;
                let message_data = Child(self.get_child());
                self.message_factory.sender_from_object(&self.child);
                self.message_factory.send_to(receiver, message_data, context);
            }
            _ => {
                self.child.send_message(message, context)
            }
        }
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

    // Delegate methods:
    delegate! {
        to self.child {
            fn draw(&self, context: &mut crate::GMDrawContext);
            fn get_name(&self) -> &str;
            fn set_name(&self, name: &str);
            fn get_z_index(&self) -> i32;
            fn set_z_index(&mut self, z_index: i32);
            fn get_active(&self) -> bool;
            fn set_active(&mut self, active: bool);
            fn get_position(&self) -> GMVec2D;
            fn get_property(&self, name: &str) -> Option<&crate::property::GMValue>;
            fn has_property(&self, name: &str) -> bool;
            fn add_property(&mut self, name: &str, value: crate::property::GMValue);
            fn add_tag(&mut self, name: &str);
            fn remove_property(&mut self, name: &str);
        }
    }
}
