use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::{GMAlign, GMRepetition};
use green_moon_2d::object_manager::GMObjectManager;
use green_moon_2d::sprite::GMSprite;
use green_moon_2d::object_util::{GMValueInterpolateF32, GMTimedMessage};
use green_moon_2d::message::GMMessage;
use green_moon_2d::movement::{GMMVCircle, GMMVMultiCircle, GMMVPath, GMMVFollow};
use green_moon_2d::target::GMTarget;


#[derive(Debug)]
struct SpriteScene2 {
    object_manager: GMObjectManager,
}

impl SpriteScene2 {
    fn new(engine: &GMEngine) -> Self {
        // Access to resources:
        let resources = engine.get_resources();

        // Crate object manager:
        let mut object_manager = GMObjectManager::new();

        // Get first font:
        let font1 = resources.get_font("font_cuddly");

        // Set up title text:
        let mut text = GMBitmapText::new(font1, (512.0, 100.0), "SPRITE 2");
        text.set_align(GMAlign::BottomCenter);
        object_manager.add_draw_object("title", text, 0, 0);

        // Set up objects:
        // Set up text:
        text = GMBitmapText::new(font1, (0.0, 0.0), "BOOO!");
        text.set_align(GMAlign::MiddleCenter);
        object_manager.add_draw_object("ghost_text1", text, 0, 0);

        // Set up ghost sprite:
        let texture = resources.get_texture("tex_ghost1");
        let animation = resources.get_animation("anim_ghost1");
        let mut sprite = GMSprite::new((0.0, 0.0), texture, animation.clone());
        object_manager.add_draw_object("ghost_sprite1", sprite.clone(), 0, 1);

        // Big outer circle:
        let target: GMTarget = ("ghost_sprite1", "small_circle").into();
        let mut circle = GMMVCircle::new(target, (250.0, 250.0), 100.0);
        object_manager.add_normal_object("big_circle", circle, 0);

        let mut interpolate = GMValueInterpolateF32::new(0.0, 360.0, 0.001,
            |value, context, object_manager| {
                let target = "big_circle".into();
                object_manager.send_custom_message1(&target, "set_angle", value, context);
            }
        );
        interpolate.interpolation.repetition = GMRepetition::LoopForward;
        object_manager.add_normal_object("big_angle", interpolate, 0);

        // Small inner circle:
        circle = GMMVCircle::new("ghost_text1", (0.0, 0.0), 50.0);
        object_manager.add_normal_object("small_circle", circle, 0);

        let mut interpolate = GMValueInterpolateF32::new(0.0, 360.0, 0.01,
            |value, context, object_manager| {
                let target = "small_circle".into();
                object_manager.send_custom_message1(&target, "set_angle", value, context);
            }
        );
        interpolate.interpolation.repetition = GMRepetition::LoopForward;
        object_manager.add_normal_object("small_angle", interpolate, 0);

        // 4 ghosts in a circle:
        object_manager.add_draw_object("multi_sprite1", sprite.clone(), 0, 0);
        object_manager.add_draw_object("multi_sprite2", sprite.clone(), 0, 0);
        object_manager.add_draw_object("multi_sprite3", sprite.clone(), 0, 0);
        object_manager.add_draw_object("multi_sprite4", sprite.clone(), 0, 0);

        let multi_circle = GMMVMultiCircle::new( (600.0, 250.0), 100.0, 90.0, 4,
            |positions, context, object_manager| {
                object_manager.send_message_object("multi_sprite1", GMMessage::SetPosition(positions[0]), context);
                object_manager.send_message_object("multi_sprite2", GMMessage::SetPosition(positions[1]), context);
                object_manager.send_message_object("multi_sprite3", GMMessage::SetPosition(positions[2]), context);
                object_manager.send_message_object("multi_sprite4", GMMessage::SetPosition(positions[3]), context);
            }
        );
        object_manager.add_normal_object("multi_circle", multi_circle, 0);

        let mut interpolate = GMValueInterpolateF32::new(0.0, 360.0, 0.005,
            |value, context, object_manager| {
                let target = "multi_circle".into();
                object_manager.send_custom_message1(&target, "set_angle", value, context);
            }
        );
        interpolate.interpolation.repetition = GMRepetition::LoopForward;
        object_manager.add_normal_object("multi_angle", interpolate, 0);

        let mut interpolate = GMValueInterpolateF32::new(50.0, 100.0, 0.02,
            |value, context, object_manager| {
                let target = "multi_circle".into();
                object_manager.send_custom_message1(&target, "set_radius", value, context);
            }
        );
        interpolate.interpolation.repetition = GMRepetition::PingPongForward;
        object_manager.add_normal_object("multi_radius", interpolate, 0);

        // Follow object:
        // Ice sprite:
        let texture = resources.get_texture("tex_ice2");
        let animation = resources.get_empty_animation();
        sprite = GMSprite::new((0.0, 0.0), texture, animation);
        object_manager.add_draw_object("ice_sprite1", sprite, 0, 0);

        // Head:
        let texture = resources.get_texture("tex_head1");
        let animation = resources.get_animation("anim_head1");
        sprite = GMSprite::new((0.0, 0.0), texture, animation);
        object_manager.add_draw_object("head_sprite1", sprite, 0, 0);

        // Ice path:
        let path = GMMVPath::new("ice_sprite1",
            vec![((50.0, 600.0).into(), 0.01),
                ((900.0, 400.0).into(), 0.01),
                ((800.0, 700.0).into(), 0.01),
                ((100.0, 700.0).into(), 0.01)]);

        object_manager.add_normal_object("ice_path", path, 0);

        // Follow:
        let follow = GMMVFollow::new("head_sprite1", "ice_sprite1", 0.01, (512.0, 600.0), (50.0, 600.0));
        object_manager.add_normal_object("follow_ice1", follow, 0);

        // Timer update position:
        let timer = GMTimedMessage::new(GMMessage::Custom0("update_source".into()), "follow_ice1", 1.0, true);
        object_manager.add_normal_object("follow_timer", timer, 0);

        Self {
            object_manager,
        }
    }
}

impl GMSceneT for SpriteScene2 {
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("sprite2.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let sprite2_scene = SpriteScene2::new(&engine);

    engine.add_scene("sprite2_scene", sprite2_scene);
    engine.run();
}

