
use std::fs::File;
use std::sync::Arc;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText, GMBitmapFont};
use green_moon_2d::util::{GMDrawT, GMAlign};

// use green_moon_2d::animation::{GMAnimation};


#[derive(Debug)]
struct SpriteScene1 {
    font1: Arc<GMBitmapFont>,
    font2: Arc<GMBitmapFont>,
    font3: Arc<GMBitmapFont>,
    title: GMBitmapText,

    descriptions: Vec<GMBitmapText>,
}

impl SpriteScene1 {
    fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();

        // Get fonts
        let font1 = resources.get_font("font_cuddly");
        let font2 = resources.get_font("font_blagger");
        let font3 = resources.get_font("font_bbc");

        // Set up title text:
        let mut title = GMBitmapText::new(font1, (512.0, 100.0), "TEXT 1");
        title.set_align2(GMAlign::BottomCenter);

        // Set up description:
        let mut descriptions = Vec::new();
        let mut text = GMBitmapText::new(font1, (512.0, 200.0), "PRESS 1 2 3");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        let mut text = GMBitmapText::new(font1, (512.0, 300.0), "TO CHANGE FONT");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        let mut text = GMBitmapText::new(font1, (512.0, 350.0), "THIS IS FONT 1");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        let mut text = GMBitmapText::new(font1, (512.0, 400.0), "CUDDLY");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        Self {
            font1: font1.clone(),
            font2: font2.clone(),
            font3: font3.clone(),
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
            self.descriptions[2].set_text("THIS IS FONT 1");
            self.descriptions[3].set_text("CUDDLY");

            for text in self.descriptions.iter_mut() {
                text.set_font2(&self.font1);
            }
        }

        if context.event(GMEventCode::Key2Up) {
            self.descriptions[2].set_text("THIS IS FONT 2");
            self.descriptions[3].set_text("BLAGGER");

            for text in self.descriptions.iter_mut() {
                text.set_font2(&self.font2);
            }
        }

        if context.event(GMEventCode::Key3Up) {
            self.descriptions[2].set_text("THIS IS FONT 3");
            self.descriptions[3].set_text("BBC");

            for text in self.descriptions.iter_mut() {
                text.set_font2(&self.font3);
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("text1.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let text1_scene = SpriteScene1::new(&engine);

    engine.add_scene("text1_scene", text1_scene);
    engine.run();
}
