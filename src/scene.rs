
use crate::error::{GMError};
use crate::context::{GMContext};

pub enum GMSSceneState {
    Enter,
    Run,
    Leave,
}

pub trait GMScene {
    fn update(&mut self, context: &mut GMContext) {

    }

    fn draw(&mut self, context: &mut GMContext) {

    }

    fn event(&mut self, context: &mut GMContext) {

    }
}

struct GMEmptyScene {

}

impl GMScene for GMEmptyScene {

}

pub struct GMSceneManager {
    scenes: Vec<(String, Box<dyn GMScene>)>,
    current_scene: usize,
}

impl GMSceneManager {
    pub(crate) fn new() -> GMSceneManager {
        let scene: Box<dyn GMScene> = Box::new(GMEmptyScene{});
        let scenes = vec![("empty".to_string(), scene)];

        GMSceneManager {
            scenes: scenes,
            current_scene: 0,
        }
    }

    fn has_scene(&self, scene_name: &str) -> bool {
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

    pub(crate) fn add_scene<T: 'static + GMScene>(&mut self, scene_name: &str, new_scene: T) -> Result<(), GMError>{
        if self.has_scene(scene_name) {
            return Err(GMError::SceneNameAlreadyInUse(scene_name.to_string()))
        }

        self.scenes.push((scene_name.to_string(), Box::new(new_scene)));

        Ok(())
    }

    pub(crate) fn remove_scene(&mut self, scene_name: &str) -> Result<(), GMError> {
        let index = self.get_scene_index(scene_name)?;
        self.scenes.remove(index);

        Ok(())
    }

    pub(crate) fn set_current_scene(&mut self, scene_name: &str) -> Result<(), GMError> {
        let index = self.get_scene_index(scene_name)?;
        self.current_scene = index;

        Ok(())
    }

    pub(crate) fn event(&mut self, context: &mut GMContext) {
        self.scenes[self.current_scene].1.event(context);
    }

    pub(crate) fn update(&mut self, context: &mut GMContext) {
        self.scenes[self.current_scene].1.update(context);
    }

    pub(crate) fn draw(&mut self, context: &mut GMContext) {
        self.scenes[self.current_scene].1.draw(context);
    }
}
