
use std::collections::HashSet;
use std::fmt::Debug;

use crate::GMError;
use crate::context::{GMUpdateContext, GMDrawContext};
use crate::math::GMVec2D;
use crate::message::{GMMessage, GMSender, GMReceiver, GMMessageData};
use crate::property::{GMPropertyManager, GMValue};

pub trait GMObjectT : Debug {
    // Must be implemented:
    fn get_name(&self) -> &str;

    fn set_name(&mut self, name: &str);

    fn get_z_index(&self) -> i32;

    fn set_z_index(&mut self, z_index: i32);

    fn get_active(&self) -> bool;

    fn set_active(&mut self, active: bool);

    fn get_position(&self) -> GMVec2D;

    fn set_position(&mut self, position: GMVec2D);

    fn add_position(&mut self, position: GMVec2D);

    fn get_velocity(&self) -> GMVec2D;

    fn set_velocity(&mut self, velocity: GMVec2D);

    fn add_velocity(&mut self, velocity: GMVec2D);

    fn update(&mut self, context: &mut GMUpdateContext);

    fn draw(&self, context: &mut GMDrawContext);

    fn send_message(&mut self, message: GMMessage, context: &mut GMUpdateContext) -> Result<GMMessage, GMError>;

    fn clone_box(&self) -> Box<dyn GMObjectT>;


    // May be implemented:
    fn is_in_group(&self, _group: &str) -> bool {
        false
    }

    fn add_group(&mut self, _group: &str) {
    }

    fn remove_group(&mut self, _group: &str) {
    }

    fn get_property(&self, _name: &str) -> Option<&GMValue> {
        None
    }

    fn has_property(&self, _name: &str) -> bool {
        false
    }

    fn add_property(&mut self, _name: &str, _value: GMValue) {
    }

    fn add_tag(&mut self, _name: &str) {
    }

    fn remove_property(&mut self, _name: &str) {
    }

    fn set_child(&mut self, _child: Box<dyn GMObjectT>) {
    }

    fn remove_child(&mut self) {
    }

    fn take_child(&mut self) -> Option<Box<dyn GMObjectT>> {
        None
    }
}

impl Clone for Box<dyn GMObjectT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub struct GMObjectBase {
    pub name: String,
    pub z_index: i32,
    pub active: bool,
    pub position: GMVec2D,
    pub velocity: GMVec2D,
    pub child: Option<Box<dyn GMObjectT>>,
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
            velocity: GMVec2D::new(0.0, 0.0),
            child: None,
            groups: HashSet::new(),
            properties: GMPropertyManager::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        if let Some(child) = &self.child {
            child.get_name()
        } else {
            &self.name
        }
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

    pub fn get_velocity(&self) -> &GMVec2D {
        &self.velocity
    }

    pub fn set_velocity(&mut self, velocity: GMVec2D) {
        self.velocity = velocity;
    }

    pub fn add_velocity(&mut self, velocity: &GMVec2D) {
        self.velocity.add2(&velocity);
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

    pub fn set_child(&mut self, child: Box<dyn GMObjectT>) {
        self.child = Some(child);
    }

    pub fn remove_child(&mut self) {
        self.child = None;
    }

    pub fn take_child(&mut self) -> Option<Box<dyn GMObjectT>> {
        self.child.take()
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

    pub fn add<O: 'static + GMObjectT>(&mut self, object: O) -> Result<(), GMError> {
        self.add_box(Box::new(object))
    }

    pub fn add_box(&mut self, object: Box<dyn GMObjectT>) -> Result<(), GMError> {
        let name = object.get_name();

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

    pub fn remove(&mut self, name: &str) -> Result<(), GMError> {
        self.take(name).map(|_| ())
    }

    pub fn replace<O: 'static + GMObjectT>(&mut self, object: O) -> Result<(), GMError> {
        self.replace_box(Box::new(object))
    }

    pub fn replace_box(&mut self, object: Box<dyn GMObjectT>) -> Result<(), GMError> {
        let name = object.get_name();

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

    pub fn set_parent(&mut self, name: &str, mut parent: Box<dyn GMObjectT>) -> Result<(), GMError> {
        let child = self.take(name)?;
        parent.set_child(child);
        self.add_box(parent)
    }

    pub fn remove_parent(&mut self, name: &str) -> Result<(), GMError> {
        // TODO: Remove parent from object
        todo!();
    }

    pub fn set_child(&mut self, name: &str, child: Box<dyn GMObjectT>) -> Result<(), GMError> {
        match self.index(name) {
            Some(index) => {
                self.objects[index].set_child(child);
                Ok(())
            }
            None => {
                Err(GMError::ObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn remove_child(&mut self, name: &str) -> Result<(), GMError> {
        match self.index(name) {
            Some(index) => {
                self.objects[index].remove_child();
                Ok(())
            }
            None => {
                Err(GMError::ObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn take_child(&mut self, name: &str) -> Result<Option<Box<dyn GMObjectT>>, GMError> {
        match self.index(name) {
            Some(index) => {
                Ok(self.objects[index].take_child())
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
                use GMMessageData::*;

                match message.message_data {
                    AddObject(object) => {
                        self.add_box(object)
                    }
                    ReplaceObject(object) => {
                        self.replace_box(object)
                    }
                    RemoveObject(ref name) => {
                        self.remove(name)
                    }
                    TakeObject(ref name) => {
                        let object = self.take(name)?;
                        let message_data = Object(object);
                        let sender = GMSender::ObjectManager;
                        if let Some(receiver) = GMMessage::sender2receiver(&message.sender) {
                            let message = GMMessage::new(sender, receiver, message_data);
                            context.send_message(message);

                            Ok(())
                        } else {
                            Err(GMError::SenderUnknown(message))
                        }

                    }
                    SetObjectParent(ref name, parent) => {
                        self.set_parent(name, parent)
                    }
                    RemoveObjectParent(ref name) => {
                        self.remove_parent(name)
                    }
                    SetObjectChild(ref name, child) => {
                        self.set_child(name, child)
                    }
                    RemoveObjectChild(ref name) => {
                        self.remove_child(name)
                    }
                    TakeObjectChild(ref name) => {
                        let child = self.take_child(name)?;
                        //TODO: send response to original sender of message
                        Ok(())
                    }

                    _ => {
                        Err(GMError::UnknownMessageToObject(message))
                    }

                }
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
