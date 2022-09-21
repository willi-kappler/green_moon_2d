

use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText, GMBitmapTextBuilder};
use green_moon_2d::sprite::{GMSprite, GMSpriteBuilder};
use green_moon_2d::sprite_effect::{GMSELinearMovement, GMSECircularMovement};
use green_moon_2d::util::{GMAlign, GMRepetition};

#[derive(Debug)]
struct SpriteScene1 {
    title: GMBitmapText,
    sprites: Vec<GMSprite>,
}

impl SpriteScene1 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();

        // Move title to the center of the window
        let title = GMBitmapTextBuilder::new(resources.get_font("font_cuddly"))
            .with_text("SPRITE TEST 1")
            .with_position((window_width / 2.0, 32.0))
            .with_align(GMAlign::TopCenter)
            .build();

        let mut sprites = Vec::new();

        let effect1 = GMSELinearMovement::new(
            (100.0, 100.0), // start position
            (900.0, 100.0), // end position
            0.004, // speed
            GMRepetition::LoopForward);

        sprites.push(GMSpriteBuilder::new(resources.get_texture("tex_bat1"))
            .with_position((100.0, 100.0))
            .with_animation(resources.get_animation("anim_bat1"))
            .with_effect(effect1)
            .build());

        sprites.push(GMSpriteBuilder::new(resources.get_texture("tex_explosion1"))
            .with_position((100.0, 150.0))
            .with_animation(resources.get_animation("anim_explosion1"))
            .build());

        let effect2 = GMSECircularMovement::new(
            (200.0, 300.0), // center
            50.0, // radius
            0.01, // speed
            GMRepetition::LoopForward);

        sprites.push(GMSpriteBuilder::new(resources.get_texture("tex_ghost1"))
            .with_position((200.0, 300.0))
            .with_animation(resources.get_animation("anim_ghost1"))
            .with_effect(effect2)
            .build());

        Self {
            title,
            sprites,
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

        for sprite in self.sprites.iter_mut() {
            sprite.update(context)
        }
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.title.draw(context);

        for sprite in self.sprites.iter() {
            sprite.draw(context);
        }
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
