
use std::fmt::Debug;

use delegate::delegate;

use crate::GMError;
use crate::context::{GMUpdateContext, GMDrawContext};
use crate::math::GMVec2D;
use crate::message::{GMMessage, GMSender, GMReceiver, GMMessageData, GMMessageFactory};
use crate::property::{GMPropertyManager, GMValue};

// TODO:
// - add angle, angle velocity
// - add get_angle, set_angle, add_angle
// - add get_angle_velocity, set_angle_velocity, add_angle_velocity
// - add collision_shape

pub trait GMObjectT : Debug {
    // Must be implemented:

    fn update(&mut self, context: &mut GMUpdateContext);

    fn draw(&self, context: &mut GMDrawContext);

    fn send_message(&mut self, message: GMMessage, context: &mut GMUpdateContext);

    fn clone_box(&self) -> Box<dyn GMObjectT>;

    fn get_name(&self) -> &str;

    fn set_name(&self, name: &str);

    fn get_z_index(&self) -> i32;

    fn set_z_index(&mut self, z_index: i32);

    fn get_active(&self) -> bool;

    fn set_active(&mut self, active: bool);

    fn get_position(&self) -> GMVec2D;

    fn set_position(&mut self, position: GMVec2D);

    fn add_position(&mut self, position: &GMVec2D);


    // May be implemented:
    fn get_next_position(&self) -> GMVec2D {
        self.get_position()
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

    fn get_child(&self) -> Option<Box<dyn GMObjectT>> {
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
    properties: GMPropertyManager,
}

impl GMObjectBase {
    pub fn new(name: &str, position: GMVec2D) -> Self {
        Self {
            name: name.to_string(),
            z_index: 0,
            active: true,
            position,
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

    pub fn get_next_position(&self) -> GMVec2D {
        self.position
    }

    delegate! {
        to self.properties {
            pub fn get_property(&self, name: &str) -> Option<&GMValue>;
            pub fn has_property(&self, name: &str) -> bool;
            pub fn add_property(&mut self, name: &str, value: GMValue);
            pub fn add_tag(&mut self, name: &str);
            pub fn remove_property(&mut self, name: &str);
        }
    }
}

pub struct GMObjectManager {
    objects: Vec<Box<dyn GMObjectT>>,
    message_factory: GMMessageFactory,
}

impl GMObjectManager {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            message_factory: GMMessageFactory::new(GMSender::ObjectManager),
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
        match self.index(name) {
            Some(index) => {
                match self.objects[index].get_child() {
                    Some(child) => {
                        self.objects[index] = child;
                        Ok(())
                    }
                    None => {
                        Err(GMError::ObjectHasNoChild(name.to_string()))
                    }
                }
            }
            None => {
                Err(GMError::ObjectNotFound(name.to_string()))
            }
        }
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
                Ok(self.objects[index].get_child())
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
                        self.objects[index].send_message(message, context);
                        Ok(())
                    }
                    None => {
                        Err(GMError::ObjectNotFound(name.to_string()))
                    }
                }
            }
            ObjectWithProperty(name) => {
                for object in self.objects.iter_mut() {
                    if object.has_property(&name) {
                        object.send_message(message.clone(), context)
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
                        self.message_factory.reply(&message, message_data, context);

                        Ok(())

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
                        match self.take_child(name)? {
                            Some(child) => {
                                let message_data = Object(child);
                                self.message_factory.reply(&message, message_data, context);

                                Ok(())
                            }
                            None => {
                                Err(GMError::ObjectHasNoChild(name.to_string()))
                            }
                        }
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
