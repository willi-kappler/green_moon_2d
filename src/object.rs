
use std::collections::HashSet;
use std::fmt::{self, Debug};

use crate::GMError;
use crate::context::{GMUpdateContext, GMDrawContext};
use crate::math::GMVec2D;
use crate::message::{GMMessage, GMReceiver};
use crate::property::{GMPropertyManager, GMValue};

pub trait GMObjectT {
    // Must be implemented:
    fn get_name(&self) -> &str;

    fn get_position(&self) -> GMVec2D;

    fn set_position(&mut self, position: GMVec2D);

    fn add_position(&mut self, position: GMVec2D);

    fn update(&mut self, context: &mut GMUpdateContext);

    fn draw(&self, context: &mut GMDrawContext);

    fn send_message(&mut self, message: GMMessage, context: &mut GMUpdateContext) -> Result<GMMessage, GMError>;

    fn clone_box(&self) -> Box<dyn GMObjectT>;

    // May be implemented:
    fn is_in_group(&self, _group: &str) -> bool {
        false
    }

    fn get_z_index(&self) -> i32 {
        0
    }
}

impl Clone for Box<dyn GMObjectT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Debug for Box<dyn GMObjectT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object: {}", self.get_name())
    }
}

#[derive(Clone, Debug)]
pub struct GMObjectBase {
    pub name: String,
    pub active: bool,
    pub position: GMVec2D,
    pub groups: HashSet<String>,
    pub properties: GMPropertyManager,
    pub sub_objects: Vec<Box<dyn GMObjectT>>,
}

impl GMObjectBase {
    pub fn new(name: &str, position: GMVec2D) -> Self {
        Self {
            name: name.to_string(),
            active: true,
            position,
            groups: HashSet::new(),
            properties: GMPropertyManager::new(),
            sub_objects: Vec::new(),
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

    // TODO: add set_position, get_position, ...

    pub fn add_group(&mut self, group: &str) {
        self.groups.insert(group.to_string());
    }

    pub fn remove_group(&mut self, group: &str) {
        self.groups.remove(group);
    }

    pub fn is_in_group(&self, group: &str) -> bool {
        self.groups.contains(group)
    }

    pub fn add_property(&mut self, name: &str, value: GMValue) {
        self.properties.add_property(name, value);
    }

    pub fn add_tag(&mut self, name: &str) {
        self.properties.add_tag(name);
    }

    pub fn remove_property(&mut self, name: &str) {
        self.properties.remove_property(name);
    }

    pub fn get_property(&self, name: &str) -> Option<&GMValue> {
        self.properties.get_property(name)
    }

    // TODO: add sub_object, remove_sub_object, ...
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
            self.send_message(message, context)?;
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

    pub fn send_message(&mut self, message: GMMessage, context: &mut GMUpdateContext) -> Result<(), GMError> {
        use GMReceiver::*;

        let receiver = message.receiver.clone();

        match receiver {
            Object(name) => {
                match self.get_index(&name) {
                    Some(index) => {
                        self.objects[index].send_message(message, context)?;
                        Ok(())
                    }
                    None => {
                        Err(GMError::ObjectNotFound(name.to_string()))
                    }
                }
            }
            ObjectGroup(name) => {
                for object in self.objects.iter_mut() {
                    if object.is_in_group(&name) {
                        object.send_message(message.clone(), context)?;
                    }
                }

                Ok(())
            }
            ObjectManager => {
                // TODO:
                todo!();
            }
            _ => {
                Err(GMError::UnknownMessageToObject(message))
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
