

use std::collections::VecDeque;

use sdl2::gfx::framerate::FPSManager;
use log::debug;

use crate::scene::{GMSceneT, GMSceneManager};
use crate::context::GMContext;
use crate::configuration::GMConfiguration;
use crate::error::GMError;

pub enum GMEngineMessage<'a> {
    AddScene(Box<dyn GMSceneT>),
    RemoveScene(&'a str),
    ChangeScene(&'a str),
    ChangeFPS(u32),
}

pub struct GMEngine {
    configuration: GMConfiguration,
    scenes: VecDeque<Box<dyn GMSceneT>>,
}

impl GMEngine {
    pub fn new() -> Self {

        Self {
            configuration: GMConfiguration::new(),
            scenes: VecDeque::new(),
        }
    }

    pub fn set_configuration(&mut self, configuration: GMConfiguration) {
        debug!("GMEngine::set_configuration()");

        self.configuration = configuration;
    }

    pub fn load_configuration(&mut self, file_name: &str) -> Result<(), GMError> {
        debug!("GMEngine::load_configuration(), file_name: '{}'", file_name);

        todo!();

        // Ok(())
    }

    pub fn add_scene<S: 'static + GMSceneT>(&mut self, scene: S) {
        debug!("GMEngine::add_scene(), name: '{}'", scene.get_name());

        self.scenes.push_back(Box::new(scene));
    }

    pub fn run(&mut self) -> Result<(), GMError> {
        debug!("GMEngine::run()");

        let mut scene_manager = GMSceneManager::new();
        let mut context = GMContext::new(&self.configuration);

        while let Some(scene) = self.scenes.pop_front() {
            scene_manager.add_scene(scene)?;
        }

        let mut fps_manager = FPSManager::new();
        fps_manager.set_framerate(self.configuration.fps).unwrap();

        while !context.quit_game {
            // Update everything
            scene_manager.update(&mut context)?;


            // Draw everything
            scene_manager.draw(&mut context)?;
            context.present();

            while let Some(message) = context.engine_messages.pop_front() {
                match message {
                    GMEngineMessage::AddScene(scene) => {
                        scene_manager.add_scene(scene)?;
                    }
                    GMEngineMessage::RemoveScene(name) => {
                        scene_manager.remove_scene(name)?;
                    }
                    GMEngineMessage::ChangeScene(name) => {
                        scene_manager.change_scene(name)?;
                    }
                    GMEngineMessage::ChangeFPS(new_fps) => {
                        fps_manager.set_framerate(new_fps).unwrap();
                        self.configuration.fps = new_fps;
                    }
                }
            }

            fps_manager.delay();
        }

        Ok(())
    }
}
