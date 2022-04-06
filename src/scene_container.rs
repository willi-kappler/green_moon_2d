

use log::debug;

use crate::error::GMError;
use crate::scene::GMSceneT;


pub struct GMSceneContainer {
    scenes: Vec<(String, Box<dyn GMSceneT>)>
}

impl GMSceneContainer {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
        }
    }

    pub fn add_scene<T: 'static + GMSceneT>(&mut self, name: &str, new_scene: T) -> Result<(), GMError> {
        debug!("GMSceneContainer::add_scene(), name: '{}'", name);

        for (s_name, _) in self.scenes.iter() {
            if s_name == name {
                return Err(GMError::SceneAlreadyExists(name.to_string()))
            }
        }

        self.scenes.push((name.to_string(), Box::new(new_scene)));

        Ok(())
    }

    pub fn first_scene(&mut self) -> &mut Box<dyn GMSceneT> {
        debug!("GMSceneContainer::first_scene()");

        &mut self.scenes[0].1
    }

    pub fn get_scene_mut(&mut self, name: &str) -> Result<&mut Box<dyn GMSceneT>, GMError> {
        debug!("GMSceneContainer::get_scene(), name: '{}'", name);

        for (s_name, scene) in self.scenes.iter_mut() {
            if s_name == name {
                return Ok(scene)
            }
        }

        Err(GMError::SceneNotFound(name.to_string()))
    }

    pub fn remove_scene(&mut self, _name: &str) -> Result<(), GMError> {
        todo!();
    }
}
