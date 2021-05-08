use crate::scene::GMSceneManager;
use crate::context::GMContext;

pub struct GreenMoon2D {
    context: GMContext,
    scene_manager: GMSceneManager,
}

impl GreenMoon2D {
    pub fn run(&mut self) {
        loop {
            if self.context.exit_game() {
                break;
            }

            self.scene_manager.event(&mut self.context);
            self.scene_manager.update(&mut self.context);
            self.scene_manager.draw(&mut self.context);
        }
    }
}


pub fn init(path_to_configuration: &str) -> GreenMoon2D {
    todo!("Read in configuration and return a GM item");

    GreenMoon2D{
        context: GMContext::new(),
        scene_manager: GMSceneManager::new(),
    }
}
