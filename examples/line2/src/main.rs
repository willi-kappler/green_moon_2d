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
use green_moon_2d::movement::{GMMVCircleFunc};


#[derive(Debug)]
struct LineScene2 {
    object_manager: GMObjectManager,
}

impl LineScene2 {
    fn new(engine: &GMEngine) -> Self {
        // Access to resources:
        let resources = engine.get_resources();

        // Crate object manager:
        let mut object_manager = GMObjectManager::new();

        // Get first font:
        let font1 = resources.get_font("font_cuddly");

        // Set up title text:
        let mut text = GMBitmapText::new(font1, (512.0, 32.0), "LINE 2");
        text.set_align(GMAlign::TopCenter);
        object_manager.add_draw_object("title", text, 0, 1);

        // Set up objects:
        let texture = resources.get_texture("tex_particle1");
        let animation = resources.get_animation("anim_particle1");
        let sprite = GMSprite::new((0.0, 0.0), texture, animation);

        let line = GMLine::new((0.0, 0.0), (0.0, 0.0), sprite, GMLineMode::Number(30));
        object_manager.add_draw_object("line", line, 0, 0);

        // Circle 1
        let circle1 = GMMVCircleFunc::new((80.0, 200.0), 60.0,
            |position, object_manager| {
                object_manager.send_message_object("line", msgt1v("start", "set", position));
            }
        );
        object_manager.add_normal_object("circle1", circle1, 0);

        let mut interpolate1 = GMValueInterpolateF32::new(0.0, 360.0, 0.02,
            |value, object_manager| {
                object_manager.send_message_object("circle1", msgt1v("angle", "set", value));
            }
        );
        interpolate1.interpolation.repetition = GMRepetition::LoopForward;
        object_manager.add_normal_object("interpolate1", interpolate1, 0);

        // Circle 2
        let circle2 = GMMVCircleFunc::new((900.0, 600.0), 100.0,
            |position, object_manager| {
                object_manager.send_message_object("line", msgt1v("end2", "set", position));
            }
        );
        object_manager.add_normal_object("circle2", circle2, 0);

        let mut interpolate2 = GMValueInterpolateF32::new(0.0, 360.0, 0.01,
            |value, object_manager| {
                object_manager.send_message_object("circle2", msgt1v("angle", "set", value));
            }
        );
        interpolate2.interpolation.repetition = GMRepetition::LoopBackward;
        object_manager.add_normal_object("interpolate2", interpolate2, 0);

        // Move center of circle 2
        let mut interpolate3 = GMValueInterpolateF32::new(0.0, 768.0, 0.002,
            |value, object_manager| {
                object_manager.send_message_object("circle2", msgt1v(("circle", "position"), "set_y", value));
            }
        );
        interpolate3.interpolation.repetition = GMRepetition::PingPongForward;
        object_manager.add_normal_object("interpolate3", interpolate3, 0);


        Self {
            object_manager,
        }
    }
}

impl GMSceneT for LineScene2 {
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("line2.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let line2_scene = LineScene2::new(&engine);

    engine.add_scene("line2_scene", line2_scene);
    engine.run();
}

