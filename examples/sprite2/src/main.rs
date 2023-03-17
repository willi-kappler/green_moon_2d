
use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::{GMDrawT, GMAlign, GMRepetition, GMUpdateT};
use green_moon_2d::sprite::{GMSprite};
use green_moon_2d::movement::{GMMVCircle};


#[derive(Debug)]
struct SpriteScene2 {
    title: GMBitmapText,
    ghost_text: GMBitmapText,
    ghost_sprite: GMSprite,
    ghost_circle: GMMVCircle,
    text_circle: GMMVCircle,
}

impl SpriteScene2 {
    fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();

        // Set up title text:
        let font = resources.get_font("font_cuddly");
        let mut title = GMBitmapText::new(font.clone(), (512.0, 100.0), "SPRITE 2");
        title.set_align(GMAlign::BottomCenter);
        title.reset_positions();


        // Set up circle text
        let mut ghost_text = GMBitmapText::new(font, (0.0, 0.0), "BOOO!");
        ghost_text.set_align(GMAlign::MiddleCenter);
        ghost_text.reset_positions();

        // Set up ghost sprite
        let texture = resources.get_texture("tex_ghost1");
        let animation = resources.get_animation("anim_ghost1");
        let ghost_sprite = GMSprite::new(texture, (0.0, 0.0), animation);

        // Set up circle movement for ghost
        let mut ghost_circle = GMMVCircle::new(0.0, 360.0, 0.001, (250.0, 250.0), 100.0);
        ghost_circle.get_interpolation_mut().set_repetition(GMRepetition::LoopForward);

        // Set up circle movement for text
        let mut text_circle = GMMVCircle::new(0.0, 360.0, 0.01, (0.0, 0.0 / 2.0), 50.0);
        text_circle.get_interpolation_mut().set_repetition(GMRepetition::LoopForward);

        Self {
            title,
            ghost_text,
            ghost_sprite,
            ghost_circle,
            text_circle,
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

        self.ghost_sprite.update(context);

        self.ghost_circle.set_position_of(&mut self.ghost_sprite);
        self.ghost_circle.set_position_of(&mut self.text_circle);
        self.ghost_circle.update();

        self.text_circle.set_and_update(&mut self.ghost_text);
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();
        self.title.draw(context);
        self.ghost_text.draw(context);
        self.ghost_sprite.draw(context);
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
