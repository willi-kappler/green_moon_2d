

use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText, GMBitmapTextBuilder};
use green_moon_2d::util::{GMAlign};
use green_moon_2d::sprite::{GMSpriteBuilder};
use green_moon_2d::line::GMLineBase;

#[derive(Debug)]
struct LineScene1 {
    title: GMBitmapText,
    line1: GMLineBase,
    line2: GMLineBase,
    y_pos: f32,
}

impl LineScene1 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();
        let window_height = engine.window_height();
        let y_center = window_height / 2.0;
        const X_OFFSET: f32 = 500.0;

        // Move title to the center of the window
        let title = GMBitmapTextBuilder::new(resources.get_font("font_cuddly"))
            .with_text("LINE TEST 1")
            .with_position((window_width / 2.0, 32.0))
            .with_align(GMAlign::TopCenter)
            .build();

        let line_sprite1 = GMSpriteBuilder::new(resources.get_texture("tex_particle1"))
            .with_position((0.0, 0.0))
            .with_animation(resources.get_animation("anim_particle1"))
            .build();

        // Number of sprites fixed:
        let line1 = GMLineBase::new((32.0, y_center), (X_OFFSET, 0.0), line_sprite1.clone(), 20);

        // Spacing fixed:
        let line2 = GMLineBase::new2((window_width - 32.0, y_center), (window_width - X_OFFSET, 0.0), line_sprite1, 32.0);

        Self {
            title,
            line1,
            line2,
            y_pos: 0.0,
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

        // self.line1.update(context);
        // self.line2.update(context);

        self.line1.update(context);
        self.line2.update(context);

        self.y_pos += 4.0;

        if self.y_pos >= 768.0 {
            self.y_pos = 0.0;
        }

        let x1 = self.line1.end.x;
        self.line1.set_end((x1, self.y_pos));

        let x2 = self.line2.end.x;
        self.line2.set_end((x2, self.y_pos));
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
