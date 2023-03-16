
use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::{GMDrawT, GMAlign, GMRepetition, GMUpdateT, GMFlipXYT};
use green_moon_2d::sprite::{GMSprite};
use green_moon_2d::movement::{GMMV2Points, GMMVRotate, GMMVCircle, GMScaleT};

// use green_moon_2d::animation::{GMAnimation};


#[derive(Debug)]
struct SpriteScene1 {
    title: GMBitmapText,
    bat_sprite: GMSprite,
    ghost_sprite: GMSprite,
    ice1_sprite: GMSprite,
    ice1_movement: GMMV2Points,
    ice1_rotation: GMMVRotate,
    head_sprite: GMSprite,
    head_circle: GMMVCircle,
    ice_troll1_sprite: GMSprite,
    ice_troll1_movement: GMMV2Points,
}

impl SpriteScene1 {
    fn new(engine: &GMEngine) -> Self {
        // Set up title text:
        let resources = engine.get_resources();
        let font = resources.get_font("font_cuddly");
        let mut title = GMBitmapText::new(font, (512.0, 100.0), "SPRITE1");
        title.set_align(GMAlign::BottomCenter);
        title.reset_positions();

        // Bat sprite
        let texture = resources.get_texture("tex_bat1");
        let animation = resources.get_animation("anim_bat1");
        let bat_sprite = GMSprite::new(texture, (512.0, 200.0), animation);

        // Ghost sprite
        let texture = resources.get_texture("tex_ghost1");
        let animation = resources.get_animation("anim_ghost1");
        let mut ghost_sprite = GMSprite::new(texture, (512.0, 250.0), animation);
        ghost_sprite.set_flip_x(true);

        // Ice1 sprite
        let texture = resources.get_texture("tex_ice_cream1");
        let animation = resources.get_empty_animation();
        let ice1_sprite = GMSprite::new(texture, (100.0, 300.0), animation);

        let mut ice1_movement = GMMV2Points::new((100.0, 300.0), (900.0, 300.0), 0.007);
        ice1_movement.get_interpolation_mut().set_repetition(GMRepetition::PingPongForward);
        let mut ice1_rotation = GMMVRotate::new(-30.0, 30.0, 0.05);
        ice1_rotation.get_interpolation_mut().set_repetition(GMRepetition::PingPongForward);

        let texture = resources.get_texture("tex_head1");
        let animation = resources.get_animation("anim_head1");
        let head_sprite = GMSprite::new(texture, (512.0, 400.0), animation);
        let mut head_circle = GMMVCircle::new(90.0-60.0, 90.0+60.0, 0.02, (512.0, 400.0), 70.0);
        head_circle.get_interpolation_mut().set_repetition(GMRepetition::PingPongForward);

        // Ice troll1 sprite
        let texture = resources.get_texture("tex_ice_troll1");
        let animation = resources.get_animation("anim_ice_troll1");
        let mut ice_troll1_sprite = GMSprite::new(texture, (512.0, 600.0), animation);
        ice_troll1_sprite.set_scale(4.0);
        let mut ice_troll1_movement = GMMV2Points::new((100.0, 600.0), (900.0, 600.0), 0.002);
        ice_troll1_movement.get_interpolation_mut().set_repetition(GMRepetition::LoopForward);

        Self {
            title,
            bat_sprite,
            ghost_sprite,
            ice1_sprite,
            ice1_movement,
            ice1_rotation,
            head_sprite,
            head_circle,
            ice_troll1_sprite,
            ice_troll1_movement,
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

        self.bat_sprite.update(context);
        self.ghost_sprite.update(context);

        self.ice1_movement.set_position(&mut self.ice1_sprite);
        self.ice1_movement.update();
        self.ice1_rotation.set_angle(&mut self.ice1_sprite);
        self.ice1_rotation.update();

        self.head_sprite.update(context);
        self.head_circle.set_position(&mut self.head_sprite);
        self.head_circle.update();

        self.ice_troll1_sprite.update(context);
        self.ice_troll1_movement.set_position(&mut self.ice_troll1_sprite);
        self.ice_troll1_movement.update();
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();
        self.title.draw(context);
        self.bat_sprite.draw(context);
        self.ghost_sprite.draw(context);
        self.ice1_sprite.draw(context);
        self.head_sprite.draw(context);
        self.ice_troll1_sprite.draw(context);
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
