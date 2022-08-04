

use std::fs;

use sdl2::gfx::framerate::FPSManager;
use log::debug;
use nanoserde::DeJson;

use crate::resources::GMResources;
use crate::context::{GMContext};
use crate::scene::{GMSceneManager};
use crate::configuration::GMConfiguration;
use crate::scene::GMSceneT;

#[derive(Debug)]
pub(crate) enum GMEngineMessage {
    ChangeFPS(u32),
    Quit,
}

pub struct GMEngine {
    configuration: GMConfiguration,
    scene_manager: GMSceneManager,
    context: Option<GMContext>,
}

impl GMEngine {
    pub fn new() -> Self {

        Self {
            configuration: GMConfiguration::new(),
            scene_manager: GMSceneManager::new(),
            context: None,
        }
    }

    pub fn set_configuration(&mut self, configuration: GMConfiguration) {
        debug!("GMEngine::set_configuration()");

        self.configuration = configuration;
    }

    pub fn configuration_from_json(&mut self, json: &str) {
        debug!("GMEngine::configuration_from_json()");

        match DeJson::deserialize_json(json) {
            Ok(configuration) => {
                self.set_configuration(configuration);
            }
            Err(e) => {
                panic!("Error in JSON configuration string: '{}'", e);
            }
        }
    }

    pub fn load_configuration(&mut self, file_name: &str) {
        debug!("GMEngine::load_configuration(), file_name: '{}'", file_name);

        match fs::read_to_string(file_name) {
            Ok(json) => {
                self.configuration_from_json(&json);
            }
            Err(e) => {
                panic!("Error in reading file: '{}', '{}'", file_name, e);
            }
        }

    }

    pub fn init(&mut self) {
        debug!("GMEngine::init()");

        let sdl_context = sdl2::init().expect("Could not initialize SDL2");
        let video_subsystem = sdl_context.video().expect("Could not initialize video");
        let window = video_subsystem.window(
            &self.configuration.window_title,
            self.configuration.screen_width,
            self.configuration.screen_height)
            .position_centered()
            .build()
            .expect("Could not initialize Window");
        let canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .build().expect("Could not initialize canvas");
        let texture_creator = canvas.texture_creator();
        let event_pump = sdl_context.event_pump().expect("Could not initialize events");

        self.context = Some(GMContext::new(texture_creator, event_pump, canvas));
    }

    pub fn load_resources(&mut self, file_name: &str) {
        debug!("GMEngine::load_resources(), file_name: '{}'", file_name);

        let update_context = self.context.as_mut()
            .expect("Update context not set, call init() on engine first!");

        update_context.resources.load_resources(file_name);
    }

    pub fn get_resources(&mut self) -> &mut GMResources {
        debug!("GMEngine::get_resources()");

        let update_context = self.context.as_mut()
            .expect("Update context not set, call init() on engine first!");

        &mut update_context.resources
    }

    pub fn add_scene<S: 'static + GMSceneT>(&mut self, scene: S) {
        debug!("GMEngine::add_scene(), name: '{}'", scene.get_name());

        self.scene_manager.add_scene(Box::new(scene))
    }

    pub fn run(&mut self) {
        debug!("GMEngine::run()");

        let context = self.context.as_mut()
            .expect("Update context not set, call init() on engine first!");

        let mut fps_manager = FPSManager::new();
        fps_manager.set_framerate(self.configuration.fps).expect("Could not set frame rate");

        use GMEngineMessage::*;

        'quit: loop {
            // Update everything
            context.update();
            self.scene_manager.update(context);

            // Draw everything
            self.scene_manager.draw(context);

            // Present
            context.present();

            while let Some(message) = context.next_engine_message() {
                match message {
                    Quit => {
                        debug!("GMEngine message: Quit");
                        break 'quit;
                    }
                    ChangeFPS(new_fps) => {
                        debug!("GMEngine message: ChangeFPS: '{}'", new_fps);
                        fps_manager.set_framerate(new_fps).expect("Could not set frame rate");
                        self.configuration.fps = new_fps;
                    }
                }
            }

            fps_manager.delay();
        }
    }
}
