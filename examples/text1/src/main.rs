

use std::fs::File;
use std::rc::Rc;

use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMBitmapText, GMBitmapFont, GMEventCode, GMResources};

#[derive(Clone, Debug)]
struct TextScene1 {
    name: String,
    texts: Vec<GMBitmapText>,
    fonts: Vec<Rc<GMBitmapFont>>,
    current_font: usize,
}

impl TextScene1 {
    pub fn new(resources: &GMResources) -> Self {
        const space: f32 = 50.0;
        let mut fonts = Vec::new();

        fonts.push(resources.get_font_clone("font_bbc"));
        fonts.push(resources.get_font_clone("font_blagger"));
        fonts.push(resources.get_font_clone("font_cuddly"));

        let current_font = 2;
        let font = &fonts[current_font];

        let mut texts = Vec::new();

        texts.push(GMBitmapText::new(font, "TEXT TEST 1", 336.0, 32.0));
        texts.push(GMBitmapText::new(font, "PRESS NUMBER TO CHANGE FONT", 32.0, 32.0 + (1.0 * space)));
        texts.push(GMBitmapText::new(font, "1 - BBC", 32.0, 32.0 + (2.0 * space)));
        texts.push(GMBitmapText::new(font, "2 - BLAGGER", 32.0, 32.0 + (3.0 * space)));
        texts.push(GMBitmapText::new(font, "3 - CUDDLY", 32.0, 32.0 + (4.0 * space)));

        Self {
            name: "text_scene1".to_string(),
            texts,
            fonts,
            current_font,
        }
    }
}

impl GMSceneT for TextScene1 {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        if context.event(GMEventCode::Key1Up) {
            if self.current_font != 0 {
                self.current_font = 0;

                debug!("TextScene1::update(), current font: {}", self.current_font);

                for text in self.texts.iter_mut() {
                    text.set_font(&self.fonts[self.current_font]);
                }
            }
        }

        if context.event(GMEventCode::Key2Up) {
            if self.current_font != 1 {
                self.current_font = 1;

                debug!("TextScene1::update(), current font: {}", self.current_font);

                for text in self.texts.iter_mut() {
                    text.set_font(&self.fonts[self.current_font]);
                }
            }
        }

        if context.event(GMEventCode::Key3Up) {
            if self.current_font != 2 {
                self.current_font = 2;

                debug!("TextScene1::update(), current font: {}", self.current_font);

                for text in self.texts.iter_mut() {
                    text.set_font(&self.fonts[self.current_font]);
                }
            }
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

    let text1_scene = TextScene1::new(engine.get_resources());

    engine.add_scene(text1_scene);
    engine.run();
}
