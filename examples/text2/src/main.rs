
use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::{GMDrawT, GMAlign};

#[derive(Debug)]
struct SpriteScene1 {
    title: GMBitmapText,

    descriptions: Vec<GMBitmapText>,
}

impl SpriteScene1 {
    fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();

        // Get font
        let font1 = resources.get_font("font_cuddly");

        // Set up title text:
        let mut title = GMBitmapText::new(font1, (512.0, 100.0), "TEXT 2");
        title.set_align2(GMAlign::BottomCenter);

        // Set up description:
        let mut descriptions = Vec::new();
        let mut text = GMBitmapText::new(font1, (512.0, 200.0), "PRESS NUMBERS");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        let mut text = GMBitmapText::new(font1, (512.0, 250.0), "TO CHANGE ALIGNMENT");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        let mut text = GMBitmapText::new(font1, (512.0, 350.0), "PRESS H TO TOGGLE");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        let mut text = GMBitmapText::new(font1, (512.0, 400.0), "HORIZONTAL");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        Self {
            title,
            descriptions,
        }
    }
}

impl GMSceneT for SpriteScene1 {
    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        if context.event(GMEventCode::Key1Up) {
            for text in self.descriptions.iter_mut() {
                text.set_align2(GMAlign::TopLeft);
            }
        }

        if context.event(GMEventCode::Key2Up) {
            for text in self.descriptions.iter_mut() {
                text.set_align2(GMAlign::TopCenter);
            }
        }

        if context.event(GMEventCode::Key3Up) {
            for text in self.descriptions.iter_mut() {
                text.set_align2(GMAlign::TopRight);
            }
        }

        if context.event(GMEventCode::Key4Up) {
            for text in self.descriptions.iter_mut() {
                text.set_align2(GMAlign::MiddleLeft);
            }
        }

        if context.event(GMEventCode::Key5Up) {
            for text in self.descriptions.iter_mut() {
                text.set_align2(GMAlign::MiddleCenter);
            }
        }

        if context.event(GMEventCode::Key6Up) {
            for text in self.descriptions.iter_mut() {
                text.set_align2(GMAlign::MiddleRight);
            }
        }

        if context.event(GMEventCode::Key7Up) {
            for text in self.descriptions.iter_mut() {
                text.set_align2(GMAlign::BottomLeft);
            }
        }

        if context.event(GMEventCode::Key8Up) {
            for text in self.descriptions.iter_mut() {
                text.set_align2(GMAlign::BottomCenter);
            }
        }

        if context.event(GMEventCode::Key9Up) {
            for text in self.descriptions.iter_mut() {
                text.set_align2(GMAlign::BottomRight);
            }
        }

        if context.event(GMEventCode::KeyHUp) {
            let text = &mut self.descriptions[3];
            let horizontal = text.get_horizontal();
            if horizontal {
                text.set_horizontal(false);
                text.set_text2("VERTICAL");
            } else {
                text.set_horizontal(true);
                text.set_text2("HORIZONTAL");
            }
        }
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.title.draw(context);

        for text in self.descriptions.iter() {
            text.draw(context);
        }
    }
}

fn main() {
    let log_config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("text2.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let text2_scene = SpriteScene1::new(&engine);

    engine.add_scene("text2_scene", text2_scene);
    engine.run();
}
