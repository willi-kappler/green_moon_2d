
use crate::error::{GMError};

pub enum GMSSceneState {
    Enter,
    Run,
    Leave,
}

pub trait GMScene {
    fn update(&mut self) {

    }

    fn draw(&self) {

    }

    fn event(&mut self) {
    }
}

pub struct GMSceneManager {
    scenes: Vec<(String, Box<dyn GMScene>)>,
    current_scene: usize,
}

impl GMSceneManager {
    pub fn new() -> GMSceneManager {
        GMSceneManager {
            scenes: Vec::new(),
            current_scene: 0,
        }
    }

    pub fn has_scene(&self, scene_name: &str) -> bool {
        for (name, _) in self.scenes.iter() {
            if scene_name == name {
                return true
            }
        }

        return false
    }

    fn get_scene_index(&self, scene_name: &str) -> Result<usize, GMError> {
        for (i, (name, _)) in self.scenes.iter().enumerate() {
            if scene_name == name {
                return Ok(i)
            }
        }

        Err(GMError::SceneNameNotFound(scene_name.to_string()))
    }

    pub fn add_scene<T: 'static + GMScene>(&mut self, scene_name: &str, new_scene: T) -> Result<(), GMError>{
        if self.has_scene(scene_name) {
            return Err(GMError::SceneNameAlreadyInUse(scene_name.to_string()))
        }

        self.scenes.push((scene_name.to_string(), Box::new(new_scene)));

        Ok(())
    }

    pub fn remove_scene(&mut self, scene_name: &str) -> Result<(), GMError> {
        let index = self.get_scene_index(scene_name)?;
        self.scenes.remove(index);

        Ok(())
    }

    pub fn set_current_scene(&mut self, scene_name: &str) -> Result<(), GMError> {
        let index = self.get_scene_index(scene_name)?;
        self.current_scene = index;

        Ok(())
    }
}
