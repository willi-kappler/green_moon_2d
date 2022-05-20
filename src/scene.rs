
use std::fmt::Debug;

use log::debug;

use crate::context::{GMContext};
use crate::message::{GMSceneManagerMessage, GMSceneMessage, GMSceneReply};
// use crate::property::{GMValue};

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
    #[allow(unused_variables)]
    fn send_message(&mut self, message: GMSceneMessage, context: &mut GMContext) -> GMSceneReply {
        GMSceneReply::Empty
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
        panic!("{}, scene with name '{}' does not exist!", location, name);
    }

    pub(crate) fn get_name(&self, index: usize) -> &str {
        debug!("GMSceneManager::get_name(), index: '{}'", index);

        if self.scenes.len() == 0 {
            panic!("No scenes found, please add a scene first");
        } else {
            &self.scenes[index].0
        }
    }

    fn get_current_name(&self) -> &str {
        self.get_name(self.current_scene_index)
    }

    fn scene_index(&self, name: &str) -> Option<usize> {
        debug!("GMSceneManager::scene_index(), name: '{}'", name);

        self.scenes.iter().position(|(scene_name, _)| scene_name == name)
    }

    pub(crate) fn add_scene(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        debug!("GMSceneManager::add_scene(), name: '{}'", name);

        match self.scene_index(name) {
            Some(_) => {
                panic!("A scene with name '{}' already exists!", name);
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
                    panic!("Can't replace current active scene: '{}'!", name);
                } else {
                    self.scenes[index].1 = scene;
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
                panic!("The scene stack is empty!");
            }
        }
    }

    fn message_to_current_scene(&mut self, message: GMSceneMessage, context: &mut GMContext) {
        self.scenes[self.current_scene_index].1.send_message(message, context);
    }

    fn message_to_scene(&mut self, name: &str, message: GMSceneMessage, context: &mut GMContext) {
        match self.scene_index(name) {
            Some(index) => {
                self.scenes[index].1.send_message(message, context);
            }
            None => {
                self.scene_does_not_exist("GMSceneManager::message_to_scene()", name);
            }
        }
    }

    pub(crate) fn update(&mut self, context: &mut GMContext) {
        context.reply_to_current_scene();
        self.message_to_current_scene(GMSceneMessage::Update, context);

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
                    self.pop_and_change_scene();
                }
                ChangeToScene(name) => {
                    self.message_to_current_scene(GMSceneMessage::Enter, context);
                    self.change_to_scene(&name);
                    self.message_to_current_scene(GMSceneMessage::Leave, context);
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
}
