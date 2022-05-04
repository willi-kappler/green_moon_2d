
use std::collections::HashSet;
use std::fmt::{self, Debug};

use crate::GMError;
use crate::context::{GMUpdateContext, GMDrawContext};
use crate::math::GMVec2D;
use crate::message::{GMMessage, GMReceiver};
use crate::property::{GMPropertyManager, GMValue};

pub trait GMObjectT {
    fn get_name(&self) -> &str;

    fn set_name(&mut self, name: &str);

    fn get_z_index(&self) -> i32;

    fn set_z_index(&mut self, z_index: i32);

    fn get_active(&self) -> bool;

    fn set_active(&mut self, active: bool);

    fn get_position(&self) -> GMVec2D;

    fn set_position(&mut self, position: GMVec2D);

    fn add_position(&mut self, position: GMVec2D);

    fn is_in_group(&self, _group: &str) -> bool;

    fn add_group(&mut self, _group: &str);

    fn remove_group(&mut self, _group: &str);

    fn get_property(&self, name: &str) -> Option<&GMValue>;

    fn has_property(&self, name: &str) -> bool;

    fn add_property(&mut self, name: &str, value: GMValue);

    fn add_tag(&mut self, name: &str);

    fn remove_property(&mut self, name: &str);

    fn update(&mut self, context: &mut GMUpdateContext);

    fn draw(&self, context: &mut GMDrawContext);

    fn send_message(&mut self, message: GMMessage, context: &mut GMUpdateContext) -> Result<GMMessage, GMError>;

    fn clone_box(&self) -> Box<dyn GMObjectT>;
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
    pub z_index: i32,
    pub active: bool,
    pub position: GMVec2D,
    groups: HashSet<String>,
    properties: GMPropertyManager,
}

impl GMObjectBase {
    pub fn new(name: &str, position: GMVec2D) -> Self {
        Self {
            name: name.to_string(),
            z_index: 0,
            active: true,
            position,
            groups: HashSet::new(),
            properties: GMPropertyManager::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_z_index(&self) -> i32 {
        self.z_index
    }

    pub fn set_z_index(&mut self, z_index: i32) {
        self.z_index = z_index;
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_position(&self) -> &GMVec2D {
        &self.position
    }

    pub fn set_position(&mut self, position: GMVec2D) {
        self.position = position;
    }

    pub fn add_position(&mut self, position: &GMVec2D) {
        self.position.add2(&position);
    }

    pub fn is_in_group(&self, group: &str) -> bool {
        self.groups.contains(group)
    }

    pub fn add_group(&mut self, group: &str) {
        self.groups.insert(group.to_string());
    }

    pub fn remove_group(&mut self, group: &str) {
        self.groups.remove(group);
    }

    pub fn get_property(&self, name: &str) -> Option<&GMValue> {
        self.properties.get_property(name)
    }

    pub fn has_property(&self, name: &str) -> bool {
        self.properties.has_property(name)
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

    fn index(&self, name: &str) -> Option<usize> {
        self.objects.iter().position(|object| object.get_name() == name)
    }

    pub fn add<O: 'static + GMObjectT>(&mut self, name: &str, object: O) -> Result<(), GMError> {
        self.add_box(name, Box::new(object))
    }

    pub fn add_box(&mut self, name: &str, object: Box<dyn GMObjectT>) -> Result<(), GMError> {
        match self.index(name) {
            Some(_) => {
                Err(GMError::ObjectAlreadyExists(name.to_string()))
            }
            None => {
                self.objects.push(object);
                Ok(())
            }
        }
    }

    pub fn take(&mut self, name: &str) -> Result<Box<dyn GMObjectT>, GMError> {
        match self.index(name) {
            Some(index) => {
                Ok(self.objects.swap_remove(index))
            }
            None => {
                Err(GMError::ObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn replace<O: 'static + GMObjectT>(&mut self, name: &str, object: O) -> Result<(), GMError> {
        self.replace_box(name, Box::new(object))
    }

    pub fn replace_box(&mut self, name: &str, object: Box<dyn GMObjectT>) -> Result<(), GMError> {
        match self.index(name) {
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
                match self.index(&name) {
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
        match self.index(name) {
            Some(index) => {
                Ok(&self.objects[index])
            }
            None => {
                Err(GMError::ObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn get_mut_ref(&mut self, name: &str) -> Result<&mut Box<dyn GMObjectT>, GMError> {
        match self.index(name) {
            Some(index) => {
                Ok(&mut self.objects[index])
            }
            None => {
                Err(GMError::ObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn GMObjectT>>  {
        self.objects.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn GMObjectT>>  {
        self.objects.iter_mut()
    }

}
