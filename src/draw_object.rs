

use std::any::Any;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::rc::Rc;

use crate::GMContext;
use crate::GMError;
use crate::movement::{GMMovementT, GMMovementCommon};

type GMDrawMessage = Rc<dyn Any>;

#[derive(Debug)]
pub struct GMDrawObjectCommon {
    pub name: String,
    pub active: bool,
    pub z_index: i32,
    groups: HashSet<String>,
    messages: VecDeque<GMDrawMessage>,
    pub movement_common: GMMovementCommon,
    movements: Vec<Box<dyn GMMovementT>>,
}

impl GMDrawObjectCommon {
    pub fn new(name: &str, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            name: name.to_string(),
            movement_common: GMMovementCommon::new(x, y, width, height),
            .. Default::default()
        }
    }

    pub fn update(&mut self, _context: &mut GMContext) {
        todo!();
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

    pub fn send_message(&mut self, message: GMDrawMessage) {
        self.messages.push_back(message);
    }

    pub fn send_message_group(&mut self, group: &str, message: GMDrawMessage) {
        if self.groups.contains(group) {
            self.send_message(message);
        }
    }

    pub fn get_next_message(&mut self) -> Option<GMDrawMessage> {
        self.messages.pop_front()
    }

    pub fn add_movement(&mut self, movement: Box<dyn GMMovementT>) {
        self.movements.push(movement);
    }

    pub fn remove_movement(&mut self, index: usize) {
        self.movements.remove(index);
    }

    pub fn send_message_movement(&mut self, index: usize, message: GMDrawMessage) {
        self.movements[index].send_message(message);
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
            movement_common: self.movement_common.clone(),
            movements: self.movements.clone()
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
            movement_common: Default::default(),
            movements: Vec::new()
        }
    }
}

pub trait GMDrawObjectT {
    fn update(&mut self, context: &mut GMContext) -> Result<(), GMError>;

    fn draw(&self, context: &mut GMContext) -> Result<(), GMError>;

    fn get_common_ref(&self) -> &GMDrawObjectCommon;

    fn get_common_mut_ref(&mut self) -> &mut GMDrawObjectCommon;

    fn box_clone(&self) -> Box<dyn GMDrawObjectT>;
}

impl Clone for Box<dyn GMDrawObjectT> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
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

    pub fn add_draw_object<O: 'static + GMDrawObjectT>(&mut self, object: O) -> Result<(), GMError> {
        let name = &object.get_common_ref().name;

        match self.get_draw_object_index(name) {
            Some(_) => {
                Err(GMError::DrawObjectAlreadyExists(name.to_string()))
            }
            None => {
                self.draw_objects.push(Box::new(object));

                Ok(())
            }
        }
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

    pub fn replace_draw_object<O: 'static + GMDrawObjectT>(&mut self, object: O) -> Result<(), GMError> {
        let name = &object.get_common_ref().name;

        match self.get_draw_object_index(name) {
            Some(index) => {
                self.draw_objects[index] = Box::new(object);

                Ok(())
            }
            None => {
                Err(GMError::DrawObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn update(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        for object in self.draw_objects.iter_mut() {
            object.update(context)?;
        }

        Ok(())

    }

    pub fn draw(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        // Sort all drawable objects by z order before drawing them
        self.draw_objects.sort_by_key(|object| object.get_common_ref().z_index);

        for object in self.draw_objects.iter() {
            object.draw(context)?;
        }

        Ok(())
    }

    pub fn send_message_group(&mut self, group: &str, message: GMDrawMessage) -> Result<(), GMError> {
        for object in self.draw_objects.iter_mut() {
            object.get_common_mut_ref().send_message_group(group, message.clone());
        }

        Ok(())
    }

    pub fn get_ref(&self, name: &str) -> Result<&Box<dyn GMDrawObjectT>, GMError> {
        match self.get_draw_object_index(name) {
            Some(index) => {
                Ok(&self.draw_objects[index])
            }
            None => {
                Err(GMError::DrawObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn get_mut_ref(&mut self, name: &str) -> Result<&mut Box<dyn GMDrawObjectT>, GMError> {
        match self.get_draw_object_index(name) {
            Some(index) => {
                Ok(&mut self.draw_objects[index])
            }
            None => {
                Err(GMError::DrawObjectNotFound(name.to_string()))
            }
        }
    }
}
