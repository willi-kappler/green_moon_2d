

use std::fs;
use std::path::Path;

use sdl2::gfx::framerate::FPSManager;
use log::debug;
use nanoserde::DeJson;

use crate::resources::GMResources;
use crate::context::{GMContext};
use crate::scene::{GMSceneManager};
use crate::configuration::GMConfiguration;
use crate::scene::GMSceneT;
use crate::util::error_panic;

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
                error_panic(&format!("Error in JSON configuration string: '{}'", e));
            }
        }
    }

    pub fn load_configuration<P: AsRef<Path>>(&mut self, path: P) {
        debug!("GMEngine::load_configuration(), path: '{:?}'", path.as_ref());

        match fs::read_to_string(path.as_ref()) {
            Ok(json) => {
                self.configuration_from_json(&json);
            }
            Err(e) => {
                error_panic(&format!("Error in reading file: '{:?}', '{}'", path.as_ref(), e));
            }
        }
    }

    pub fn load_configuration_and_init<P: AsRef<Path>>(&mut self, path: P) {
        debug!("GMEngine::load_configuration_and_init(), path: '{:?}'", path.as_ref());

        self.load_configuration(path);
        self.init();
    }

    pub fn init(&mut self) {
        debug!("GMEngine::init()");

        let sdl_context = sdl2::init().expect("GMEngine::init(), could not initialize SDL2");
        let video_subsystem = sdl_context.video().expect("GMEngine::init(), could not initialize video");
        let window = video_subsystem.window(
            &self.configuration.window_title,
            self.configuration.screen_width,
            self.configuration.screen_height)
            .position_centered()
            .build()
            .expect("GMEngine::init(), could not initialize Window");
        let canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .build().expect("GMEngine::init(), could not initialize canvas");
        let texture_creator = canvas.texture_creator();
        let event_pump = sdl_context.event_pump().expect("GMEngine::init(), could not initialize events");

        self.context = Some(GMContext::new(texture_creator, event_pump, canvas, &self.configuration));

        if let Some(resources) = &self.configuration.resources {
            let path = resources.clone();
            self.load_resources(path);
        }
    }

    pub fn load_resources<P: AsRef<Path>>(&mut self, path: P) {
        debug!("GMEngine::load_resources(), file_name: '{:?}'", path.as_ref());

        let context = self.context.as_mut()
            .expect("GMEngine::load_resources(), context not set, call init() on engine first!");

        context.get_resources_mut().load_resources(path);
    }

    pub fn get_resources(&self) -> &GMResources {
        debug!("GMEngine::get_resources()");

        let context = self.context.as_ref()
            .expect("GMEngine::get_resources(), context not set, call init() on engine first!");

        &context.get_resources()
    }

    pub fn get_resources_mut(&mut self) -> &mut GMResources {
        debug!("GMEngine::get_resources()");

        let context = self.context.as_mut()
            .expect("GMEngine::get_resources(), context not set, call init() on engine first!");

        context.get_resources_mut()
    }

    pub fn add_scene<T: 'static + GMSceneT>(&mut self, name: &str, scene: T) {
        self.add_scene2(name, Box::new(scene));
    }

    pub fn add_scene2(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        debug!("GMEngine::add_scene2(), name: '{}'", name);

        self.scene_manager.add_scene2(name, scene)
    }

    pub fn window_width(&self) -> f32 {
        self.configuration.screen_width as f32
    }

    pub fn window_height(&self) -> f32 {
        self.configuration.screen_height as f32
    }

    pub fn run(&mut self) {
        debug!("GMEngine::run()");

        let context = self.context.as_mut()
            .expect("GMEngine::run(), context not set, call init() on engine first!");

        let mut fps_manager = FPSManager::new();
        fps_manager.set_framerate(self.configuration.fps).expect("GMEngine::run(), could not set frame rate");

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
                        fps_manager.set_framerate(new_fps).expect("GMEngine::run(), could not set frame rate");
                        self.configuration.fps = new_fps;
                    }
                }
            }

            fps_manager.delay();
        }
    }
}
