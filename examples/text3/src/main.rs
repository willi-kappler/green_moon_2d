

use std::fs::File;

use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMBitmapText, GMBitmapFont, GMEventCode, GMResources, GMAlign, GMTextEffectT};

#[derive(Debug)]
struct TextScene3 {
    texts: Vec<GMBitmapText>,
    effects: Vec<Box<dyn GMTextEffectT>>,
}

impl TextScene3 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();

        let font = resources.get_font_clone("font_cuddly");

        const space: f32 = 50.0;
        let mut texts = Vec::new();

        texts.push(GMBitmapText::new(&font, "TEXT TEST 3", window_width / 2.0, 32.0 + (1.0 * space)));
        texts.push(GMBitmapText::new(&font, "PRESS NUMBER TO CHANGE EFFECT", 32.0, 32.0 + (5.0 * space)));
        texts.push(GMBitmapText::new(&font, "CURSOR TO CHANGE SETTING", 32.0, 32.0 + (6.0 * space)));

        // Move title to the center of the window
        texts[0].align(GMAlign::TopCenter);

        let mut effects = Vec::new();


        Self {
            texts,
            effects,
        }
    }
}

impl GMSceneT for TextScene3 {
    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        if context.event(GMEventCode::Key1Up) {
        }

        if context.event(GMEventCode::Key2Up) {
        }

        if context.event(GMEventCode::Key3Up) {
        }

        if context.event(GMEventCode::Key4Up) {
        }

        if context.event(GMEventCode::Key5Up) {
        }

        if context.event(GMEventCode::Key6Up) {
        }

        if context.event(GMEventCode::Key7Up) {
        }

        if context.event(GMEventCode::Key8Up) {
        }

        if context.event(GMEventCode::Key9Up) {
        }
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        for text in self.texts.iter() {
            text.draw(context);
        }
    }
}

fn main() {
    let config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, config, File::create("text3.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration("config.json");
    engine.init();
    engine.load_resources("resources.json");

    let text3_scene = TextScene3::new(&engine);

    engine.add_scene("text3_scene", text3_scene);
    engine.run();
}
