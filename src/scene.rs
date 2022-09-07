
use std::fmt::Debug;

use log::debug;

use crate::context::{GMContext};
use crate::util::error_panic;

#[derive(Debug)]
pub enum GMSceneState {
    Enter,
    Run,
    Leave,
}

#[derive(Debug)]
pub(crate) enum GMSceneManagerMessage {
    AddScene(String, Box<dyn GMSceneT>),
    ChangeToScene(String),
    PopAndChangeScene,
    PushAndChangeScene(String),
    RemoveScene(String),
    ReplaceScene(String, Box<dyn GMSceneT>),
    SendMessage(String, String),
}

pub trait GMSceneT: Debug {
    fn enter(&mut self, _context: &mut GMContext) {
    }

    fn update(&mut self, context: &mut GMContext);

    fn draw(&self, context: &mut GMContext);

    fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
    }
}

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

    fn scene_does_not_exist(&self, location: &str, name: &str) {
        error_panic(&format!("{}, scene with name '{}' does not exist!", location, name));
    }

    pub(crate) fn get_name(&self, index: usize) -> &str {
        debug!("GMSceneManager::get_name(), index: '{}'", index);

        if self.scenes.len() == 0 {
            error_panic("No scenes found, please add a scene first");
        } else {
            &self.scenes[index].0
        }
    }

    fn get_current_name(&self) -> &str {
        self.get_name(self.current_scene_index)
    }

    fn scene_index(&self, name: &str) -> Option<usize> {
        debug!("GMSceneManager::scene_index(), name: '{}'", name);

        self.scenes.iter().position(|scene| name == scene.0)
    }

    pub(crate) fn add_scene(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        debug!("GMSceneManager::add_scene(), name: '{}'", name);

        match self.scene_index(name) {
            Some(_) => {
                error_panic(&format!("A scene with name '{}' already exists!", name));
            }
            None => {
                self.scenes.push((name.to_string(), scene));
            }
        }
    }

    fn remove_scene(&mut self, name: &str) {
        debug!("GMSceneManager::remove_scene(), name: '{}'", name);

        match self.scene_index(name) {
            Some(index) => {
                // TODO: if current scene: error, else swap_remove()
                todo!("remove scene: '{}'", index);
            }
            None => {
                self.scene_does_not_exist("GMSceneManager::remove_scene()", name);
            }
        }
    }

    fn change_to_scene(&mut self, name: &str, context: &mut GMContext) {
        debug!("GMSceneManager::change_to_scene(), name: '{}'", name);

        match self.scene_index(name) {
            Some(index) => {
                self.current_scene_index = index;
                self.scenes[index].1.enter(context);
            }
            None => {
                self.scene_does_not_exist("GMSceneManager::change_to_scene()", name);
            }
        }
    }

    fn replace_scene(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        debug!("GMSceneManager::replace_scene(), name: '{}'", name);

        match self.scene_index(name) {
            Some(index) => {
                if self.current_scene_index == index {
                    error_panic(&format!("Can't replace current active scene: '{}'!", name));
                } else {
                    self.scenes[index] = (name.to_string(), scene);
                }
            }
            None => {
                self.scene_does_not_exist("GMSceneManager::replace_scene()", name);
            }
        }
    }

    fn push_and_change_scene(&mut self, name: &str, context: &mut GMContext) {
        let current_name = self.get_current_name().to_string();
        debug!("GMSceneManager::push_and_change_scene(), current scene: '{}', next scene: {}",
            current_name, name);

        self.scene_stack.push(current_name);
        self.change_to_scene(name, context);
    }

    fn pop_and_change_scene(&mut self, context: &mut GMContext) {
        match self.scene_stack.pop() {
            Some(name) => {
                let current_name = self.get_current_name();
                debug!("GMSceneManager::pop_and_change_scene(), current scene: '{}', next scene: '{}'",
                    current_name, name);

                self.change_to_scene(&name, context);
            }
            None => {
                error_panic("The scene stack is empty!");
            }
        }
    }

    fn send_message(&mut self, scene: &str, message: &str, context: &mut GMContext) {
        debug!("GMSceneManager::send_message(), name: '{}', message: '{}'", scene, message);

        match self.scene_index(scene) {
            Some(index) => {
                self.scenes[index].1.send_message(message, context);
            }
            None => {
                self.scene_does_not_exist("GMSceneManager::send_message()", scene);
            }
        }
    }

    pub(crate) fn update(&mut self, context: &mut GMContext) {
        use GMSceneManagerMessage::*;

        while let Some(message) = context.next_scene_message() {
            match message {
                AddScene(name, scene) => {
                    self.add_scene(&name, scene);
                }
                ChangeToScene(name) => {
                    self.change_to_scene(&name, context);
                }
                PopAndChangeScene => {
                    self.pop_and_change_scene(context);
                }
                PushAndChangeScene(name) => {
                    self.push_and_change_scene(&name, context);
                }
                RemoveScene(name) => {
                    self.remove_scene(&name);
                }
                ReplaceScene(name, scene) => {
                    self.replace_scene(&name, scene);
                }
                SendMessage(scene, message) => {
                    self.send_message(&scene, &message, context);
                }
            }
        }

        self.scenes[self.current_scene_index].1.update(context);
    }

    pub(crate) fn draw(&self, context: &mut GMContext) {
        self.scenes[self.current_scene_index].1.draw(context);
    }
}
