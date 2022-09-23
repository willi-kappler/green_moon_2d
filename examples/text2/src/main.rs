

use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText, GMBitmapTextBuilder};
use green_moon_2d::util::{GMAlign};

#[derive(Debug)]
struct TextScene2 {
    texts: Vec<GMBitmapText>,
    horizontal: bool,
}

impl TextScene2 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();
        let window_height = engine.window_height();

        let font = resources.get_font("font_cuddly");

        const space: f32 = 50.0;
        let mut texts = Vec::new();

        // Move title to the center of the window
        texts.push(GMBitmapTextBuilder::new(&font)
            .with_text("TEXT TEST 2")
            .with_position((window_width / 2.0, window_height / 2.0))
            .with_align(GMAlign::MiddleCenter)
            .build());


        texts.push(GMBitmapTextBuilder::new(&font)
            .with_text("PRESS NUMBER TO CHANGE ALIGN")
            .with_position((32.0, 32.0 + (1.0 * space)))
            .build());


        texts.push(GMBitmapTextBuilder::new(&font)
            .with_text("PRESS H TO TOGGLE HORIZONTAL")
            .with_position((32.0, 32.0 + (2.0 * space)))
            .build());

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

        let text = &mut self.texts[0].base;

        if context.event(GMEventCode::Key1Up) {
            text.set_align(GMAlign::TopLeft);
        }

        if context.event(GMEventCode::Key2Up) {
            text.set_align(GMAlign::TopCenter);
        }

        if context.event(GMEventCode::Key3Up) {
            text.set_align(GMAlign::TopRight);
        }

        if context.event(GMEventCode::Key4Up) {
            text.set_align(GMAlign::MiddleLeft);
        }

        if context.event(GMEventCode::Key5Up) {
            text.set_align(GMAlign::MiddleCenter);
        }

        if context.event(GMEventCode::Key6Up) {
            text.set_align(GMAlign::MiddleRight);
        }

        if context.event(GMEventCode::Key7Up) {
            text.set_align(GMAlign::BottomLeft);
        }

        if context.event(GMEventCode::Key8Up) {
            text.set_align(GMAlign::BottomCenter);
        }

        if context.event(GMEventCode::Key9Up) {
            text.set_align(GMAlign::BottomRight);
        }

        if context.event(GMEventCode::KeyHUp) {
            self.horizontal = !self.horizontal;
            text.set_horizontal(self.horizontal);
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
    engine.load_configuration_and_init("config.json");

    let text2_scene = TextScene2::new(&engine);

    engine.add_scene("text2_scene", text2_scene);
    engine.run();
}
