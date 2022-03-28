
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::fs::File;
use std::io::Read;

use log::debug;

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
        debug!("GMApp::new(), initial scene name: '{}'", name);

        let mut scenes = GMSceneContainer::new();
        scenes.add_scene(name, first_scene);

        Self {
            scenes,
            configuration: GMConfiguration::new(),
        }
    }

    pub fn load_configuration(&mut self, file_name: &str) -> Result<(), GMError> {
        debug!("GMApp::load_configuration(), from file: '{}'", file_name);

        let mut file = File::open(file_name)?;
        let mut data = Vec::new();

        file.read_to_end(&mut data)?;

        self.configuration = serde_json::from_slice(&data)?;

        Ok(())
    }

    pub fn set_configuration(&mut self, configuration: GMConfiguration) {
        debug!("GMApp::set_configuration()");

        self.configuration = configuration;
    }

    pub fn run(&mut self) -> Result<(), GMError> {
        debug!("GMApp::run()");

        let mut current_scene = self.scenes.first_scene();

        let mut context = GMContext::new(self.configuration.clone());

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
                    context.present();
                }
                GMSceneState::Leave => {
                    current_scene.leave(&mut context)?;
                }
                GMSceneState::ChangeToScene(scene_name) => {
                    current_scene = self.scenes.get_scene(scene_name)?;
                    context.enter_scene();
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
