use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::{GMAlign, GMRepetition};
use green_moon_2d::object_manager::GMObjectManager;
use green_moon_2d::sprite::GMSprite;
use green_moon_2d::object_util::{GMValueInterpolateF32};
use green_moon_2d::message::{msgt1v};
use green_moon_2d::line::{GMLine, GMLineMode};


#[derive(Debug)]
struct LineScene1 {
    object_manager: GMObjectManager,
}

impl LineScene1 {
    fn new(engine: &GMEngine) -> Self {
        // Access to resources:
        let resources = engine.get_resources();

        // Set some positions:
        let window_width = engine.window_width();
        let window_height = engine.window_height();
        let y_center = window_height / 2.0;
        const X_OFFSET: f32 = 500.0;

        // Crate object manager:
        let mut object_manager = GMObjectManager::new();

        // Get first font:
        let font1 = resources.get_font("font_cuddly");

        // Set up title text:
        let mut text = GMBitmapText::new(font1, (512.0, 32.0), "LINE 1");
        text.set_align(GMAlign::TopCenter);
        object_manager.add_draw_object("title", text, 0, 1);

        // Set up objects:
        let texture = resources.get_texture("tex_particle1");
        let animation = resources.get_animation("anim_particle1");
        let sprite = GMSprite::new((0.0, 0.0), texture, animation);

        let line1 = GMLine::new((32.0, y_center), (X_OFFSET, 0.0), sprite.clone(), GMLineMode::Number(20));
        object_manager.add_draw_object("line1", line1, 0, 0);

        let line2 = GMLine::new((window_width - 32.0, y_center), (window_width - X_OFFSET, 0.0), sprite, GMLineMode::Spacing(32.0));
        object_manager.add_draw_object("line2", line2, 0, 0);

        let mut interpolate = GMValueInterpolateF32::new(0.0, window_height, 0.005,
            |value, object_manager| {
                object_manager.send_message_object("line1", msgt1v("end2", "set_y", value));
                object_manager.send_message_object("line2", msgt1v("end2", "set_y", value));
            }
        );
        interpolate.interpolation.repetition = GMRepetition::PingPongForward;
        object_manager.add_normal_object("y_movement", interpolate, 0);

        Self {
            object_manager,
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

        self.object_manager.update(context);
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.object_manager.draw(context);
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

