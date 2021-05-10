
use crate::error::GMError;
use crate::context::GMContext;

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

struct GMSceneWrapper {
    name: String,
    scene: Box<dyn GMScene>,
}

impl GMSceneWrapper {
    fn new<T: 'static + GMScene>(name: &str, scene: T) -> GMSceneWrapper {
        GMSceneWrapper {
            name: name.to_string(),
            scene: Box::new(scene),
        }
    }

    fn event(&mut self, context: &mut GMContext) {
        self.scene.event(context)
    }

    fn update(&mut self, context: &mut GMContext) {
        self.scene.update(context)
    }

    fn draw(&mut self, context: &mut GMContext) {
        self.scene.draw(context)
    }
}

pub struct GMSceneManager {
    scenes: Vec<GMSceneWrapper>,
    current_scene: usize,
}

impl GMSceneManager {
    pub(crate) fn new() -> GMSceneManager {
        let scenes = vec![GMSceneWrapper::new("empty", GMEmptyScene{})];

        GMSceneManager {
            scenes: scenes,
            current_scene: 0,
        }
    }

    fn has_scene(&self, scene_name: &str) -> bool {
        for scene in self.scenes.iter() {
            if scene_name == scene.name {
                return true
            }
        }

        return false
    }

    fn get_scene_index(&self, scene_name: &str) -> Result<usize, GMError> {
        for (i, scene) in self.scenes.iter().enumerate() {
            if scene_name == scene.name {
                return Ok(i)
            }
        }

        Err(GMError::SceneNameNotFound(scene_name.to_string()))
    }

    pub fn add_scene<T: 'static + GMScene>(&mut self, scene_name: &str, new_scene: T) -> Result<(), GMError>{
        if self.has_scene(scene_name) {
            return Err(GMError::SceneNameAlreadyInUse(scene_name.to_string()))
        }

        self.scenes.push(GMSceneWrapper::new(scene_name, new_scene));

        Ok(())
    }

    pub fn remove_scene(&mut self, scene_name: &str) -> Result<(), GMError> {
        let index = self.get_scene_index(scene_name)?;

        if index == self.current_scene {
            return Err(GMError::CantRemoveCurrentScene(scene_name.to_string()));
        }

        self.scenes.remove(index);

        if index < self.current_scene {
            self.current_scene -= 1;
        }

        Ok(())
    }

    pub fn set_current_scene(&mut self, scene_name: &str) -> Result<(), GMError> {
        let index = self.get_scene_index(scene_name)?;
        self.current_scene = index;

        Ok(())
    }

    pub fn event(&mut self, context: &mut GMContext) {
        self.scenes[self.current_scene].event(context);
    }

    pub fn update(&mut self, context: &mut GMContext) {
        self.scenes[self.current_scene].update(context);
    }

    pub fn draw(&mut self, context: &mut GMContext) {
        self.scenes[self.current_scene].draw(context);
    }
}
