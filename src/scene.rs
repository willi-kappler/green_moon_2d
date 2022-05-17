
use std::fmt::Debug;

use log::debug;

use crate::context::{GMUpdateContext, GMDrawContext};
use crate::message::{GMSceneMessage};
// use crate::property::{GMValue};

pub trait GMSceneT : Debug {
    // Must be implemented:
    fn clone_box(&self) -> Box<dyn GMSceneT>;

    // May be implemented:
    fn update(&mut self, context: &mut GMUpdateContext) {
    }

    fn draw(&self, context: &GMDrawContext) {
    }

    fn send_message(&mut self, message: GMSceneMessage, context: &mut GMUpdateContext) {
    }

    fn init(&mut self, context: &mut GMUpdateContext) {
    }

    fn exit(&mut self, context: &mut GMUpdateContext) {
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

    pub(crate) fn add(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        debug!("GMSceneManager::add_scene(), name: '{}'", name);

        match self.index(name) {
            Some(_) => {
                panic!("A scene with name {} already exists!", name);
            }
            None => {
                self.scenes.push((name.to_string(), scene));
            }
        }
    }

    fn change_to(&mut self, name: &str) {
        debug!("GMSceneManager::change_scene(), name: '{}'", name);

        match self.index(name) {
            Some(index) => {
                self.current_scene_index = index;

            }
            None => {
                panic!("A scene with name {} does not exist!", name);
            }
        }
    }

    fn replace(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        debug!("GMSceneManager::replace_scene(), name: '{}'", name);

        match self.index(name) {
            Some(index) => {
                self.scenes[index].1 = scene;
            }
            None => {
                panic!("A scene with name {} does not exist!", name);
            }
        }
    }

    fn push(&mut self) {
        let name = &self.scenes[self.current_scene_index].0;

        debug!("GMSceneManager::push(), current scene: '{}'", name);

        self.scene_stack.push(name.to_string());
    }

    fn pop(&mut self) {
        match self.scene_stack.pop() {
            Some(name) => {
                debug!("GMSceneManager::pop(), scene: '{}'", name);

                self.change_to(&name)
            }
            None => {
                panic!("The scene stack is empty!");
            }
        }
    }

    pub(crate) fn update(&mut self, context: &mut GMUpdateContext) {
        self.scenes[self.current_scene_index].1.update(context);


        while let Some(message) = context.next_scene_message() {
            /*
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
            */
        }
    }

    pub(crate) fn draw(&mut self, context: &mut GMDrawContext) {
        self.scenes[self.current_scene_index].1.draw(context)
    }
}
