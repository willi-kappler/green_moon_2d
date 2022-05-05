
use std::collections::HashSet;
use std::fmt::{self, Debug};

use log::debug;

use crate::context::{GMUpdateContext, GMDrawContext};
use crate::error::GMError;
use crate::message::{GMReceiver, GMMessage, GMMessageData};
use crate::property::{GMPropertyManager, GMValue};

pub trait GMSceneT {
    // Must be implemented:
    fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError>;

    fn draw(&mut self, context: &mut GMDrawContext) -> Result<(), GMError>;

    fn get_name(&self) -> &str;



    fn send_message(&mut self, message: GMMessage, context: &mut GMUpdateContext) -> Result<GMMessage, GMError>;

    fn clone_box(&self) -> Box<dyn GMSceneT>;

    // May be implemented:
    fn init(&mut self, _context: &mut GMUpdateContext) -> Result<(), GMError> {
        Ok(())
    }

    fn exit(&mut self, _context: &mut GMUpdateContext) -> Result<(), GMError> {
        Ok(())
    }

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

    fn set_child(&mut self, _child: Box<dyn GMSceneT>) {
    }

    fn remove_child(&mut self) {
    }
}

impl Clone for Box<dyn GMSceneT> {
    fn clone(&self) -> Box<dyn GMSceneT> {
        self.clone_box()
    }
}

impl Debug for Box<dyn GMSceneT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scene: {}", self.get_name())
    }
}

#[derive(Debug, Clone)]
pub struct GMSceneBase {
    pub name: String,
    pub child: Option<Box<dyn GMSceneT>>,
    groups: HashSet<String>,
    properties: GMPropertyManager,
}

impl GMSceneBase {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
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

    pub fn add_group(&mut self, group: &str) {
        self.groups.insert(group.to_string());
    }

    pub fn remove_group(&mut self, group: &str) {
        self.groups.remove(group);
    }

    pub fn is_in_group(&self, group: &str) -> bool {
        self.groups.contains(group)
    }

    pub fn add_property(&mut self, name: &str, property: GMValue) {
        self.properties.add_property(name, property);
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

    pub fn set_child(&mut self, child: Box<dyn GMSceneT>) {
        self.child = Some(child);
    }

    pub fn remove_child(&mut self) {
        self.child = None;
    }
}


#[derive(Debug, Clone)]
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

    fn index(&self, name: &str) -> Option<usize> {
        debug!("GMSceneManager::get_scene_index(), name: '{}'", name);

        self.scenes.iter().position(|scene| scene.get_name() == name)
    }

    pub(crate) fn add(&mut self, scene: Box<dyn GMSceneT>) -> Result<(), GMError> {
        let name = scene.get_name();

        debug!("GMSceneManager::add_scene(), name: '{}'", name);

        match self.index(name) {
            Some(_) => {
                Err(GMError::SceneAlreadyExists(name.to_string()))
            }
            None => {
                self.scenes.push(scene);

                Ok(())
            }
        }
    }

    fn take(&mut self, name: &str) -> Result<Box<dyn GMSceneT>, GMError> {
        debug!("GMSceneManager::take(), name: '{}'", name);

        match self.index(name) {
            Some(index) => {
                if self.current_scene == index {
                    Err(GMError::CantRemoveCurrentScene(name.to_string()))
                } else {
                    if self.current_scene == self.scenes.len() - 1 {
                        self.current_scene = index;
                    }

                    Ok(self.scenes.swap_remove(index))
                }
            }
            None => {
                Err(GMError::SceneNotFound(name.to_string()))
            }
        }
    }

    fn change_to(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMSceneManager::change_scene(), name: '{}'", name);

        match self.index(name) {
            Some(index) => {
                self.current_scene = index;

                Ok(())
            }
            None => {
                Err(GMError::SceneNotFound(name.to_string()))
            }
        }
    }

    fn replace(&mut self, scene: Box<dyn GMSceneT>) -> Result<(), GMError> {
        let name = scene.get_name();

        debug!("GMSceneManager::replace_scene(), name: '{}'", name);

        match self.index(name) {
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
                self.change_to(&name)
            }
            None => {
                Err(GMError::SceneStackEmpty)
            }
        }
    }

    pub(crate) fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        self.scenes[self.current_scene].update(context)?;

        use GMReceiver::*;
        use GMMessageData::*;

        while let Some(message) = context.next_scene_message() {
            let receiver = message.receiver.clone();

            match receiver {
                CurrentScene => {
                    self.scenes[self.current_scene].send_message(message, context)?;
                }
                Scene(name) => {
                    match self.index(&name) {
                        Some(index) => {
                            self.scenes[index].send_message(message, context)?;
                        }
                        None => {
                            return Err(GMError::SceneNotFound(name))
                        }
                    }
                }
                SceneGroup(name) => {
                    for scene in self.scenes.iter_mut() {
                        if scene.is_in_group(&name) {
                            scene.send_message(message.clone(), context)?;
                        }
                    }
                }
                SceneManager => {
                    match message.message_data {
                        AddScene(scene) => {
                            self.add(scene)?
                        }
                        RemoveScene(name) => {
                            self.take(&name).map(|_| ())?
                        }
                        // TODO: Maybe add TakeScene message
                        ChangeToScene(name) => {
                            self.change_to(&name)?
                        }
                        ReplaceScene(scene) => {
                            self.replace(scene)?
                        }
                        PushCurrentScene => {
                            self.push()
                        }
                        PopCurrentScene => {
                            self.pop()?
                        }
                        _ => {
                            return Err(GMError::UnknownMessageToScene(message))
                        }
                    }
                        }
                _ => {
                    return Err(GMError::UnknownMessageToScene(message))
                }
            }
        }

        Ok(())
    }

    pub(crate) fn draw(&mut self, context: &mut GMDrawContext) -> Result<(), GMError> {
        self.scenes[self.current_scene].draw(context)
    }
}
