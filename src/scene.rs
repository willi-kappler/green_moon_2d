
use std::fmt::Debug;

use log::debug;

use crate::context::{GMUpdateContext, GMDrawContext};
use crate::error::GMError;
use crate::message::{GMSender, GMReceiver, GMMessage, GMMessageData};
use crate::property::{GMValue};

pub trait GMSceneT : Debug {
    // Must be implemented:
    fn clone_box(&self) -> Box<dyn GMSceneT>;

    // May be implemented:
    fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        if let Some(child) = self.get_child_mut() {
            child.update(context)
        } else {
            panic!("Implement 'update()' or set a child!");
        }
    }

    fn draw(&self, context: &GMDrawContext) -> Result<(), GMError> {
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

    fn init(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        if let Some(child) = self.get_child_mut() {
            child.init(context)
        } else {
            Ok(())
        }
    }

    fn exit(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        if let Some(child) = self.get_child_mut() {
            child.exit(context)
        } else {
            Ok(())
        }
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

    fn set_child(&mut self, _child: Box<dyn GMSceneT>) {
    }

    fn remove_child(&mut self) {
    }

    fn take_child(&mut self) -> Option<Box<dyn GMSceneT>> {
        None
    }

    fn get_child_ref(&self) -> Option<&Box<dyn GMSceneT>> {
        None
    }

    fn get_child_mut(&mut self) -> Option<&mut Box<dyn GMSceneT>> {
        None
    }
}

impl Clone for Box<dyn GMSceneT> {
    fn clone(&self) -> Box<dyn GMSceneT> {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct GMSceneManager {
    scenes: Vec<(String, Box<dyn GMSceneT>)>,
    scene_stack: Vec<String>,
    current_scene_index: usize,
}

impl GMSceneManager {
    pub(crate) fn new() -> Self {
        Self {
            scenes: Vec::new(),
            scene_stack: Vec::new(),
            current_scene_index: 0,
        }
    }

    fn index(&self, name: &str) -> Option<usize> {
        debug!("GMSceneManager::get_scene_index(), name: '{}'", name);

        self.scenes.iter().position(|(scene_name, _)| scene_name == name)
    }

    pub(crate) fn add(&mut self, name: &str, scene: Box<dyn GMSceneT>) -> Result<(), GMError> {
        debug!("GMSceneManager::add_scene(), name: '{}'", name);

        match self.index(name) {
            Some(_) => {
                Err(GMError::SceneAlreadyExists(name.to_string()))
            }
            None => {
                self.scenes.push((name.to_string(), scene));

                Ok(())
            }
        }
    }

    fn take(&mut self, name: &str) -> Result<Box<dyn GMSceneT>, GMError> {
        debug!("GMSceneManager::take(), name: '{}'", name);

        match self.index(name) {
            Some(index) => {
                if self.current_scene_index == index {
                    Err(GMError::CantRemoveCurrentScene(name.to_string()))
                } else {
                    if self.current_scene_index == self.scenes.len() - 1 {
                        self.current_scene_index = index;
                    }

                    Ok(self.scenes.swap_remove(index).1)
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
                self.current_scene_index = index;

                Ok(())
            }
            None => {
                Err(GMError::SceneNotFound(name.to_string()))
            }
        }
    }

    fn replace(&mut self, name: &str, scene: Box<dyn GMSceneT>) -> Result<(), GMError> {
        debug!("GMSceneManager::replace_scene(), name: '{}'", name);

        match self.index(name) {
            Some(index) => {
                self.scenes[index].1 = scene;
                Ok(())
            }
            None => {
                Err(GMError::SceneNotFound(name.to_string()))
            }
        }
    }

    fn push(&mut self) {
        let name = &self.scenes[self.current_scene_index].0;

        debug!("GMSceneManager::push(), current scene: '{}'", name);

        self.scene_stack.push(name.to_string());
    }

    fn pop(&mut self) -> Result<(), GMError> {
        match self.scene_stack.pop() {
            Some(name) => {
                debug!("GMSceneManager::pop(), scene: '{}'", name);

                self.change_to(&name)
            }
            None => {
                Err(GMError::SceneStackEmpty)
            }
        }
    }

    pub(crate) fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        self.scenes[self.current_scene_index].1.update(context)?;

        use GMReceiver::*;
        use GMMessageData::*;

        while let Some(message) = context.next_scene_message() {
            let receiver = message.receiver.clone();

            match receiver {
                CurrentScene => {
                    self.scenes[self.current_scene_index].1.send_message(message, context)?;
                }
                Scene(name) => {
                    match self.index(&name) {
                        Some(index) => {
                            if let Some(message) = self.scenes[index].1.send_message(message, context)? {
                                context.send_message(message);
                            }
                        }
                        None => {
                            return Err(GMError::SceneNotFound(name))
                        }
                    }
                }
                SceneWithProperty(name) => {
                    for (_, scene) in self.scenes.iter_mut() {
                        if scene.has_property(&name) {
                            if let Some(message) = scene.send_message(message.clone(), context)? {
                                context.send_message(message);
                            }
                        }
                    }
                }
                SceneManager => {
                    match message.message_data {
                        AddScene(name, scene) => {
                            self.add(&name, scene)?
                        }
                        RemoveScene(name) => {
                            self.take(&name).map(|_| ())?
                        }
                        // TODO: Maybe add TakeScene message
                        ChangeToScene(name) => {
                            self.change_to(&name)?;
                            context.set_current_sender(GMSender::Scene(name));
                        }
                        ReplaceScene(name, scene) => {
                            self.replace(&name, scene)?
                        }
                        PushCurrentScene => {
                            self.push()
                        }
                        PopCurrentScene => {
                            self.pop()?
                        }
                        // TODO: SetSceneParent, RemoveSceneParent, ...
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
        self.scenes[self.current_scene_index].1.draw(context)
    }
}
