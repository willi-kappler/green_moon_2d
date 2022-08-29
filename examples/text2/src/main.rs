

use std::fs::File;
use std::rc::Rc;

use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMBitmapText, GMBitmapFont, GMEventCode, GMResources, GMAlign};

#[derive(Clone, Debug)]
struct TextScene2 {
    texts: Vec<GMBitmapText>,
    horizontal: bool,
}

impl TextScene2 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();
        let window_height = engine.window_height();

        let font = resources.get_font_clone("font_cuddly");

        const space: f32 = 50.0;
        let mut texts = Vec::new();

        texts.push(GMBitmapText::new(&font, "TEXT TEST 2", window_width / 2.0, window_height / 2.0));
        texts.push(GMBitmapText::new(&font, "PRESS NUMBER TO CHANGE ALIGN", 32.0, 32.0 + (1.0 * space)));
        texts.push(GMBitmapText::new(&font, "PRESS H TO TOGGLE HORIZONTAL", 32.0, 32.0 + (2.0 * space)));

        // Move title to the center of the window
        texts[0].align(GMAlign::MiddleCenter);

        Self {
            texts,
            horizontal: true,
        }
    }
}

impl GMSceneT for TextScene2 {
    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        if context.event(GMEventCode::Key1Up) {
            self.texts[0].align(GMAlign::TopLeft);
        }

        if context.event(GMEventCode::Key2Up) {
            self.texts[0].align(GMAlign::TopCenter);
        }

        if context.event(GMEventCode::Key3Up) {
            self.texts[0].align(GMAlign::TopRight);
        }

        if context.event(GMEventCode::Key4Up) {
            self.texts[0].align(GMAlign::MiddleLeft);
        }

        if context.event(GMEventCode::Key5Up) {
            self.texts[0].align(GMAlign::MiddleCenter);
        }

        if context.event(GMEventCode::Key6Up) {
            self.texts[0].align(GMAlign::MiddleRight);
        }

        if context.event(GMEventCode::Key7Up) {
            self.texts[0].align(GMAlign::BottomLeft);
        }

        if context.event(GMEventCode::Key8Up) {
            self.texts[0].align(GMAlign::BottomCenter);
        }

        if context.event(GMEventCode::Key9Up) {
            self.texts[0].align(GMAlign::BottomRight);
        }

        if context.event(GMEventCode::KeyHUp) {
            self.horizontal = !self.horizontal;
            self.texts[0].set_horizontal(self.horizontal);
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, config, File::create("text2.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration("config.json");
    engine.init();
    engine.load_resources("resources.json");

    let text2_scene = TextScene2::new(&engine);

    engine.add_scene("text2_scene", text2_scene);
    engine.run();
}
