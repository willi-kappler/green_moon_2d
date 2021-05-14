use std::time::Duration;
use std::thread;

use crate::scene::{GMSceneWrapper, GMScene, GMEmptyScene, GMSceneState};
use crate::context::GMContext;
use crate::resource_manager::GMResourceManager;
use crate::error::GMError;

pub struct GreenMoon2D {
    context: GMContext,
    scene_manager: GMResourceManager<GMSceneWrapper>,
}

impl GreenMoon2D {
    pub fn run(&mut self) -> Result<(), GMError> {
        let mut current_scene = self.scene_manager.get_item("Empty")?;
        let mut scene_state = GMSceneState::Stay;

        loop {
            let first_tick = self.context.elapsed();

            if self.context.quit {
                break;
            }

            if let GMSceneState::Switch(new_scene) = scene_state {
                current_scene = self.scene_manager.get_item(&new_scene)?;
                let mut scene = current_scene.borrow_mut();
                scene.enter(&mut self.context);
            }

            let mut scene = current_scene.borrow_mut();

            scene.event(&mut self.context); // TODO: Pass event
            scene_state = scene.update(&mut self.context);
            scene.draw(&mut self.context);

            let second_tick = self.context.elapsed();

            let duration = second_tick - first_tick;
            self.context.current_fps = 1000.0 / duration;
            let diff = self.context.expected_duration - duration;

            if diff > 0.0 {
                thread::sleep(Duration::from_millis(diff as u64));
            }
        };

        Ok(())
    }
}


pub fn init(path_to_configuration: &str) -> Result<GreenMoon2D, GMError> {
    // TODO: Read in configuration and return a GM item

    let empty_scene = GMSceneWrapper::new("Empty", GMEmptyScene{});

    let mut scene_manager = GMResourceManager::new("SceneManager");
    scene_manager.add_item(empty_scene)?;

    let gm = GreenMoon2D {
        context: GMContext::new(),
        scene_manager,
    };

    Ok(gm)
}
