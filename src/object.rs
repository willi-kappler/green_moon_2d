
use std::fmt::Debug;
use std::collections::HashMap;

use log::debug;

use crate::context::{GMContext};
use crate::message::{GMObjectMessage, GMObjectReply, GMObjectManagerMessage, GMMessageReplyTo, GMSceneMessage};
use crate::property::GMValue;


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
    #[allow(unused_variables)]
    fn send_message(&mut self, message: GMObjectMessage, context: &mut GMContext) -> GMObjectReply {
        GMObjectReply::Empty
    }

    // The following trait methods may be implemented for performance.
    // But the default implementations are OK.
    fn update(&mut self, context: &mut GMContext) {
        self.send_message(GMObjectMessage::Update, context);
    }

    fn set_child(&mut self, child: Box<dyn GMObjectT>, context: &mut GMContext) {
        self.send_message(GMObjectMessage::SetChild(child), context);
    }

    fn add_property(&mut self, name: &str, value: GMValue, context: &mut GMContext) {
        self.send_message(GMObjectMessage::AddProperty(name.to_string(), value), context);
    }

    fn get_property(&mut self, name: &str, context: &mut GMContext) -> GMObjectReply {
        todo!();
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

    fn object_does_not_exist(&self, location: &str, name: &str) {
        panic!("{}, Object with name '{}' does not exist!", location, name);
    }

    fn add_box(&mut self, name: &str, object: Box<dyn GMObjectT>) {
        if self.objects.contains_key(name) {
            panic!("Object with name '{}' already exists!", name);
        } else {
            self.objects.insert(name.to_string(), object);
        }
    }

    fn remove_object(&mut self, name: &str) {
        if self.objects.remove(name).is_none() {
            self.object_does_not_exist("GMObjectManager::add_box()", name);
        }
    }

    fn replace_object(&mut self, name: &str, new_object: Box<dyn GMObjectT>) {
        if let Some(object) = self.objects.get_mut(name) {
            *object = new_object;
        } else {
            self.object_does_not_exist("GMObjectManager::replace_object()", name);
        }
    }

    fn clear(&mut self) {
        debug!("GMObjectManager::clear()");

        self.objects.clear();
    }

    fn set_parent(&mut self, name: &str, mut parent: Box<dyn GMObjectT>, context: &mut GMContext) {
        if let Some(child) = self.objects.remove(name) {
            parent.set_child(child, context);
            self.objects.insert(name.to_string(), parent);
        } else {
            self.object_does_not_exist("GMObjectManager::set_parent()", name);
        }
    }

    fn get_clone(&mut self, name: &str, reply_to: GMMessageReplyTo, context: &mut GMContext) {
        if let Some(object) = self.objects.get(name) {
            let new_object = object.clone();

            use GMMessageReplyTo::*;

            match reply_to {
                Object(reply_name) => {
                    context.message_to_object(&reply_name, GMObjectMessage::ClonedFrom(name.to_string(), new_object));
                }
                Scene => {
                    context.message_to_current_scene(GMSceneMessage::ClonedFrom(name.to_string(), new_object));
                }
            }
        } else {
            self.object_does_not_exist("GMObjectManager::get_clone()", name);
        }
    }

    fn message_to_object(&mut self, name: &str, message: GMObjectMessage, context: &mut GMContext) {
        if let Some(object) = self.objects.get_mut(name) {
            context.set_current_object(name);
            object.send_message(message, context);
        } else {
            self.object_does_not_exist("GMObjectManager::message_to_object()", name);
        }
    }

    pub(crate) fn update(&mut self, context: &mut GMContext) {
        for (name, object) in self.objects.iter_mut() {
            context.set_current_object(name);
            object.update(context);
        }

        while let Some(message) = context.next_object_message() {
            self.send_message(message, context);
        }
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
           Clear => {
               self.clear();
           }
           SetParent(name, object) => {
               self.set_parent(&name, object, context);
           }
           GetClone(name, reply_to) => {
               self.get_clone(&name, reply_to, context);
           }
           MessageToObject(name, message) => {
               self.message_to_object(&name, message, context);
           }
        }
    }
}
