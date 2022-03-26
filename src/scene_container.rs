

use crate::error::GMError;
use crate::scene::GMScene;


pub struct GMSceneContainer {
    scenes: Vec<(String, Box<dyn GMScene>)>
}

impl GMSceneContainer {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
        }
    }

    pub fn add_scene<T: 'static + GMScene>(&mut self, name: &str, new_scene: T) {
        self.scenes.push((name.to_string(), Box::new(new_scene)));
    }

    pub fn first_scene(&mut self) -> &mut Box<dyn GMScene> {
        &mut self.scenes[0].1
    }

    pub fn get_scene(&mut self, name: &str) -> Result<&mut Box<dyn GMScene>, GMError> {
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
