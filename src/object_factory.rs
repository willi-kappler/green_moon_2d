

use std::rc::Rc;

use crate::object::{GMObjectT, GMObjectBox};
use crate::message::GMMessage;
use crate::object_manager::GMObjectManager;
use crate::value::GMValue;
use crate::target::GMTarget;
use crate::util::error_panic;

#[derive(Debug, Clone)]
pub struct GMObjectFactory {
    // TODO: use GMObjectInfo!
    pub object: GMObjectBox,
    pub name_prefix: String,
    id: u64,
}

impl GMObjectFactory {
    pub fn new(object: GMObjectBox) -> Self {
        Self::new2(object, "")
    }

    pub fn new2(object: GMObjectBox, name_prefix: &str) -> Self {
        Self {
            object,
            name_prefix: name_prefix.to_string(),
            id: 0,
        }
    }

    fn new_object1(&mut self) -> (GMObjectBox, String) {
        let new_object = self.object.clone();
        let new_name = format!("{}_{}", self.name_prefix, self.id);
        self.id += 1;

        (new_object, new_name)
    }

    fn new_object2(&mut self, new_object: GMObjectBox, new_name: String) -> GMMessage {
        let value = GMValue::Any(Rc::new((new_name, new_object)));
        let message = GMMessage::new2("add_custom_object", value);

        message
    }

    fn new_object3(&mut self, messages: Vec<GMMessage>, object_manager: &GMObjectManager) -> GMMessage {
        let (mut new_object, new_name) = self.new_object1();
        new_object.send_message_multiple(messages, object_manager);
        self.new_object2(new_object, new_name)
    }

    fn new_object4(&mut self, message: GMMessage, object_manager: &GMObjectManager) {
        let target = GMTarget::ObjectManager;
        object_manager.send_message(&target, message);
    }

    pub fn new_object(&mut self, object_manager: &GMObjectManager) {
        let (new_object, new_name) = self.new_object1();
        let message = self.new_object2(new_object, new_name);
        self.new_object4(message, object_manager);
    }

    pub fn new_object_messages(&mut self, messages: Vec<GMMessage>, object_manager: &GMObjectManager) {
        let message = self.new_object3(messages, object_manager);
        self.new_object4(message, object_manager);
    }
}

impl GMObjectT for GMObjectFactory {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "" => {
                match method {
                    "set_object" => {
                        self.object = message.value.into_object();
                    }
                    "get_object" => {
                        return self.object.clone().into();
                    }
                    "new_object" => {
                        self.new_object(object_manager)
                    }
                    _ => {
                        error_panic(&format!("GMObjectFactory::send_message: Unknown method '{}', no tag", method));
                    }
                }
            }
            _ => {
                message.pre_tag(tag);
                return self.object.send_message(message, object_manager);
            }
        }

        GMValue::None
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}
