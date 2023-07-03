
use std::fmt::Debug;
use std::collections::VecDeque;

use crate::context::GMContext;
use crate::value::GMValue;
use crate::object_manager::GMObjectManager;
use crate::message::GMMessage;

pub type GMObjectBox = Box<dyn GMObjectT>;


pub trait GMObjectT: Debug {
    fn send_message(&mut self, _message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        GMValue::None
    }

    fn send_message_multiple(&mut self, mut messages: Vec<GMMessage>, object_manager: &GMObjectManager) -> VecDeque<GMValue> {
        let mut result = VecDeque::new();

        for message in messages.drain(0..) {
            result.push_back(self.send_message(message, object_manager));
        }

        result
    }

    fn update(&mut self, _object_manager: &GMObjectManager, _context: &mut GMContext) {
    }

    fn draw(&self, _context: &mut GMContext) {
    }

    fn clone_box(&self) -> GMObjectBox;
}

impl Clone for GMObjectBox {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<U: GMObjectT + 'static> From<U> for GMObjectBox {
    fn from(object: U) -> Self {
        Box::new(object)
    }
}

impl From<&dyn GMObjectT> for GMObjectBox {
    fn from(object: &dyn GMObjectT) -> Self {
        object.clone_box()
    }
}
