use std::collections::HashSet;
use std::rc::Rc;

use crate::GMError;
use crate::context::{GMUpdateContext, GMDrawContext};
use crate::math::GMVec2D;
use crate::message::{GMObjectMessage, GMReceiver};


pub trait GMObjectT {
    fn get_name(&self) -> &str;

    fn set_active(&mut self, _active: bool) {
    }

    fn get_active(&self) -> bool {
        true
    }

    fn set_z_index(&mut self, _z_index: i32) {
    }

    fn get_z_index(&self) -> i32 {
        0
    }

    fn add_group(&mut self, _group: &str) {
    }

    fn remove_group(&mut self, _group: &str) {
    }

    fn is_in_group(&self, _group: &str) -> bool {
        false
    }

    fn set_position(&mut self, position: GMVec2D);

    fn add_position(&mut self, position: GMVec2D);

    fn get_position(&self) -> GMVec2D;

    fn update(&mut self, context: &mut GMUpdateContext);

    fn draw(&self, context: &mut GMDrawContext);

    fn send_message(&mut self, message: Rc<GMObjectMessage>);
}


pub struct GMObjectBase {
    pub name: String,
    pub active: bool,
    pub z_index: i32,
    pub groups: HashSet<String>,
    pub position: GMVec2D,
}

impl GMObjectBase {
    pub fn new(name: &str, position: GMVec2D) -> Self {
        Self {
            name: name.to_string(),
            active: true,
            z_index: 0,
            groups: HashSet::new(),
            position
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    pub fn set_z_index(&mut self, z_index: i32) {
        self.z_index = z_index;
    }

    pub fn get_z_index(&self) -> i32 {
        self.z_index
    }

    pub fn add_group(&mut self, group: &str) {
        self.groups.insert(group.to_string());
    }

    pub fn remove_group(&mut self, group: &str) {
        self.groups.remove(group);
    }

    pub fn is_in_group(&self, group: &str) -> bool {
        self.groups.contains(group)
    }

    pub fn set_position(&mut self, position: &GMVec2D) {
        self.position = *position;
    }

    pub fn add_position(&mut self, position: &GMVec2D) {
        self.position.add2(position);
    }

    pub fn get_position(&self) -> &GMVec2D {
        &self.position
    }

}

pub struct GMObjectManager {
    objects: Vec<Box<dyn GMObjectT>>,
}

impl GMObjectManager {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn get_index(&self, name: &str) -> Option<usize> {
        self.objects.iter().position(|object| object.get_name() == name)
    }

    pub fn add_object<O: 'static + GMObjectT>(&mut self, name: &str, object: O) -> Result<(), GMError> {
        self.add_object_box(name, Box::new(object))
    }

    pub fn add_object_box(&mut self, name: &str, object: Box<dyn GMObjectT>) -> Result<(), GMError> {
        match self.get_index(name) {
            Some(_) => {
                Err(GMError::ObjectAlreadyExists(name.to_string()))
            }
            None => {
                self.objects.push(object);
                Ok(())
            }
        }
    }

    pub fn remove_object(&mut self, name: &str) -> Result<(), GMError> {
        match self.get_index(name) {
            Some(index) => {
                self.objects.swap_remove(index);
                Ok(())
            }
            None => {
                Err(GMError::ObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn replace_object<O: 'static + GMObjectT>(&mut self, name: &str, object: O) -> Result<(), GMError> {
        self.replace_object_box(name, Box::new(object))
    }

    pub fn replace_object_box(&mut self, name: &str, object: Box<dyn GMObjectT>) -> Result<(), GMError> {
        match self.get_index(name) {
            Some(index) => {
                self.objects[index] = object;
                Ok(())
            }
            None => {
                Err(GMError::ObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        for object in self.objects.iter_mut() {
            object.update(context);
        }

        while let Some(message) = context.next_object_message() {
            self.send_message(message)?;
        }

        Ok(())
    }

    pub fn draw(&mut self, context: &mut GMDrawContext) {
        // Sort all objects by z order before drawing them
        self.objects.sort_by_key(|object| object.get_z_index());

        for object in self.objects.iter() {
            object.draw(context);
        }
    }

    pub fn send_message(&mut self, message: GMObjectMessage) -> Result<(), GMError> {
        use GMReceiver::*;

        let receiver = message.receiver.clone();

        match receiver {
            Object(name) => {
                match self.get_index(&name) {
                    Some(index) => {
                        self.objects[index].send_message(Rc::new(message));
                        Ok(())
                    }
                    None => {
                        Err(GMError::ObjectNotFound(name.to_string()))
                    }
                }        
            }
            Group(name) => {
                let message = Rc::new(message);

                for object in self.objects.iter_mut() {
                    if object.is_in_group(&name) {
                        object.send_message(message.clone());
                    }
                }

                Ok(())
            }
            Scene(_) => {
                Err(GMError::CantSendSceneMessageToObject(message))
            }
        }

    }

    pub fn get_ref(&self, name: &str) -> Result<&Box<dyn GMObjectT>, GMError> {
        match self.get_index(name) {
            Some(index) => {
                Ok(&self.objects[index])
            }
            None => {
                Err(GMError::ObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn get_mut_ref(&mut self, name: &str) -> Result<&mut Box<dyn GMObjectT>, GMError> {
        match self.get_index(name) {
            Some(index) => {
                Ok(&mut self.objects[index])
            }
            None => {
                Err(GMError::ObjectNotFound(name.to_string()))
            }
        }
    }
}
