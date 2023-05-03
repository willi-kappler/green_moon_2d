
use std::fmt::Debug;

use crate::context::GMContext;
use crate::value::GMValue;
use crate::object_manager::GMObjectManager;
use crate::message::GMMessage;


// TODO: Add pre-processing for messages: position, active, visible, ...

pub trait GMObjectT: Debug {
    fn send_message(&mut self, _message: GMMessage, _context: &mut GMContext, _object_manager: &GMObjectManager) -> GMValue {
        GMValue::None
    }

    fn send_multi_message(&mut self, messages: Vec<GMMessage>, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        let mut result = Vec::new();

        for message in messages.iter() {
            result.push(self.send_message(message.clone(), context, object_manager));
        }

        return result.into()
    }

    fn update(&mut self, _context: &mut GMContext, _object_manager: &GMObjectManager) {
    }

    fn draw(&self, _context: &mut GMContext) {
    }

    fn clone_box(&self) -> Box<dyn GMObjectT>;
}

impl Clone for Box<dyn GMObjectT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<U: GMObjectT + 'static> From<U> for Box<dyn GMObjectT> {
    fn from(object: U) -> Self {
        Box::new(object)
    }
}

impl From<&dyn GMObjectT> for Box<dyn GMObjectT> {
    fn from(object: &dyn GMObjectT) -> Self {
        object.clone_box()
    }
}
