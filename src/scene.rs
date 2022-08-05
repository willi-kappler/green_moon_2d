
use std::any::Any;
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
    AddScene(Box<dyn GMSceneT>),
    ChangeToScene(String),
    PopAndChangeScene,
    PushAndChangeScene(String),
    RemoveScene(String),
    ReplaceScene(String, Box<dyn GMSceneT>),
    SendMessage(String, String, Option<Box<dyn Any>>),
}

pub trait CloneBox {
    fn clone_box(&self) -> Box<dyn GMSceneT>;
}

impl<T> CloneBox for T where T: Clone + GMSceneT + 'static {
    fn clone_box(&self) -> Box<dyn GMSceneT> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn GMSceneT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait GMSceneT: Debug + CloneBox {
    fn get_name(&self) -> &str;

    fn update(&mut self, context: &mut GMContext);

    fn draw(&self, context: &mut GMContext);

    fn send_message(&mut self, _message: &str, _data: Option<Box<dyn Any>>, _context: &mut GMContext) {
        error_panic(&format!("GMSceneT::send_message() is not implemented for this scene: '{}'", self.get_name()));
    }
}

pub struct GMSceneManager {
    scenes: Vec<Box<dyn GMSceneT>>,
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
            &self.scenes[index].get_name()
        }
    }

    fn get_current_name(&self) -> &str {
        self.get_name(self.current_scene_index)
    }

    fn scene_index(&self, name: &str) -> Option<usize> {
        debug!("GMSceneManager::scene_index(), name: '{}'", name);

        self.scenes.iter().position(|scene| name == scene.get_name())
    }

    pub(crate) fn add_scene(&mut self, scene: Box<dyn GMSceneT>) {
        let name = scene.get_name();

        debug!("GMSceneManager::add_scene(), name: '{}'", name);

        match self.scene_index(name) {
            Some(_) => {
                error_panic(&format!("A scene with name '{}' already exists!", name));
            }
            None => {
                self.scenes.push(scene);
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

    fn change_to_scene(&mut self, name: &str) {
        debug!("GMSceneManager::change_to_scene(), name: '{}'", name);

        match self.scene_index(name) {
            Some(index) => {
                self.current_scene_index = index;
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
                    self.scenes[index] = scene;
                }
            }
            None => {
                self.scene_does_not_exist("GMSceneManager::replace_scene()", name);
            }
        }
    }

    fn push_and_change_scene(&mut self, name: &str) {
        let current_name = self.get_current_name().to_string();
        debug!("GMSceneManager::push_and_change_scene(), current scene: '{}', next scene: {}",
            current_name, name);

        self.scene_stack.push(current_name);
        self.change_to_scene(name);
    }

    fn pop_and_change_scene(&mut self) {
        match self.scene_stack.pop() {
            Some(name) => {
                let current_name = self.get_current_name();
                debug!("GMSceneManager::pop_and_change_scene(), current scene: '{}', scene: '{}'",
                    current_name, name);

                self.change_to_scene(&name);
            }
            None => {
                error_panic("The scene stack is empty!");
            }
        }
    }

    fn send_message(&mut self, scene: &str, message: &str, data: Option<Box<dyn Any>>, context: &mut GMContext) {
        debug!("GMSceneManager::send_message(), name: '{}', message: '{}'", scene, message);

        match self.scene_index(scene) {
            Some(index) => {
                self.scenes[index].send_message(message, data, context);
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
                AddScene(scene) => {
                    self.add_scene(scene);
                }
                ChangeToScene(name) => {
                    self.change_to_scene(&name);
                }
                PopAndChangeScene => {
                    self.pop_and_change_scene();
                }
                PushAndChangeScene(name) => {
                    self.push_and_change_scene(&name);
                }
                RemoveScene(name) => {
                    self.remove_scene(&name);
                }
                ReplaceScene(name, scene) => {
                    self.replace_scene(&name, scene);
                }
                SendMessage(scene, message, data) => {
                    self.send_message(&scene, &message, data, context);
                }
            }
        }

        self.scenes[self.current_scene_index].update(context);
    }

    pub(crate) fn draw(&self, context: &mut GMContext) {
        self.scenes[self.current_scene_index].draw(context);
    }
}
