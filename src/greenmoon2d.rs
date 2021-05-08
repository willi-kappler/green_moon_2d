use crate::scene::GMSceneManager;
use crate::context::GMContext;

pub struct GreenMoon2D {
    context: GMContext,
    scene_manager: GMSceneManager,
}

impl GreenMoon2D {
    pub fn run(&self) {

    }
}


pub fn init(path_to_configuration: &str) -> GreenMoon2D {
    todo!("Read in configuration and return a GM item");

    GreenMoon2D{
        context: GMContext::new(),
        scene_manager: GMSceneManager::new(),
    }
}
