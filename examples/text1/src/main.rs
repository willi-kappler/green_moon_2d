

use std::fs::File;
use std::rc::Rc;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMBitmapText, GMBitmapFont};

#[derive(Clone, Debug)]
struct TextScene1 {
    name: String,
    title: GMBitmapText,
    description: GMBitmapText,
    font1: GMBitmapText,
    font2: GMBitmapText,
    font3: GMBitmapText,
}

impl TextScene1 {
    pub fn new(font: &Rc<GMBitmapFont>) -> Self {
        const space: f32 = 50.0;

        Self {
            name: "text_scene1".to_string(),
            title: GMBitmapText::new(font, "TEXT TEST 1", 336.0, 32.0),
            description: GMBitmapText::new(font, "PRESS NUMBER TO CHANGE FONT", 32.0, 32.0 + (1.0 * space)),
            font1: GMBitmapText::new(font, "1 - BBC", 32.0, 32.0 + (2.0 * space)),
            font2: GMBitmapText::new(font, "2 - BLAGGER", 32.0, 32.0 + (3.0 * space)),
            font3: GMBitmapText::new(font, "3 - CUDDLY", 32.0, 32.0 + (4.0 * space)),
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

            self.title.set_font(&font);
            self.description.set_font(&font);
            self.font1.set_font(&font);
            self.font2.set_font(&font);
            self.font3.set_font(&font);
        }

        if context.input.key_2_up() {
            let font = context.resources.get_font_clone("font_blagger");

            self.title.set_font(&font);
            self.description.set_font(&font);
            self.font1.set_font(&font);
            self.font2.set_font(&font);
            self.font3.set_font(&font);
        }

        if context.input.key_3_up() {
            let font = context.resources.get_font_clone("font_cuddly");

            self.title.set_font(&font);
            self.description.set_font(&font);
            self.font1.set_font(&font);
            self.font2.set_font(&font);
            self.font3.set_font(&font);
        }
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.title.draw(context);
        self.description.draw(context);
        self.font1.draw(context);
        self.font2.draw(context);
        self.font3.draw(context);
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
