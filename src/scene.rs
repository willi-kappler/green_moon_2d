
use std::fmt::Debug;

use log::debug;

use crate::context::{GMUpdateContext, GMDrawContext};
use crate::message::{GMSceneManagerMessage, GMSceneMessage};
// use crate::property::{GMValue};

pub trait GMSceneT : Debug {
    // Must be implemented:
    fn clone_box(&self) -> Box<dyn GMSceneT>;

    // May be implemented:
    fn send_message(&mut self, _message: GMSceneMessage, _context: &mut GMUpdateContext) {
    }

    fn update(&mut self, _context: &mut GMUpdateContext) {
    }

    fn draw(&self, _context: &GMDrawContext) {
    }

    fn init(&mut self, _context: &mut GMUpdateContext) {
    }

    fn exit(&mut self, _context: &mut GMUpdateContext) {
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

    pub(crate) fn get_name(&self, index: usize) -> &str {
        debug!("GMSceneManager::get_name(), index: {}", index);

        if self.scenes.len() == 0 {
            panic!("No scenes found, please add a scene first before initializing the engine!");
        } else {
            &self.scenes[index].0
        }
    }

    fn scene_index(&self, name: &str) -> Option<usize> {
        debug!("GMSceneManager::scene_index(), name: '{}'", name);

        self.scenes.iter().position(|(scene_name, _)| scene_name == name)
    }

    pub(crate) fn add_scene(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        debug!("GMSceneManager::add_scene(), name: '{}'", name);

        match self.scene_index(name) {
            Some(_) => {
                panic!("A scene with name {} already exists!", name);
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
                dbg!(index);
                todo!();
            }
            None => {
                panic!("A scene with name {} does not exist!", name);
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
                panic!("A scene with name {} does not exist!", name);
            }
        }
    }

    fn replace_scene(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        debug!("GMSceneManager::replace_scene(), name: '{}'", name);

        match self.scene_index(name) {
            Some(index) => {
                self.scenes[index].1 = scene;
            }
            None => {
                panic!("A scene with name {} does not exist!", name);
            }
        }
    }

    fn push_and_change_scene(&mut self, name: &str) {
        let current_name = &self.scenes[self.current_scene_index].0;

        debug!("GMSceneManager::push_and_change_scene(), current scene: '{}', next scene: {}",
            current_name, name);

        self.scene_stack.push(current_name.to_string());
        self.change_to_scene(name);
    }

    fn pop_and_change_scene(&mut self, context: &mut GMUpdateContext) {
        match self.scene_stack.pop() {
            Some(name) => {
                debug!("GMSceneManager::pop_and_change_scene(), scene: '{}'", name);

                self.change_to_scene(&name);
                context.set_mode_scene(&name);
            }
            None => {
                panic!("The scene stack is empty!");
            }
        }
    }

    fn message_to_current_scene(&mut self, message: GMSceneMessage, context: &mut GMUpdateContext) {
        self.scenes[self.current_scene_index].1.send_message(message, context);
    }

    fn message_to_scene(&mut self, name: &str, message: GMSceneMessage, context: &mut GMUpdateContext) {
        match self.scene_index(name) {
            Some(index) => {
                self.scenes[index].1.send_message(message, context);
            }
            None => {
                panic!("A scene with name {} does not exist!", name);
            }
        }
    }

    pub(crate) fn update(&mut self, context: &mut GMUpdateContext) {
        self.scenes[self.current_scene_index].1.update(context);

        use GMSceneManagerMessage::*;

        while let Some(message) = context.next_scene_message() {
            match message {
                AddScene(name, scene) => {
                    self.add_scene(&name, scene);
                }
                RemoveScene(name) => {
                    self.remove_scene(&name);
                }
                ReplaceScene(name, scene) => {
                    self.replace_scene(&name, scene);
                }
                PushAndChangeScene(name) => {
                    self.push_and_change_scene(&name);
                }
                PopAndChangeScene => {
                    self.pop_and_change_scene(context);
                }
                ChangeToScene(name) => {
                    self.change_to_scene(&name);
                    context.set_mode_scene(&name);
                }

                MessageToCurrentScene(message) => {
                    self.message_to_current_scene(message, context);
                }
                MessageToScene(name, message) => {
                    self.message_to_scene(&name, message, context);
                }
            }
        }
    }

    pub(crate) fn draw(&mut self, context: &mut GMDrawContext) {
        self.scenes[self.current_scene_index].1.draw(context);
    }
}
