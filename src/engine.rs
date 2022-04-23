


use sdl2::gfx::framerate::FPSManager;
use log::debug;

use crate::scene::{GMSceneT, GMSceneManager};
use crate::context::create_context;
use crate::configuration::GMConfiguration;
use crate::error::GMError;

pub(crate) enum GMEngineMessage {
    Quit,
    ChangeFPS(u32),
}

pub struct GMEngine {
    configuration: GMConfiguration,
    scene_manager: GMSceneManager,
}

impl GMEngine {
    pub fn new() -> Self {

        Self {
            configuration: GMConfiguration::new(),
            scene_manager: GMSceneManager::new(),
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

    pub fn add_scene<S: 'static + GMSceneT>(&mut self, scene: S) -> Result<(), GMError> {
        debug!("GMEngine::add_scene(), name: '{}'", scene.get_name());

        self.scene_manager.add_scene(Box::new(scene))
    }

    pub fn run(&mut self) -> Result<(), GMError> {
        debug!("GMEngine::run()");

        let (mut update_context, mut draw_context) = create_context(&self.configuration);

        let mut fps_manager = FPSManager::new();
        fps_manager.set_framerate(self.configuration.fps).unwrap();

        'quit: loop {
            // Update everything
            update_context.update()?;
            self.scene_manager.update(&mut update_context)?;


            // Draw everything
            self.scene_manager.draw(&mut draw_context)?;
            draw_context.present();

            while let Some(message) = update_context.next_engine_message() {
                match message {
                    GMEngineMessage::Quit => {
                        break 'quit;
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
