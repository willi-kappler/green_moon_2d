

use delegate::delegate;

use crate::math::GMVec2D;
use crate::object::{GMObjectT, GMObjectBase};
use crate::message::{GMMessage, GMMessageData, GMSender, GMReceiver};
use crate::timer::GMTimer;


#[derive(Debug, Clone)]
pub struct GMParentCircular {
    center: GMVec2D,
    radius: f32,
    angle: f32,
    angle_velocity: f32,
    child: Box<dyn GMObjectT>,
}

impl GMParentCircular {
    pub fn new(center: GMVec2D, radius: f32, start_angle: f32, angle_velocity: f32, child: Box<dyn GMObjectT>) -> Self {
        Self {
            center,
            radius,
            angle: start_angle,
            angle_velocity,
            child,
        }
    }

    pub fn calc_position(&self, angle: f32) -> GMVec2D {
        todo!();
    }
}

impl GMObjectT for GMParentCircular {
    fn update(&mut self, context: &mut crate::GMUpdateContext) {
        self.angle += self.angle_velocity;
        self.child.set_position(self.calc_position(self.angle));
        self.child.update(context);
        todo!();
    }

    fn send_message(&mut self, message: GMMessage, context: &mut crate::GMUpdateContext) -> Result<GMMessage, crate::GMError> {
        // TODO: handle messages send to Self

        self.child.send_message(message, context)
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
        self.calc_position(self.angle + self.angle_velocity)
    }

    fn take_child(&self) -> Option<Box<dyn GMObjectT>> {
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
            fn set_name(&mut self, name: &str);
            fn get_z_index(&self) -> i32;
            fn set_z_index(&mut self, z_index: i32);
            fn get_active(&self) -> bool;
            fn set_active(&mut self, active: bool);
            fn get_position(&self) -> GMVec2D;
            fn is_in_group(&self, group: &str) -> bool;
            fn add_group(&mut self, group: &str);
            fn remove_group(&mut self, group: &str);
            fn get_property(&self, name: &str) -> Option<&crate::property::GMValue>;
            fn has_property(&self, name: &str) -> bool;
            fn add_property(&mut self, name: &str, value: crate::property::GMValue);
            fn add_tag(&mut self, name: &str);
            fn remove_property(&mut self, name: &str);
        }
    }
}
