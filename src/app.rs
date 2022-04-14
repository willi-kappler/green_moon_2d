
use std::fs::File;
use std::io::Read;

use log::debug;
use sdl2::gfx::framerate::FPSManager;

use crate::configuration::GMConfiguration;
use crate::context::{GMContext, GMSceneState};
use crate::draw_object::GMDrawContainer;
use crate::error::GMError;
use crate::scene::{GMSceneT, GMSceneContainer};

pub struct GMApp {
    scenes: GMSceneContainer,
    configuration: GMConfiguration,
}

impl GMApp {
    pub fn new<S: 'static + GMSceneT>(name: &str, first_scene: S) -> Self {
        debug!("GMApp::new(), initial scene name: '{}'", name);

        let mut scenes = GMSceneContainer::new();
        scenes.add_scene(name, first_scene).unwrap();

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
        let mut fps_manager = FPSManager::new();
        fps_manager.set_framerate(self.configuration.fps).unwrap();

        let mut draw_objects = GMDrawContainer::new();

        loop {
            let scene_state = context.get_scene_state();

            match scene_state {
                GMSceneState::Run => {
                    current_scene.update_before(&mut context, &mut draw_objects)?;
                    context.update()?;
                    draw_objects.update(&mut context)?;
                    current_scene.update_after(&mut context, &mut draw_objects)?;

                    current_scene.draw_before(&mut context, &mut draw_objects)?;
                    draw_objects.draw(&mut context)?;
                    current_scene.draw_after(&mut context, &mut draw_objects)?;

                    context.present();
                }
                GMSceneState::ChangeToScene(scene_name) => {
                    let scene_name = scene_name.clone();
                    current_scene.exit(&mut context, &mut draw_objects)?;
                    current_scene = self.scenes.get_scene_mut(&scene_name)?;
                    current_scene.init(&mut context, &mut draw_objects)?;
                    context.run_scene();
                }
                GMSceneState::Quit => {
                    break
                }
            }

            if context.new_fps > 0 {
                fps_manager.set_framerate(context.new_fps).unwrap();
                context.new_fps = 0;
            }

            fps_manager.delay();
        }

        Ok(())
    }
}
