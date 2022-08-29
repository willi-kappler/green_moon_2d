

use std::fs::File;
use std::rc::Rc;

use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMBitmapText, GMBitmapFont, GMEventCode, GMResources, GMAlign};

#[derive(Debug)]
struct TextScene1 {
    texts: Vec<GMBitmapText>,
    fonts: Vec<Rc<GMBitmapFont>>,
    current_font: usize,
    char_spacing: f32,
}

impl TextScene1 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();

        const space: f32 = 50.0;
        let mut fonts = Vec::new();

        fonts.push(resources.get_font_clone("font_bbc"));
        fonts.push(resources.get_font_clone("font_blagger"));
        fonts.push(resources.get_font_clone("font_cuddly"));

        let current_font = 2;
        let font = &fonts[current_font];

        let mut texts = Vec::new();

        texts.push(GMBitmapText::new(font, "TEXT TEST 1", window_width / 2.0, 32.0));
        texts.push(GMBitmapText::new(font, "PRESS NUMBER TO CHANGE FONT", 32.0, 32.0 + (1.0 * space)));
        texts.push(GMBitmapText::new(font, "1 - BBC", 32.0, 32.0 + (2.0 * space)));
        texts.push(GMBitmapText::new(font, "2 - BLAGGER", 32.0, 32.0 + (3.0 * space)));
        texts.push(GMBitmapText::new(font, "3 - CUDDLY", 32.0, 32.0 + (4.0 * space)));
        texts.push(GMBitmapText::new(font, "CURSOR TO CHANGE H-SPACING", 32.0, 32.0 + (5.0 * space)));

        // Move title to the center of the window
        texts[0].align(GMAlign::TopCenter);

        Self {
            texts,
            fonts,
            current_font,
            char_spacing: 0.0,
        }
    }

    fn change_font(&mut self, new_font: usize) {
        if self.current_font != new_font {
            self.current_font = new_font;

            debug!("TextScene1::change_font(), current font: {}", self.current_font);

            for text in self.texts.iter_mut() {
                text.set_font(&self.fonts[self.current_font]);
            }
        }
    }

    fn change_spacing(&mut self, spacing: f32) {
        self.char_spacing += spacing;

        debug!("TextScene1::change_spacing(), char_spacing: {}", self.char_spacing);

        for text in self.texts.iter_mut() {
            text.set_spacing_x(self.char_spacing);
        }
    }
}

impl GMSceneT for TextScene1 {
    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        if context.event(GMEventCode::Key1Up) {
            self.change_font(0);
        }

        if context.event(GMEventCode::Key2Up) {
            self.change_font(1);
        }

        if context.event(GMEventCode::Key3Up) {
            self.change_font(2);
        }

        if context.event(GMEventCode::KeyLeftUp) {
            self.change_spacing(-1.0);
        }

        if context.event(GMEventCode::KeyRightUp) {
            self.change_spacing(1.0);
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, config, File::create("text1.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration("config.json");
    engine.init();
    engine.load_resources("resources.json");

    let text1_scene = TextScene1::new(&engine);

    engine.add_scene("text1_scene", text1_scene);
    engine.run();
}
