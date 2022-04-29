

use log::debug;

use crate::context::{GMUpdateContext, GMDrawContext};
use crate::error::GMError;
use crate::object::GMObjectMessage;


pub trait GMSceneT {
    fn init(&mut self, _context: &mut GMUpdateContext) -> Result<(), GMError> {
        Ok(())
    }

    fn update(&mut self, _context: &mut GMUpdateContext) -> Result<(), GMError>;

    fn draw(&mut self, _context: &mut GMDrawContext) -> Result<(), GMError>;

    fn exit(&mut self, _context: &mut GMUpdateContext) -> Result<(), GMError> {
        Ok(())
    }

    fn get_name(&self) -> &str;

    fn send_message(&mut self, message: GMObjectMessage);
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

    fn get_scene_index(&self, name: &str) -> Option<usize> {
        debug!("GMSceneManager::get_scene_index(), name: '{}'", name);

        self.scenes.iter().position(|scene| scene.get_name() == name)
    }

    pub(crate) fn add_scene(&mut self, scene: Box<dyn GMSceneT>) -> Result<(), GMError> {
        let name = scene.get_name();

        debug!("GMSceneManager::add_scene(), name: '{}'", name);

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

    fn remove_scene(&mut self, name: &str) -> Result<(), GMError> {
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

    fn change_to_scene(&mut self, name: &str) -> Result<(), GMError> {
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

    fn replace_scene(&mut self, scene: Box<dyn GMSceneT>) -> Result<(), GMError> {
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
                    self.scenes[self.current_scene].send_message(message);
                }
            }
        }

        Ok(())
    }

    pub(crate) fn draw(&mut self, context: &mut GMDrawContext) -> Result<(), GMError> {
        self.scenes[self.current_scene].draw(context)
    }
}
