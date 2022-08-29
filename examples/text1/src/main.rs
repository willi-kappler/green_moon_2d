

use std::fs::File;
use std::rc::Rc;

use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMBitmapText, GMBitmapFont, GMEventCode, GMResources, GMAlign};

#[derive(Clone, Debug)]
struct TextScene1 {
    texts: Vec<GMBitmapText>,
    fonts: Vec<Rc<GMBitmapFont>>,
    current_font: usize,
    char_spacing: f32,
}

impl TextScene1 {
    pub fn new(resources: &GMResources, window_width: f32) -> Self {
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

    fn change_font(&mut self) {
        debug!("TextScene1::update(), current font: {}", self.current_font);

        for text in self.texts.iter_mut() {
            text.set_font(&self.fonts[self.current_font]);
        }
    }

    fn change_spacing(&mut self) {
        debug!("TextScene1::update(), char_spacing: {}", self.char_spacing);

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
            if self.current_font != 0 {
                self.current_font = 0;
                self.change_font();
            }
        }

        if context.event(GMEventCode::Key2Up) {
            if self.current_font != 1 {
                self.current_font = 1;
                self.change_font();
            }
        }

        if context.event(GMEventCode::Key3Up) {
            if self.current_font != 2 {
                self.current_font = 2;
                self.change_font();
            }
        }

        if context.event(GMEventCode::KeyLeftUp) {
            self.char_spacing -= 1.0;
            self.change_spacing();
        }

        if context.event(GMEventCode::KeyRightUp) {
            self.char_spacing += 1.0;
            self.change_spacing();
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

    let text1_scene = TextScene1::new(engine.get_resources(), engine.window_width());

    engine.add_scene("text1_scene", text1_scene);
    engine.run();
}
