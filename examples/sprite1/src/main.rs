
use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};


#[derive(Debug)]
struct SpriteScene1 {
}

impl GMSceneT for SpriteScene1 {
    fn init(&mut self, context: &mut GMContext) {
        let resources = context.resources_mut();

        // Set up text:
        let font = resources.get_font("font_cuddly");
        let text = "SPRITES 1";
        let position = (512.0, 100.0);

    }

    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();
    }
}

fn main() {
    let log_config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("sprite1.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let sprite1_scene = SpriteScene1 {};

    engine.add_scene("sprite1_scene", sprite1_scene);
    engine.run();
}
