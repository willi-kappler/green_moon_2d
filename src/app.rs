
use std::time::{Duration, Instant};
use std::thread::sleep;


use crate::configuration::GMConfiguration;
use crate::context::{GMContext, GMSceneState};
use crate::error::GMError;
use crate::scene_container::GMSceneContainer;
use crate::scene::GMSceneT;

pub struct GMApp {
    scenes: GMSceneContainer,
    configuration: GMConfiguration,
}

impl GMApp {
    pub fn new<S: 'static + GMSceneT>(name: &str, first_scene: S) -> Self {
        let mut scenes = GMSceneContainer::new();
        scenes.add_scene(name, first_scene);

        Self {
            scenes,
            configuration: GMConfiguration::new(),
        }
    }

    pub fn load_configuration(&mut self, _file: &str) -> Result<(), GMError> {
        todo!();
    }

    pub fn run(&mut self) -> Result<(), GMError> {
        let mut current_scene = self.scenes.first_scene();

        let mut context = GMContext::new();
        context.set_configuration(&self.configuration);

        loop {
            let start_time = Instant::now();
            let scene_state = context.get_scene_state();

            match scene_state {
                GMSceneState::Enter => {
                    current_scene.enter(&mut context)?;
                }
                GMSceneState::Run => {
                    current_scene.update_before(&mut context)?;
                    context.update()?;
                    current_scene.update_after(&mut context)?;
                    current_scene.draw_before(&mut context)?;
                    context.draw()?;
                    current_scene.draw_after(&mut context)?;
                }
                GMSceneState::Leave => {
                    current_scene.leave(&mut context)?;
                }
                GMSceneState::ChangeToScene(scene_name) => {
                    current_scene = self.scenes.get_scene(scene_name)?;
                }
                GMSceneState::Quit => {
                    break
                }
            }

            let elapsed = start_time.elapsed().as_secs_f32();
            let diff = context.frame_time - elapsed;

            if diff > 0.0 {
                sleep(Duration::from_secs_f32(diff));
            }

        }

        Ok(())
    }
}
