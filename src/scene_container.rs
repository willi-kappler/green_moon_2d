

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

    pub fn add_scene<T: 'static + GMSceneT>(&mut self, name: &str, new_scene: T) {
        self.scenes.push((name.to_string(), Box::new(new_scene)));
    }

    pub fn first_scene(&mut self) -> &mut Box<dyn GMSceneT> {
        &mut self.scenes[0].1
    }

    pub fn get_scene(&mut self, name: &str) -> Result<&mut Box<dyn GMSceneT>, GMError> {
        match self.scenes.iter_mut().find(|(stored_name, _)| stored_name == name) {
            Some(item) => {
                Ok(&mut item.1)
            }
            None => {
                Err(GMError::SceneNotFound(name.to_string()))
            }
        }
    }
}
