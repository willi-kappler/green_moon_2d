

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

        self.child.update(context);
        todo!();
    }

    fn send_message(&mut self, message: GMMessage, context: &mut crate::GMUpdateContext) -> Result<GMMessage, crate::GMError> {

        self.child.send_message(message, context);
        todo!()
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

    fn take_child(&self) -> Option<Box<dyn GMObjectT>> {
        Some(self.child.clone_box())
    }

    // Delegate methods:

    fn draw(&self, context: &mut crate::GMDrawContext) {
        self.child.draw(context);
    }

    fn get_name(&self) -> &str {
        self.child.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.child.set_name(name)
    }

    fn get_z_index(&self) -> i32 {
        self.child.get_z_index()
    }

    fn set_z_index(&mut self, z_index: i32) {
        self.child.set_z_index(z_index)
    }

    fn get_active(&self) -> bool {
        self.child.get_active()
    }

    fn set_active(&mut self, active: bool) {
        self.child.set_active(active)
    }

    fn get_position(&self) -> GMVec2D {
        self.child.get_position()
    }

    fn get_next_position(&self) -> GMVec2D {
        self.calc_position(self.angle + self.angle_velocity)
    }

    fn is_in_group(&self, group: &str) -> bool {
        self.child.is_in_group(group)
    }

    fn add_group(&mut self, group: &str) {
        self.child.add_group(group)
    }

    fn remove_group(&mut self, group: &str) {
        self.child.remove_group(group)
    }

    fn get_property(&self, name: &str) -> Option<&crate::property::GMValue> {
        self.child.get_property(name)
    }

    fn has_property(&self, name: &str) -> bool {
        self.child.has_property(name)
    }

    fn add_property(&mut self, name: &str, value: crate::property::GMValue) {
        self.child.add_property(name, value)
    }

    fn add_tag(&mut self, name: &str) {
        self.child.add_tag(name)
    }

    fn remove_property(&mut self, name: &str) {
        self.child.remove_property(name)
    }

    fn set_child(&mut self, child: Box<dyn GMObjectT>) {
        self.child = child;
    }
}
