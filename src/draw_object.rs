

use std::any::Any;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::rc::Rc;
use std::fmt::{self, Debug, Formatter};

use crate::collision::GMCollisionShape;
use crate::math::GMVec2D;
use crate::{GMUpdateContext, GMDrawContext};
use crate::GMError;

#[derive(Debug, Clone)]
pub struct GMDrawObjectMessage {
    pub from: String,
    pub value: Rc<dyn Any>,
}

#[derive(Debug)]
pub struct GMDrawObjectCommon {
    pub name: String,
    pub active: bool,
    pub z_index: i32,
    pub groups: HashSet<String>,
    pub messages: VecDeque<GMDrawObjectMessage>,
    pub position: GMVec2D,
    pub velocity: GMVec2D,
    pub acceleration: GMVec2D,
    pub rotation: f32,
    pub rotation_velocity: f32,
    pub collision_shape: GMCollisionShape,
}

impl GMDrawObjectCommon {
    pub fn new(name: &str, _x: f32, _y: f32, _width: f32, _height: f32) -> Self {
        Self {
            name: name.to_string(),
            .. Default::default()
        }
    }

    // TODO: add getter and setter

    pub fn update(&mut self) {
        self.velocity.add2(&self.acceleration);
        self.position.add2(&self.velocity);
        self.rotation += self.rotation_velocity;
    }

    pub fn add_group(&mut self, group: &str) -> bool {
        self.groups.insert(group.to_string())
    }

    pub fn remove_group(&mut self, group: &str) -> bool {
        self.groups.remove(group)
    }

    pub fn is_in_group(&self, group: &str) -> bool {
        self.groups.contains(group)
    }

    pub fn send_message(&mut self, message: GMDrawObjectMessage) {
        self.messages.push_back(message);
    }

    pub fn send_message_group(&mut self, group: &str, message: GMDrawObjectMessage) {
        if self.groups.contains(group) {
            self.send_message(message);
        }
    }

    pub fn get_next_message(&mut self) -> Option<GMDrawObjectMessage> {
        self.messages.pop_front()
    }
}

impl Clone for GMDrawObjectCommon {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            active: self.active.clone(),
            z_index: self.z_index.clone(),
            groups: self.groups.clone(),
            messages: VecDeque::new(), // Don't clone all the messages
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            acceleration: self.acceleration.clone(),
            rotation: self.rotation,
            rotation_velocity: self.rotation_velocity,
            collision_shape: self.collision_shape.clone(),
        }
    }
}

impl Default for GMDrawObjectCommon {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            active: true,
            z_index: 0,
            groups: HashSet::new(),
            messages: VecDeque::new(),
            position: GMVec2D::default(),
            velocity: GMVec2D::default(),
            acceleration: GMVec2D::default(),
            rotation: 0.0,
            rotation_velocity: 0.0,
            collision_shape: GMCollisionShape::default(),
        }
    }
}

pub trait GMDrawObjectT {
    fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError>;

    fn draw(&self, context: &mut GMDrawContext) -> Result<(), GMError>;

    fn get_common_ref(&self) -> &GMDrawObjectCommon;

    fn get_common_mut_ref(&mut self) -> &mut GMDrawObjectCommon;

    fn box_clone(&self) -> Box<dyn GMDrawObjectT>;
}

impl Clone for Box<dyn GMDrawObjectT> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Debug for Box<dyn GMDrawObjectT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GMDrawObject: '{}'", self.get_common_ref().name)
    }
}

pub(crate) enum GMDrawObjectManagerMessage {
    AddDrawObject(Box<dyn GMDrawObjectT>),
    RemoveDrawObject(String),
    ReplaceDrawObject(Box<dyn GMDrawObjectT>),
    SendMessage(String, GMDrawObjectMessage),
    SendMessageGroup(String, GMDrawObjectMessage),
}

pub struct GMDrawObjectManager {
    draw_objects: Vec<Box<dyn GMDrawObjectT>>,
}

impl GMDrawObjectManager {
    pub fn new() -> Self {
        Self {
            draw_objects: Vec::new(),
        }
    }

    fn get_draw_object_index(&self, name: &str) -> Option<usize> {
        self.draw_objects.iter().position(|object| object.get_common_ref().name == name)
    }

    pub fn add_draw_object_box(&mut self, object: Box<dyn GMDrawObjectT>) -> Result<(), GMError> {
        let name = &object.get_common_ref().name;

        match self.get_draw_object_index(name) {
            Some(_) => {
                Err(GMError::DrawObjectAlreadyExists(name.to_string()))
            }
            None => {
                self.draw_objects.push(object);

                Ok(())
            }
        }
    }

    pub fn add_draw_object<O: 'static + GMDrawObjectT>(&mut self, object: O) -> Result<(), GMError> {
        self.add_draw_object_box(Box::new(object))
    }

    pub fn remove_draw_object(&mut self, name: &str) -> Result<(), GMError> {
        match self.get_draw_object_index(name) {
            Some(index) => {
                self.draw_objects.swap_remove(index);
                Ok(())
            }
            None => {
                Err(GMError::DrawObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn replace_draw_object_box(&mut self, object: Box<dyn GMDrawObjectT>) -> Result<(), GMError> {
        let name = &object.get_common_ref().name;

        match self.get_draw_object_index(name) {
            Some(index) => {
                self.draw_objects[index] = object;

                Ok(())
            }
            None => {
                Err(GMError::DrawObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn replace_draw_object<O: 'static + GMDrawObjectT>(&mut self, object: O) -> Result<(), GMError> {
        self.replace_draw_object_box(Box::new(object))
    }

    pub fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        for object in self.draw_objects.iter_mut() {
            object.update(context)?;
        }

        while let Some(message) = context.next_draw_manager_message() {
            use GMDrawObjectManagerMessage::*;

            match message {
                AddDrawObject(object) => {
                    self.add_draw_object_box(object)?;
                }
                RemoveDrawObject(name) => {
                    self.remove_draw_object(&name)?;
                }
                ReplaceDrawObject(object) => {
                    self.replace_draw_object_box(object)?;
                }
                SendMessage(receiver, message) => {
                    let common = self.get_common_mut_ref(&receiver)?;
                    common.send_message(message);
                }
                SendMessageGroup(group, message) => {
                    self.send_message_group(&group, message)?;
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, context: &mut GMDrawContext) -> Result<(), GMError> {
        // Sort all drawable objects by z order before drawing them
        self.draw_objects.sort_by_key(|object| object.get_common_ref().z_index);

        for object in self.draw_objects.iter() {
            object.draw(context)?;
        }

        Ok(())
    }

    pub fn send_message_group(&mut self, group: &str, message: GMDrawObjectMessage) -> Result<(), GMError> {
        for object in self.draw_objects.iter_mut() {
            object.get_common_mut_ref().send_message_group(group, message.clone());
        }

        Ok(())
    }

    pub fn get_common_ref(&self, name: &str) -> Result<&GMDrawObjectCommon, GMError> {
        match self.get_draw_object_index(name) {
            Some(index) => {
                Ok(self.draw_objects[index].get_common_ref())
            }
            None => {
                Err(GMError::DrawObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn get_common_mut_ref(&mut self, name: &str) -> Result<&mut GMDrawObjectCommon, GMError> {
        match self.get_draw_object_index(name) {
            Some(index) => {
                Ok(self.draw_objects[index].get_common_mut_ref())
            }
            None => {
                Err(GMError::DrawObjectNotFound(name.to_string()))
            }
        }
    }
}
