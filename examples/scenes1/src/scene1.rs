
use green_moon_2d::scene::{GMSceneT, GMSceneResult};
use green_moon_2d::resource_manager::GMResourceManager;

use std::rc::Rc;

pub struct Scene1 {
    resources: Rc<GMResourceManager>,
}

impl Scene1 {
    pub fn new(resources: &Rc<GMResourceManager>) -> Box<dyn GMSceneT> {
        let result = Self {
            resources: resources.clone(),
        };
        Box::new(result)
    }
}

impl GMSceneT for Scene1 {
    fn init(&mut self) {

    }
    fn draw(&self) {

    }
    fn update(&mut self) {

    }
    fn event(&mut self) -> GMSceneResult {
        GMSceneResult::GMKeepScene
    }

}
