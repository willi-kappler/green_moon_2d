

use log::debug;

use crate::context::{GMUpdateContext, GMDrawContext};
use crate::error::GMError;

pub(crate) enum GMSceneMessage {
    AddScene(Box<dyn GMSceneT>),
    RemoveScene(String),
    ChangeToScene(String),
    ReplaceScene(Box<dyn GMSceneT>),
}

pub trait GMSceneT {
    fn update(&mut self, _context: &mut GMUpdateContext) -> Result<(), GMError> {
        Ok(())
    }

    fn draw(&mut self, _context: &mut GMDrawContext) -> Result<(), GMError> {
        Ok(())
    }

    fn get_name(&self) -> &str;
}


pub struct GMSceneManager {
    scenes: Vec<Box<dyn GMSceneT>>,
    current_scene: usize,
}

impl GMSceneManager {
    pub(crate) fn new() -> Self {
        Self {
            scenes: Vec::new(),
            current_scene: 0,
        }
    }

    fn get_scene_index(&self, name: &str) -> Option<usize> {
        debug!("GMSceneManager::get_scene_index(), name: '{}'", name);

        self.scenes.iter().position(|scene| scene.get_name() == name)
    }

    pub(crate) fn add_scene(&mut self, scene: Box<dyn GMSceneT>) -> Result<(), GMError> {
        let name = scene.get_name();

        debug!("GMSceneManager::add_sc(), name: '{}'", name);

        match self.get_scene_index(name) {
            Some(_) => {
                Err(GMError::SceneAlreadyExists(name.to_string()))
            }
            None => {
                self.scenes.push(scene);

                Ok(())
            }
        }
    }

    pub(crate) fn remove_scene(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMSceneManager::remove_scene(), name: '{}'", name);

        match self.get_scene_index(name) {
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

    pub(crate) fn change_to_scene(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMSceneManager::change_scene(), name: '{}'", name);

        match self.get_scene_index(name) {
            Some(index) => {
                self.current_scene = index;

                Ok(())
            }
            None => {
                Err(GMError::SceneNotFound(name.to_string()))
            }
        }
    }

    pub(crate) fn replace_scene(&mut self, scene: Box<dyn GMSceneT>) -> Result<(), GMError> {
        let name = scene.get_name();

        debug!("GMSceneManager::replace_scene(), name: '{}'", name);

        match self.get_scene_index(name) {
            Some(index) => {
                self.scenes[index] = scene;
                Ok(())
            }
            None => {
                Err(GMError::SceneNotFound(name.to_string()))
            }
        }
    }

    pub(crate) fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        self.scenes[self.current_scene].update(context)?;

        while let Some(message) = context.next_scene_message() {
            match message {
                GMSceneMessage::AddScene(scene) => {
                    self.add_scene(scene)?
                }
                GMSceneMessage::RemoveScene(name) => {
                    self.remove_scene(&name)?
                }
                GMSceneMessage::ChangeToScene(name) => {
                    self.change_to_scene(&name)?
                }
                GMSceneMessage::ReplaceScene(scene) => {
                    self.replace_scene(scene)?
                }
            }
        }

        Ok(())
    }

    pub(crate) fn draw(&mut self, context: &mut GMDrawContext) -> Result<(), GMError> {
        self.scenes[self.current_scene].draw(context)
    }
}
