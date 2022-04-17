


use sdl2::gfx::framerate::FPSManager;


use crate::scene::GMSceneT;
use crate::draw_object::GMDrawT;
use crate::context::GMContext;
use crate::configuration::GMConfiguration;
use crate::error::GMError;

pub struct GMEngine {
    scenes: Vec<Box<dyn GMSceneT>>,
    draw_objects: Vec<Box<dyn GMDrawT>>,
    context: GMContext,
}

impl GMEngine {
    pub fn new() -> Self {
        let configuration = GMConfiguration::new();

        Self {
            scenes: Vec::new(),
            draw_objects: Vec::new(),
            context: GMContext::new(configuration),
        }
    }

    pub fn set_configuration(&mut self, _configuration: GMConfiguration) {
        // self.context.set_configuration(configuration);
    }

    pub fn load_configuration(&mut self, _file_name: &str) -> Result<(), GMError> {
        todo!();

        // Ok(())
    }

    pub fn add_scene<S: 'static + GMSceneT>(&mut self, _scene: S) -> Result<(), GMError> {
        todo!();

        // Ok(())
    }

    pub fn add_scenes<S: 'static + GMSceneT> (&mut self, _scenes: &[S]) -> Result<(), GMError> {
        todo!();

        // Ok(())
    }

    pub fn remove_scene(&mut self, _name: &str) -> Result<(), GMError> {
        todo!();

        // Ok(())
    }

    pub fn has_scene(&self, _name: &str) -> bool {
        false
    }

    pub fn get_scene(&mut self, _name: &str) -> Result<&mut Box<dyn GMSceneT>, GMError> {
        todo!();
    }

    pub fn run(&mut self) -> Result<(), GMError> {
        let current_scene = &mut self.scenes[0];

        let mut fps_manager = FPSManager::new();
        fps_manager.set_framerate(self.context.configuration.fps).unwrap();

        while !self.context.quit_game {
            // Update everything
            current_scene.update_before(&mut self.context)?;
            self.context.update()?;
            for object in self.draw_objects.iter_mut() {
                object.update(&mut self.context)?;
            }
            current_scene.update_after(&mut self.context)?;


            // Draw everything
            current_scene.draw_before(&mut self.context)?;
            // Sort all drawable objects by z order before drawing them
            self.draw_objects.sort_by_key(|object| object.get_z_index());

            for object in self.draw_objects.iter() {
                object.draw(&mut self.context);
            }
            current_scene.draw_after(&mut self.context)?;
            self.context.present();


            if self.context.new_fps > 0 {
                fps_manager.set_framerate(self.context.new_fps).unwrap();
                self.context.new_fps = 0;
            }

            fps_manager.delay();
        }

        Ok(())
    }
}
