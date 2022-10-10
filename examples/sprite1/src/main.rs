

use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText, GMBitmapTextBuilder};
use green_moon_2d::sprite::{GMSprite, GMSpriteBuilder};
use green_moon_2d::sprite_effect::{GMBoxSpriteEffect, GMSELinearMovement, GMSECircularMovement,
    GMSEPolygonMovement, GMSETimed, GMSERotating, GMSEScaling, GMSETarget, GMSEFollow};
use green_moon_2d::util::{GMAlign, GMRepetition};
use green_moon_2d::math::GMVec2D;

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

        // Bat
        let effect1 = GMSELinearMovement::new(
            (100.0, 100.0), // start position
            (900.0, 100.0), // end position
            4.0, // speed
            GMRepetition::LoopForward);

        sprites.push(GMSpriteBuilder::new(resources.get_texture("tex_bat1"))
            .with_position((100.0, 100.0))
            .with_animation(resources.get_animation("anim_bat1"))
            .with_effect(effect1)
            .build());

        // Explosion
        sprites.push(GMSpriteBuilder::new(resources.get_texture("tex_explosion1"))
            .with_position((100.0, 150.0))
            .with_animation(resources.get_animation("anim_explosion1"))
            .build());

        // Ghost
        // Add multiple effects as vec.
        let effects: Vec<GMBoxSpriteEffect> = vec![Box::new(GMSECircularMovement::new(
            (200.0, 300.0), // center
            50.0, // radius
            0.01, // speed
            GMRepetition::LoopForward)),

            Box::new(GMSETimed::new(
                3.0, // duration, call closure after every three seconds
                true, // repeat
                // Closure:
                |sprite, _| -> () {
                    sprite.flip_x = !sprite.flip_x;
                }
            ))
        ];

        sprites.push(GMSpriteBuilder::new(resources.get_texture("tex_ghost1"))
            .with_position((200.0, 300.0))
            .with_animation(resources.get_animation("anim_ghost1"))
            .with_effects(effects)
            .build());

        // Ice cream
        // Add effects individually
        let effect1 = GMSERotating::new(
            0.0, // initial angle
            -20.0, // min angle
            20.0, // max angle
            2.0 // speed
        );

        let effect2 = GMSEScaling::new(
            1.0, // initial size
            0.5, // min size
            1.5, // max size
            0.01 // speed
        );

        let positions = vec![
            GMVec2D::new(400.0, 200.0),
            GMVec2D::new(900.0, 300.0),
            GMVec2D::new(900.0, 600.0),
            GMVec2D::new(100.0, 700.0),
            GMVec2D::new(100.0, 400.0),
            GMVec2D::new(300.0, 400.0),
            GMVec2D::new(400.0, 200.0),
        ];

        let speeds = vec![
            2.0,
            4.0,
            3.0,
            1.0,
            3.0,
            1.0,
        ];

        let effect3 = GMSEPolygonMovement::new(
            positions, // polygon corners
            speeds, // movement speed between corners
            GMRepetition::LoopForward,
        );

        let effect4 = GMSETarget::new(
            1.0, // Update once per second
            "ice_cream", // name of target
        );

        sprites.push(GMSpriteBuilder::new(resources.get_texture("tex_ice_cream1"))
            .with_position((400.0, 200.0))
            .with_effect(effect1)
            .with_effect(effect2)
            .with_effect(effect3)
            .with_effect(effect4)
            .build());

        // Head
        let effect1 = GMSEFollow::new(
            1.0, // Update once per second
            "ice_cream", // name of target
            0.8, // speed of follower
        );

        sprites.push(GMSpriteBuilder::new(resources.get_texture("tex_head1"))
            .with_position((512.0, 384.0))
            .with_animation(resources.get_animation("anim_head1"))
            .with_effect(effect1)
            .build());

        // Ice cream 2
        sprites.push(GMSpriteBuilder::new(resources.get_texture("tex_ice_cream2"))
            .with_position((800.0, 50.0))
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

        let head_index = 4;
        self.sprites[head_index].base.move_step();
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
