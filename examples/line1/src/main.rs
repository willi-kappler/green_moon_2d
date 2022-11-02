

use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText, GMBitmapTextBuilder};
use green_moon_2d::util::{GMAlign};
use green_moon_2d::sprite::{GMSpriteBuilder};
use green_moon_2d::line::GMLine;

#[derive(Debug)]
struct LineScene1 {
    title: GMBitmapText,
    line1: GMLine,
    line2: GMLine,
}

impl LineScene1 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();
        let window_height = engine.window_height();
        const X_OFFSET: f32 = 350.0;

        // Move title to the center of the window
        let title = GMBitmapTextBuilder::new(resources.get_font("font_cuddly"))
            .with_text("LINE TEST 1")
            .with_position((window_width / 2.0, 32.0))
            .with_align(GMAlign::TopCenter)
            .build();

        let line1 = GMLine::new();
        let line2 = GMLine::new2();

        Self {
            title,
            line1,
            line2,
        }
    }
}

impl GMSceneT for LineScene1 {
    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        self.line1.update(context);
        self.line2.update(context);
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.title.draw(context);

        self.line1.draw(context);
        self.line2.draw(context);
    }
}

fn main() {
    let log_config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("line1.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let line1_scene = LineScene1::new(&engine);

    engine.add_scene("line1_scene", line1_scene);
    engine.run();
}
