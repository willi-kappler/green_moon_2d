

use std::fs::File;
use std::rc::Rc;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMBitmapText, GMBitmapFont};

#[derive(Clone, Debug)]
struct TextScene1 {
    name: String,
    texts: Vec<GMBitmapText>,
}

impl TextScene1 {
    pub fn new(font: &Rc<GMBitmapFont>) -> Self {
        const space: f32 = 50.0;
        let mut texts = Vec::new();

        texts.push(GMBitmapText::new(font, "TEXT TEST 1", 336.0, 32.0));
        texts.push(GMBitmapText::new(font, "PRESS NUMBER TO CHANGE FONT", 32.0, 32.0 + (1.0 * space)));
        texts.push(GMBitmapText::new(font, "1 - BBC", 32.0, 32.0 + (2.0 * space)));
        texts.push(GMBitmapText::new(font, "2 - BLAGGER", 32.0, 32.0 + (3.0 * space)));
        texts.push(GMBitmapText::new(font, "3 - CUDDLY", 32.0, 32.0 + (4.0 * space)));

        Self {
            name: "text_scene1".to_string(),
            texts,
        }
    }
}

impl GMSceneT for TextScene1 {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update(&mut self, context: &mut GMContext) {
        if context.input.key_esc_up() {
            context.quit();
        }

        if context.input.key_1_up() {
            let font = context.resources.get_font_clone("font_bbc");

            for text in self.texts.iter_mut() {
                text.set_font(&font);
            }
        }

        if context.input.key_2_up() {
            let font = context.resources.get_font_clone("font_blagger");

            for text in self.texts.iter_mut() {
                text.set_font(&font);
            }
        }

        if context.input.key_3_up() {
            let font = context.resources.get_font_clone("font_cuddly");

            for text in self.texts.iter_mut() {
                text.set_font(&font);
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

    let font = engine.get_resources().get_font_clone("font_cuddly");
    let text1_scene = TextScene1::new(&font);

    engine.add_scene(text1_scene);
    engine.run();
}
