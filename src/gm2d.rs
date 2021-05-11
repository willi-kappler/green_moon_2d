use std::rc::Rc;
use std::cell::RefCell;

use crate::scene::{GMSceneWrapper, GMScene, GMEmptyScene};
use crate::context::GMContext;
use crate::resource_manager::GMResourceManager;

pub struct GreenMoon2D {
    context: GMContext,
    scene_manager: GMResourceManager<GMSceneWrapper>,
    current_scene: Rc<RefCell<GMSceneWrapper>>,
}

impl GreenMoon2D {
    pub fn run(&mut self) {
        loop {
            if self.context.exit_game() {
                break;
            }

            let mut scene = self.current_scene.borrow_mut();

            scene.event(&mut self.context);
            scene.update(&mut self.context);
            scene.draw(&mut self.context);
        }
    }
}


pub fn init(path_to_configuration: &str) -> GreenMoon2D {
    // TODO: Read in configuration and return a GM item

    let scene = GMSceneWrapper::new("Empty", GMEmptyScene{});

    GreenMoon2D {
        context: GMContext::new(),
        scene_manager: GMResourceManager::new("SceneManager"),
        current_scene: Rc::new(RefCell::new(scene)),
    }
}
