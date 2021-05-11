
use crate::context::GMContext;
use crate::resource_manager::GMName;

pub enum GMSceneState {
    Stay,
    Switch(String),
}

pub trait GMScene {
    fn enter(&mut self, context: &mut GMContext) {

    }

    fn update(&mut self, context: &mut GMContext) -> GMSceneState {
        GMSceneState::Stay
    }

    fn draw(&self, context: &mut GMContext) {

    }

    fn event(&mut self, context: &mut GMContext) {

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
    fn enter(&mut self, context: &mut GMContext) {
        self.scene.enter(context)
    }

    fn event(&mut self, context: &mut GMContext) {
        self.scene.event(context)
    }

    fn update(&mut self, context: &mut GMContext) -> GMSceneState {
        self.scene.update(context)
    }

    fn draw(&self, context: &mut GMContext) {
        self.scene.draw(context)
    }
}

