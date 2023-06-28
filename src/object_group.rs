

use crate::object::{GMObjectT, GMObjectBox};
use crate::value::GMValue;



#[derive(Clone, Debug)]
pub struct GMObjectGroup {
    pub objects: Vec<GMObjectBox>,
}

impl GMObjectGroup {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    // TODO: add_object, remove_object, ...
}

impl GMObjectT for GMObjectGroup {
    fn send_message(&mut self, _message: crate::message::GMMessage, _object_manager: &crate::object_manager::GMObjectManager) -> crate::value::GMValue {
        GMValue::None
    }

    fn update(&mut self, _object_manager: &crate::object_manager::GMObjectManager, _context: &mut crate::GMContext) {
    }

    fn draw(&self, _context: &mut crate::GMContext) {
    }

    fn clone_box(&self) -> GMObjectBox {
        todo!()
    }
}
