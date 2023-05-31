use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::{GMAlign, GMRepetition};
use green_moon_2d::object_manager::GMObjectManager;
use green_moon_2d::sprite::GMSprite;
use green_moon_2d::object_util::{GMValueInterpolateF32, GMValueInterpolateVec2D};
use green_moon_2d::message::{msgt1v};
use green_moon_2d::movement::GMMVCircle;


#[derive(Debug)]
struct SpriteScene1 {
    object_manager: GMObjectManager,
}

impl SpriteScene1 {
    fn new(engine: &GMEngine) -> Self {
        // Access to resources
        let resources = engine.get_resources();

        // Crate object manager
        let mut object_manager = GMObjectManager::new();

        // Get first font
        let font1 = resources.get_font("font_cuddly");

        // Set up title text:
        let mut text = GMBitmapText::new(font1, (512.0, 100.0), "SPRITE 1");
        text.set_align(GMAlign::BottomCenter);
        object_manager.add_draw_object("title", text, 0, 0);

        // Set up sprites:

        // Bat sprite
        let texture = resources.get_texture("tex_bat1");
        let animation = resources.get_animation("anim_bat1");
        let mut sprite = GMSprite::new((512.0, 200.0), texture, animation);
        object_manager.add_draw_object("bat1", sprite, 0, 0);

        // Ghost sprite
        let texture = resources.get_texture("tex_ghost1");
        let animation = resources.get_animation("anim_ghost1");
        sprite = GMSprite::new((512.0, 250.0), texture, animation);
        sprite.flipxy.x = true;
        object_manager.add_draw_object("ghost1", sprite, 0, 0);


        // Ice1 sprite
        let texture = resources.get_texture("tex_ice_cream1");
        let animation = resources.get_empty_animation();
        sprite = GMSprite::new((100.0, 300.0), texture, animation);
        object_manager.add_draw_object("ice1", sprite, 0, 0);

        let mut interpolate = GMValueInterpolateVec2D::new((100.0, 300.0), (900.0, 300.0), 0.007,
            |value, object_manager| {
                object_manager.send_message(&"ice1".into(), msgt1v("position", "set", value));
            }
        );
        interpolate.interpolation.repetition = GMRepetition::PingPongForward;
        object_manager.add_normal_object("move_ice1", interpolate, 0);

        let mut interpolate = GMValueInterpolateF32::new(-30.0, 30.0, 0.05,
            |value, object_manager| {
                object_manager.send_message(&"ice1".into(), msgt1v("angle", "set", value));
            }
        );
        interpolate.interpolation.repetition = GMRepetition::PingPongForward;
        object_manager.add_normal_object("rotate_ice1", interpolate, 0);

        // Head sprite
        let texture = resources.get_texture("tex_head1");
        let animation = resources.get_animation("anim_head1");
        sprite = GMSprite::new((512.0, 400.0), texture, animation);
        object_manager.add_draw_object("head1", sprite, 0, 0);

        let circle = GMMVCircle::new("head1", (512.0, 400.0), 70.0);
        object_manager.add_normal_object("circle_head1", circle, 0);

        let mut interpolate = GMValueInterpolateF32::new(90.0-60.0, 90.0+60.0, 0.02,
            |value, object_manager| {
                object_manager.send_message(&"circle_head1".into(), msgt1v("angle", "set", value));
            }
        );
        interpolate.interpolation.repetition = GMRepetition::PingPongForward;
        object_manager.add_normal_object("angle_circle_head1", interpolate, 0);

        // Ice troll1 sprite
        let texture = resources.get_texture("tex_ice_troll1");
        let animation = resources.get_animation("anim_ice_troll1");
        sprite = GMSprite::new((512.0, 600.0), texture, animation);
        object_manager.add_draw_object("ice_troll1", sprite, 0, 0);

        let mut interpolate = GMValueInterpolateVec2D::new((100.0, 600.0), (900.0, 600.0), 0.002,
            |value, object_manager| {
                object_manager.send_message(&"ice_troll1".into(), msgt1v("position", "set", value));
            }
        );
        interpolate.interpolation.repetition = GMRepetition::LoopForward;
        object_manager.add_normal_object("move_ice_troll1", interpolate, 0);

        let mut interpolate = GMValueInterpolateF32::new(0.5, 4.0, 0.01,
            |value, object_manager| {
                object_manager.send_message(&"ice_troll1".into(), msgt1v("scale", "set", value));
            }
        );
        interpolate.interpolation.repetition = GMRepetition::PingPongForward;
        object_manager.add_normal_object("scale_ice_troll1", interpolate, 0);

        Self {
            object_manager,
        }
    }
}

impl GMSceneT for SpriteScene1 {
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("sprite1.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let sprite1_scene = SpriteScene1::new(&engine);

    engine.add_scene("sprite1_scene", sprite1_scene);
    engine.run();
}

