

use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMBitmapText};

#[derive(Clone, Debug)]
struct TextScene1 {
    name: String,
    text: GMBitmapText,
}

impl TextScene1 {
    pub fn new(engine: &GMEngine) -> Self {
        let font = engine.get_resources().get_font_clone("font_bbc");

        Self {
            name: "text_scene1".to_string(),
            text: GMBitmapText::new(font, "TEXT TEST 1", 32.0, 32.0),
        }
    }
}

impl GMSceneT for TextScene1 {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update(&mut self, context: &mut GMContext) {
        if context.input.key_esc_down() {
            context.quit();
        }
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.text.draw(context);
    }
}

fn main() {
    let config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, config, File::create("text1.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration("config.json");
    engine.init();
    engine.load_resources("resources.json");

    let text1_scene = TextScene1::new(&engine);

    engine.add_scene(text1_scene);
    engine.run();
}
