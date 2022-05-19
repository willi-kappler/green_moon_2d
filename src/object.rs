
use std::fmt::Debug;
use std::collections::HashMap;

use crate::context::{GMContext};
use crate::message::{GMObjectMessage, GMObjectReply, GMObjectManagerMessage, GMMessageReplyTo};


pub trait CloneBox {
    fn clone_box(&self) -> Box<dyn GMObjectT>;
}

impl<T> CloneBox for T where T: Clone + GMObjectT + 'static {
    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn GMObjectT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait GMObjectT: Debug + CloneBox {
    fn send_message(&mut self, _message: GMObjectMessage, _context: &mut GMContext) -> GMObjectReply {
        GMObjectReply::Empty
    }
}

pub struct GMObjectManager {
    objects: HashMap<String, Box<dyn GMObjectT>>,
}

impl GMObjectManager {
    pub(crate) fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn add<O: 'static + GMObjectT>(&mut self, name: &str, object: O) {
        self.add_box(name, Box::new(object))
    }

    fn add_box(&mut self, name: &str, object: Box<dyn GMObjectT>) {
        if self.objects.contains_key(name) {
            panic!("Object with name '{}' already exists!", name);
        } else {
            self.objects.insert(name.to_string(), object);
        }
    }

    fn remove_object(&mut self, name: &str) {
        todo!();
    }

    fn replace_object(&mut self, name: &str, object: Box<dyn GMObjectT>) {
        todo!();
    }

    fn set_parent(&mut self, name: &str, parent: Box<dyn GMObjectT>) {

    }

    fn get_clone(&mut self, name: &str, reply_to: GMMessageReplyTo) {
        todo!();
    }

    fn message_to_object(&mut self, name: &str, message: GMObjectMessage, reply_to: GMMessageReplyTo) {
        todo!();
    }

    pub(crate) fn update(&mut self, context: &mut GMContext) {
        context.push_message_reply();

        for (name, object) in self.objects.iter_mut() {
            context.reply_to_object(name);
            object.send_message(GMObjectMessage::Update, context);
        }

        while let Some(message) = context.next_object_message() {
            self.send_message(message, context);
        }

        context.pop_message_reply();
    }

    fn send_message(&mut self, message: GMObjectManagerMessage, context: &mut GMContext) {
        use GMObjectManagerMessage::*;

        match message {
           AddObject(name, object) => {
               self.add_box(&name, object);
           }
           RemoveObject(name) => {
               self.remove_object(&name);
           }
           ReplaceObject(name, object) => {
               self.replace_object(&name, object);
           }
           SetParent(name, object) => {
               self.set_parent(&name, object);
           }
           GetClone(name, reply_to) => {
               self.get_clone(&name, reply_to);
           }
           MessageToObject(name, message, reply_to) => {
               self.message_to_object(&name, message, reply_to);
           }
        }
    }
}
