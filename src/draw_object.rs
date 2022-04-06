


use std::any::Any;

use crate::movement::GMMovementInner;


pub trait GMDrawT {
    fn update(&mut self) {
    }

    fn draw(&self);

    fn get_z_index(&self) -> i32 {
        0
    }

    fn box_clone(&self) -> Box<dyn GMDrawT>;

    fn get_movement_inner(&self) -> &GMMovementInner;

    fn get_movement_inner_mut(&mut self) -> &mut GMMovementInner;

    fn set_property(&mut self, name: &str, value: &dyn Any);

    fn get_property(&self, name: &str) -> &dyn Any;

    fn get_property_mut(&mut self, name: &str) -> &mut dyn Any;

    fn send_message(&mut self, message: &str);
}
