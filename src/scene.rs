
use crate::resource_manager::GMName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GMSceneState {
    Stay,
    Switch(String),
}

pub trait GMScene {
    fn enter(&mut self) {

    }

    fn update(&mut self) -> GMSceneState {
        GMSceneState::Stay
    }

    fn draw(&self) {

    }

    fn event(&mut self) {

    }
}

pub struct GMEmptyScene {

}

impl GMScene for GMEmptyScene {

}

pub struct GMSceneWrapper {
    pub(crate) name: String,
    pub(crate) scene: Box<dyn GMScene>,
}

impl GMSceneWrapper {
    pub fn new<T: 'static + GMScene>(name: &str, scene: T) -> GMSceneWrapper {
        GMSceneWrapper {
            name: name.to_string(),
            scene: Box::new(scene),
        }
    }
}

impl GMName for GMSceneWrapper {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn has_name(&self, name: &str) -> bool {
        self.name == name
    }

    fn has_prefix(&self, name: &str) -> bool {
        self.name.starts_with(name)
    }
}

impl GMScene for GMSceneWrapper {
    fn enter(&mut self) {
        self.scene.enter()
    }

    fn update(&mut self) -> GMSceneState {
        self.scene.update()
    }

    fn draw(&self) {
        self.scene.draw()
    }

    fn event(&mut self) {
        self.scene.event()
    }
}
