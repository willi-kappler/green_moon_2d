

use std::collections::{HashSet, HashMap};
use std::any::Any;

use log::debug;

use crate::context::{GMUpdateContext, GMDrawContext};
use crate::error::GMError;
use crate::message::{GMReceiver, GMObjectMessage};


pub trait GMSceneT {
    // Must be implemented:
    fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError>;

    fn draw(&mut self, context: &mut GMDrawContext) -> Result<(), GMError>;

    fn get_name(&self) -> &str;

    fn send_message(&mut self, message: GMObjectMessage, context: &mut GMUpdateContext);

    // May be implemented:    
    fn init(&mut self, _context: &mut GMUpdateContext) -> Result<(), GMError> {
        Ok(())
    }

    fn exit(&mut self, _context: &mut GMUpdateContext) -> Result<(), GMError> {
        Ok(())
    }

    fn add_group(&mut self, _group: &str) {
    }

    fn remove_group(&mut self, _group: &str) {
    }

    fn is_in_group(&self) -> bool {
        false
    }

    fn add_property(&mut self, _name: &str, _property: Box<dyn Any>) {
    }

    fn add_tag(&mut self, _name: &str) {
    }

    fn remove_property(&mut self, _name: &str) {
    }

    fn get_property(&self, _name: &str) -> Option<Box<dyn Any>> {
        None
    }

    fn set_child(&mut self, _child: Box<dyn GMSceneT>) {
    }

    fn get_child(&mut self) -> Option<Box<dyn GMSceneT>> {
        None
    }
}

pub struct GMSceneBase {
    pub name: String,
    pub groups: HashSet<String>,
    pub properties: HashMap<String, Box<dyn Any>>,
    pub child: Option<Box<dyn GMSceneT>>,
}

impl GMSceneBase {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            groups: HashSet::new(),
            properties: HashMap::new(),
            child: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
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

    pub fn add_property(&mut self, name: &str, property: Box<dyn Any>) {
        self.properties.insert(name.to_string(), property);
    }

    pub fn add_tag(&mut self, name: &str) {
        self.properties.insert(name.to_string(), Box::new(()));
    }

    pub fn remove_property(&mut self, name: &str) {
        self.properties.remove(name);
    }

    pub fn get_property(&self, name: &str) -> Option<&Box<dyn Any>> {
        self.properties.get(name)
    }

    pub fn set_child(&mut self, child: Box<dyn GMSceneT>) {
        self.child = Some(child);
    }

    pub fn get_child(&mut self) -> &Option<Box<dyn GMSceneT>> {
        &self.child
    }
}

pub(crate) enum GMSceneMessage {
    AddScene(Box<dyn GMSceneT>),
    RemoveScene(String),
    ChangeToScene(String),
    ReplaceScene(Box<dyn GMSceneT>),
    Push,
    Pop,
    ObjectMessage(GMObjectMessage),
}

pub struct GMSceneManager {
    scenes: Vec<Box<dyn GMSceneT>>,
    scene_stack: Vec<String>,
    current_scene: usize,
}

impl GMSceneManager {
    pub(crate) fn new() -> Self {
        Self {
            scenes: Vec::new(),
            scene_stack: Vec::new(),
            current_scene: 0,
        }
    }

    fn get_index(&self, name: &str) -> Option<usize> {
        debug!("GMSceneManager::get_scene_index(), name: '{}'", name);

        self.scenes.iter().position(|scene| scene.get_name() == name)
    }

    pub(crate) fn add_scene(&mut self, scene: Box<dyn GMSceneT>) -> Result<(), GMError> {
        let name = scene.get_name();

        debug!("GMSceneManager::add_scene(), name: '{}'", name);

        match self.get_index(name) {
            Some(_) => {
                Err(GMError::SceneAlreadyExists(name.to_string()))
            }
            None => {
                self.scenes.push(scene);

                Ok(())
            }
        }
    }

    fn remove_scene(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMSceneManager::remove_scene(), name: '{}'", name);

        match self.get_index(name) {
            Some(index) => {
                if self.current_scene == index {
                    Err(GMError::CantRemoveCurrentScene(name.to_string()))
                } else {
                    if self.current_scene == self.scenes.len() - 1 {
                        self.current_scene = index;
                    }

                    self.scenes.swap_remove(index);

                    Ok(())
                }
            }
            None => {
                Err(GMError::SceneAlreadyExists(name.to_string()))
            }
        }
    }

    fn change_to_scene(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMSceneManager::change_scene(), name: '{}'", name);

        match self.get_index(name) {
            Some(index) => {
                self.current_scene = index;

                Ok(())
            }
            None => {
                Err(GMError::SceneNotFound(name.to_string()))
            }
        }
    }

    fn replace_scene(&mut self, scene: Box<dyn GMSceneT>) -> Result<(), GMError> {
        let name = scene.get_name();

        debug!("GMSceneManager::replace_scene(), name: '{}'", name);

        match self.get_index(name) {
            Some(index) => {
                self.scenes[index] = scene;
                Ok(())
            }
            None => {
                Err(GMError::SceneNotFound(name.to_string()))
            }
        }
    }

    fn push(&mut self) {
        let name = self.scenes[self.current_scene].get_name();

        self.scene_stack.push(name.to_string());
    }

    fn pop(&mut self) -> Result<(), GMError> {
        match self.scene_stack.pop() {
            Some(name) => {
                self.change_to_scene(&name)
            }
            None => {
                Err(GMError::SceneStackEmpty)
            }
        }
    }

    fn send_message(&mut self, message: GMObjectMessage, context: &mut GMUpdateContext) -> Result<(), GMError> {
        use GMReceiver::*;

        let receiver = message.receiver.clone();

        match receiver {
            CurrentScene => {
                self.scenes[self.current_scene].send_message(message, context);
                Ok(())
            }
            Scene(name) => {
                match self.get_index(&name) {
                    Some(index) => {
                        self.scenes[index].send_message(message, context);
                        Ok(())
                    }
                    None => {
                        Err(GMError::SceneNotFound(name))
                    }
                }
            }
            SceneGroup(name) => {
                todo!();
                Ok(())
            }
            _ => {
                Err(GMError::CantSendObjectMessageToScene(message))
            }
        }
    }

    pub(crate) fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        self.scenes[self.current_scene].update(context)?;

        use GMSceneMessage::*;

        while let Some(message) = context.next_scene_message() {
            match message {
                AddScene(scene) => {
                    self.add_scene(scene)?
                }
                RemoveScene(name) => {
                    self.remove_scene(&name)?
                }
                ChangeToScene(name) => {
                    self.change_to_scene(&name)?
                }
                ReplaceScene(scene) => {
                    self.replace_scene(scene)?
                }
                Push => {
                    self.push()
                }
                Pop => {
                    self.pop()?
                }
                ObjectMessage(message) => {
                    self.send_message(message, context)?;
                }
            }
        }

        Ok(())
    }

    pub(crate) fn draw(&mut self, context: &mut GMDrawContext) -> Result<(), GMError> {
        self.scenes[self.current_scene].draw(context)
    }
}
