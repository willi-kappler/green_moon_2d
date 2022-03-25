

use crate::error::GMError;
use crate::scene::{GMSceneState, GMScene};
pub struct GMApp {
    scenes: Vec<Box<dyn GMScene>>,
}

impl GMApp {
    pub fn new<S: 'static + GMScene>(first_scene: S) -> Self {
        Self {
            scenes: vec![Box::new(first_scene)],
        }
    }

    pub fn run(&mut self) -> Result<(), GMError> {
        let current_scene = &mut self.scenes[0];
        let mut current_state = GMSceneState::Enter;

        loop {
            let new_state = match current_state {
                GMSceneState::Enter => {
                    current_scene.enter()
                }
                GMSceneState::Run => {
                    current_scene.run()
                }
                GMSceneState::Leave => {
                    current_scene.leave()
                }
                GMSceneState::NewScene(ref new_scene) => {
                    // TODO:
                    Ok(GMSceneState::Enter)
                }
                GMSceneState::Quit => {
                    return Ok(())
                }
            };

            match new_state {
                Ok(state) => {

                }
                Err(e) => {
                    return Err(e)
                }
            }
        }
    }
}
