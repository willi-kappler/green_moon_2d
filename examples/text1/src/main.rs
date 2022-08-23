

use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMBitmapText};

#[derive(Clone, Debug)]
struct TextScene1 {
}

impl TextScene1 {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl GMSceneT for TextScene1 {
    fn get_name(&self) -> &str {
        "text_scene1"
    }

    fn update(&mut self, context: &mut GMContext) {
        if context.input.key_esc_down() {
            context.quit();
        }
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();



    }
}

fn main() {
    let config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, config, File::create("text1.log").expect("Could not create log file"));

    let text1_scene = TextScene1::new();

    let mut engine = GMEngine::new();
    engine.load_configuration("config.json");
    engine.init();
    engine.load_resources("resources.json");
    engine.add_scene(text1_scene);
    engine.run();
}
