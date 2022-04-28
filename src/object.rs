use std::any::Any;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::GMError;
use crate::context::{GMUpdateContext, GMDrawContext};
use crate::math::GMVec2D;

pub enum GMObjectMessage {
    SetPosition(GMVec2D),
    AddPosition(GMVec2D),
    GetPosition(GMVec2D),

    Custom(Box<dyn Any>),
}

pub enum GMObjectAnswer {
    None,
    Position(GMVec2D),

    Custom(Box<dyn Any>),
}

pub trait GMObjectT {
    fn update(&mut self, context: &mut GMUpdateContext, object_manager: &GMObjectSender);

    fn draw(&self, context: &mut GMDrawContext);

    fn send_message(&mut self, message: Rc<GMObjectMessage>) -> GMObjectAnswer;
}

struct GMObjectWrapper {
    name: String,
    object: RefCell<Box<dyn GMObjectT>>,
    z_index: i32,
    groups: HashSet<String>,
}

impl GMObjectWrapper {
    fn new(name: &str, object: Box<dyn GMObjectT>) -> Self {
        Self {
            name: name.to_string(),
            object: RefCell::new(object),
            z_index: 0,
            groups: HashSet::new(),
        }
    }

    fn set_z_index(&mut self, z_index: i32) {
        self.z_index = z_index;
    }

    fn add_group(&mut self, group: &str) {
        self.groups.insert(group.to_string());
    }

    fn remove_group(&mut self, group: &str) {
        self.groups.remove(group);
    }

    fn update(&self, context: &mut GMUpdateContext, object_sender: &GMObjectSender) {
        if let Ok(mut object) = self.object.try_borrow_mut() {
            object.update(context, object_sender);
        }
    }

    fn draw(&self, context: &mut GMDrawContext) {
        if let Ok(object) = self.object.try_borrow_mut() {
            object.draw(context);
        }
    }

    fn send_message(&self, message: Rc<GMObjectMessage>) -> GMObjectAnswer {
        if let Ok(mut object) = self.object.try_borrow_mut() {
            object.send_message(message)
        } else {
            GMObjectAnswer::None
        }
    }

    fn send_group_message(&self, group: &str, message: Rc<GMObjectMessage>) -> GMObjectAnswer {
        if self.groups.contains(group) {
            self.send_message(message)
        } else {
            GMObjectAnswer::None
        }
    }
}

pub struct GMObjectSender {
    objects: Vec<GMObjectWrapper>,
}

impl GMObjectSender {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn get_index(&self, name: &str) -> Option<usize> {
        self.objects.iter().position(|object| object.name == name)
    }

    fn set_z_index(&mut self, name: &str, z_index: i32) -> Result<(), GMError> {
        match self.get_index(name) {
            Some(index) => {
                self.objects[index].set_z_index(z_index);
                Ok(())
            }
            None => {
                Err(GMError::ObjectAlreadyExists(name.to_string()))
            }
        }
    }

    fn add_group(&mut self, name: &str, group: &str) -> Result<(), GMError> {
        match self.get_index(name) {
            Some(index) => {
                self.objects[index].add_group(group);
                Ok(())
            }
            None => {
                Err(GMError::ObjectAlreadyExists(name.to_string()))
            }
        }
    }

    fn remove_group(&mut self, name: &str, group: &str) -> Result<(), GMError> {
        match self.get_index(name) {
            Some(index) => {
                self.objects[index].remove_group(group);
                Ok(())
            }
            None => {
                Err(GMError::ObjectAlreadyExists(name.to_string()))
            }
        }
    }

    fn add_object(&mut self, name: &str, object: Box<dyn GMObjectT>) -> Result<(), GMError> {
        match self.get_index(name) {
            Some(_) => {
                Err(GMError::ObjectAlreadyExists(name.to_string()))
            }
            None => {
                self.objects.push(GMObjectWrapper::new(name, object));
                Ok(())
            }
        }
    }

    fn remove_object(&mut self, name: &str) -> Result<(), GMError> {
        match self.get_index(name) {
            Some(index) => {
                self.objects.swap_remove(index);
                Ok(())
            }
            None => {
                Err(GMError::ObjectAlreadyExists(name.to_string()))
            }
        }
    }

    fn replace_object(&mut self, name: &str, object: Box<dyn GMObjectT>) -> Result<(), GMError> {
        match self.get_index(name) {
            Some(index) => {
                self.objects[index] = GMObjectWrapper::new(name, object);
                Ok(())
            }
            None => {
                Err(GMError::ObjectAlreadyExists(name.to_string()))
            }
        }
    }

    fn update(&self, context: &mut GMUpdateContext) {
        for object in self.objects.iter() {
            object.update(context, self);
        }
    }


    fn draw(&mut self, context: &mut GMDrawContext) {
        // Sort all objects by z order before drawing them
        self.objects.sort_by_key(|object| object.z_index);

        for object in self.objects.iter() {
            object.draw(context);
        }
    }

    pub fn send_message(&self, receiver: &str, message: GMObjectMessage) -> Result<GMObjectAnswer, GMError> {
        match self.get_index(receiver) {
            Some(index) => {
                let message = Rc::new(message);
                Ok(self.objects[index].send_message(message))
            }
            None => {
                Err(GMError::ObjectNotFound(receiver.to_string()))
            }
        }
    }

    pub fn send_group_message(&self, group: &str, message: GMObjectMessage) -> Vec<GMObjectAnswer> {
        let mut results = Vec::new();
        let message = Rc::new(message);

        for object in self.objects.iter() {
            results.push(object.send_group_message(group, message.clone()))
        }

        results
    }
}

pub struct GMObjectManager {
    object_sender: GMObjectSender,
}

impl GMObjectManager {
    pub fn new() -> Self {
        Self {
            object_sender: GMObjectSender::new(),
        }
    }

    pub fn set_z_index(&mut self, name: &str, z_index: i32) -> Result<(), GMError> {
        self.object_sender.set_z_index(name, z_index)
    }

    pub fn add_group(&mut self, name: &str, group: &str) -> Result<(), GMError> {
        self.object_sender.add_group(name, group)
    }

    pub fn remove_group(&mut self, name: &str, group: &str) -> Result<(), GMError> {
        self.object_sender.remove_group(name, group)
    }

    pub fn add_object<O: 'static + GMObjectT>(&mut self, name: &str, object: O) -> Result<(), GMError> {
        self.add_object_box(name, Box::new(object))
    }

    pub fn add_object_box(&mut self, name: &str, object: Box<dyn GMObjectT>) -> Result<(), GMError> {
        self.object_sender.add_object(name, object)
    }

    pub fn remove_object(&mut self, name: &str) -> Result<(), GMError> {
        self.object_sender.remove_object(name)
    }

    pub fn replace_object<O: 'static + GMObjectT>(&mut self, name: &str, object: O) -> Result<(), GMError> {
        self.replace_object_box(name, Box::new(object))
    }

    pub fn replace_object_box(&mut self, name: &str, object: Box<dyn GMObjectT>) -> Result<(), GMError> {
        self.object_sender.replace_object(name, object)
    }

    pub fn update(&self, context: &mut GMUpdateContext) {
        self.object_sender.update(context)
    }

    pub fn draw(&mut self, context: &mut GMDrawContext) {
        self.object_sender.draw(context)
    }

    pub fn send_message(&self, receiver: &str, message: GMObjectMessage) -> Result<GMObjectAnswer, GMError> {
        self.object_sender.send_message(receiver, message)
    }

    pub fn send_group_message(&self, group: &str, message: GMObjectMessage) -> Vec<GMObjectAnswer> {
        self.object_sender.send_group_message(group, message)
    }
}
