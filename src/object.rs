
use std::fmt::Debug;

use delegate::delegate;

use crate::GMError;
use crate::context::{GMUpdateContext, GMDrawContext};
use crate::math::GMVec2D;
use crate::message::{GMMessage, GMSender, GMReceiver, GMMessageData};
use crate::property::{GMPropertyManager, GMValue};

// TODO:
// - add angle, angle velocity
// - add get_angle, set_angle, add_angle
// - add get_angle_velocity, set_angle_velocity, add_angle_velocity
// - add collision_shape

pub trait GMObjectT : Debug {
    // Must be implemented:
    fn clone_box(&self) -> Box<dyn GMObjectT>;


    // May be implemented:
    fn update(&mut self, context: &mut GMUpdateContext) {
        if let Some(child) = self.get_child_mut() {
            child.update(context)
        } else {
            panic!("Implement 'update()' or set a child!");
        }
    }

    fn draw(&self, context: &mut GMDrawContext) {
        if let Some(child) = self.get_child_ref() {
            child.draw(context)
        } else {
            panic!("Implement 'draw()' or set a child!");
        }
    }

    fn send_message(&mut self, message: GMMessage, context: &mut GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        if let Some(child) = self.get_child_mut() {
            child.send_message(message, context)
        } else {
            panic!("Implement 'send_message()' or set a child!");
        }
    }

    fn get_name(&self) -> &str {
        if let Some(child) = self.get_child_ref() {
            child.get_name()
        } else {
            panic!("Implement 'get_name()' or set a child!");
        }
    }

    fn get_position(&self) -> GMVec2D {
        if let Some(child) = self.get_child_ref() {
            child.get_position()
        } else {
            panic!("Implement 'get_position()' or set a child!");
        }
    }

    fn set_position(&mut self, position: GMVec2D) {
        if let Some(child) = self.get_child_mut() {
            child.set_position(position)
        } else {
            panic!("Implement 'set_position()' or set a child!");
        }
    }

    fn add_position(&mut self, position: &GMVec2D) {
        if let Some(child) = self.get_child_mut() {
            child.add_position(position)
        } else {
            panic!("Implement 'add_position()' or set a child!");
        }
    }

    fn get_active(&self) -> bool {
        if let Some(child) = self.get_child_ref() {
            child.get_active()
        } else {
            true
        }
    }

    fn set_active(&mut self, active: bool) {
        if let Some(child) = self.get_child_mut() {
            child.set_active(active)
        }
    }

    fn get_z_index(&self) -> i32 {
        if let Some(child) = self.get_child_ref() {
            child.get_z_index()
        } else {
            0
        }
    }

    fn set_z_index(&mut self, z_index: i32) {
        if let Some(child) = self.get_child_mut() {
            child.set_z_index(z_index)
        }
    }

    fn get_next_position(&self) -> GMVec2D {
        self.get_position()
    }

    fn get_property(&self, name: &str) -> Option<&GMValue> {
        if let Some(child) = self.get_child_ref() {
            child.get_property(name)
        } else {
            None
        }
    }

    fn has_property(&self, name: &str) -> bool {
        if let Some(child) = self.get_child_ref() {
            child.has_property(name)
        } else {
            false
        }
    }

    fn add_property(&mut self, name: &str, value: GMValue) {
        if let Some(child) = self.get_child_mut() {
            child.add_property(name, value)
        }
    }

    fn add_tag(&mut self, name: &str) {
        if let Some(child) = self.get_child_mut() {
            child.add_tag(name)
        }
    }

    fn remove_property(&mut self, name: &str) {
        if let Some(child) = self.get_child_mut() {
            child.remove_property(name)
        }
    }

    fn set_child(&mut self, _child: Box<dyn GMObjectT>) {
    }

    fn remove_child(&mut self) {
    }

    fn get_child(&self) -> Option<Box<dyn GMObjectT>> {
        None
    }

    fn get_child_ref(&self) -> Option<&Box<dyn GMObjectT>> {
        None
    }

    fn get_child_mut(&mut self) -> Option<&mut Box<dyn GMObjectT>> {
        None
    }

    fn as_sender(&self) -> GMSender {
        GMSender::Object(self.get_name().to_string())
    }

    fn as_receiver(&self) -> GMReceiver {
        GMReceiver::Object(self.get_name().to_string())
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

    pub fn send_message(&mut self, message: GMMessage, _context: &mut GMUpdateContext) -> Result<Option<GMMessage>, GMError> {
        use GMMessageData::*;

        let sender = GMSender::Object(self.name.to_string());
        let receiver: GMReceiver = (&message.sender).into();

        match message.message_data {
            GetZIndex => {
                Ok(Some(GMMessage::new(sender, receiver, GMMessageData::ZIndex(self.z_index))))
            }
            SetZIndex(z_index) => {
                self.set_z_index(z_index);
                Ok(None)
            }
            GetActive => {
                Ok(Some(GMMessage::new(sender, receiver, GMMessageData::Active(self.active))))
            }
            SetActive(active) => {
                self.set_active(active);
                Ok(None)
            }
            GetPosition => {
                Ok(Some(GMMessage::new(sender, receiver, GMMessageData::Position(self.position))))
            }
            SetPosition(position) => {
                self.set_position(position);
                Ok(None)
            }
            AddPosition(position) => {
                self.add_position(&position);
                Ok(None)
            }
            GetNextPosition => {
                Ok(Some(GMMessage::new(sender, receiver, GMMessageData::Position(self.position))))
            }
            GetProperty(name) => {
                let message_data = match self.get_property(&name) {
                    Some(value) => {
                        GMMessageData::Property(name, value.clone())
                    }
                    None => {
                        GMMessageData::PropertyNotFound(name)
                    }
                };

                Ok(Some(GMMessage::new(sender, receiver, message_data)))
            }
            HasProperty(name) => {
                let message_data = if self.has_property(&name) {
                    GMMessageData::PropertyFound(name)
                } else {
                    GMMessageData::PropertyNotFound(name)
                };

                Ok(Some(GMMessage::new(sender, receiver, message_data)))
            }
            AddProperty(name, value) => {
                self.add_property(&name, value);
                Ok(None)
            }
            AddTag(name) => {
                self.add_tag(&name);
                Ok(None)
            }
            RemoveProperty(name) => {
                self.remove_property(&name);
                Ok(None)
            }
            _ => {
                Err(GMError::UnknownMessageToObject(message))
            }
        }
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

    // Maybe use From trait, GMObjectT -> Box<dyn GMObjectT>
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

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn replace<O: 'static + GMObjectT>(&mut self, object: O) -> Result<(), GMError> {
        self.replace_box(Box::new(object))
    }

    // Maybe use From trait, GMObjectT -> Box<dyn GMObjectT>
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
                        if let Some(message) = self.objects[index].send_message(message, context)? {
                            context.send_message(message);
                        }

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
                        if let Some(message) = object.send_message(message.clone(), context)? {
                            context.send_message(message)
                        }
                    }
                }

                Ok(())
            }
            ObjectManager => {
                use GMMessageData::*;

                let sender = GMSender::ObjectManager;
                let receiver = message.as_reply();

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
                        context.send_message(GMMessage::new(sender, receiver, message_data));

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
                                context.send_message(GMMessage::new(sender, receiver, message_data));

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

#[derive(Clone)]
pub struct GMObjectAction {
    pub action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> (),
}

impl GMObjectAction {
    pub fn new(action: fn(&mut Box<dyn GMObjectT>, &mut GMUpdateContext) -> ()) -> Self {
        Self {
            action
        }
    }
}

impl Debug for GMObjectAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GMObjectAction")
    }
}
