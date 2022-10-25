

use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText, GMBitmapTextBuilder};
use green_moon_2d::sprite::{GMSprite, GMSpriteBuilder};
use green_moon_2d::sprite_effect::{GMBoxSpriteEffect, GMSEVelocity};
use green_moon_2d::particle_manager::{GMParticleManager, GMParticleManagerBuilder};
use green_moon_2d::particle_effects::{GMPESimple};
use green_moon_2d::util::{GMAlign, GMRepetition};
use green_moon_2d::math::GMVec2D;
use green_moon_2d::data::GMData;

#[derive(Debug)]
struct ParticleScene1 {
    title: GMBitmapText,
    particle_manager: GMParticleManager,
}

impl ParticleScene1 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();

        // Move title to the center of the window
        let title = GMBitmapTextBuilder::new(resources.get_font("font_cuddly"))
            .with_text("PARTICLE TEST 1")
            .with_position((window_width / 2.0, 32.0))
            .with_align(GMAlign::TopCenter)
            .build();

        let sprite_effect1 = GMSEVelocity::new((0.0, 0.0));

        let head_sprite = GMSpriteBuilder::new(resources.get_texture("tex_head1"))
            .with_position((512.0, 384.0))
            .with_animation(resources.get_animation("anim_head1"))
            .with_effect(sprite_effect1)
            .build();

        let messages = vec![
            (0, "set_random_direction".to_string(), GMData::F32F32(0.0, 360.0)), // sprite effect index, message, data
            (0, "set_random_speed".to_string(), GMData::F32F32(3.0, 4.0)) // sprite effect index, message, data
        ];

        let particle_effect1 = GMPESimple::new(messages);

        let particle_manager = GMParticleManagerBuilder::new(head_sprite)
            .with_position((400.0, 400.0))
            .with_wait_time(1.0, 1.5)
            .with_run_time(2.0, 3.0)
            .with_max_num_of_particles(30)
            .with_effect(particle_effect1)
            .build();


        Self {
            title,
            particle_manager,
        }
    }
}

impl GMSceneT for ParticleScene1 {
    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        self.particle_manager.update(context);
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.title.draw(context);

        self.particle_manager.draw(context);
    }
}

fn main() {
    let log_config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("particle1.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let particle1_scene = ParticleScene1::new(&engine);

    engine.add_scene("particle1_scene", particle1_scene);
    engine.run();
}
