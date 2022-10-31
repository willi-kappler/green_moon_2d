

use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText, GMBitmapTextBuilder};
use green_moon_2d::sprite::{GMSpriteBuilder};
use green_moon_2d::sprite_effect::{GMSEVelocity, GMSERotation1};
use green_moon_2d::particle_manager::{GMParticleManager, GMParticleManagerBuilder};
use green_moon_2d::particle_effects::{GMPESimple};
use green_moon_2d::util::{GMAlign};
use green_moon_2d::data::GMData;

#[derive(Debug)]
struct ParticleScene1 {
    title: GMBitmapText,
    particle_manager1: GMParticleManager,
    particle_manager2: GMParticleManager,
}

impl ParticleScene1 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();
        let window_height = engine.window_height();
        const X_OFFSET: f32 = 350.0;

        // Move title to the center of the window
        let title = GMBitmapTextBuilder::new(resources.get_font("font_cuddly"))
            .with_text("PARTICLE TEST 1")
            .with_position((window_width / 2.0, 32.0))
            .with_align(GMAlign::TopCenter)
            .build();


        // Particle Manager 1
        let sprite_effect1 = GMSEVelocity::new((0.0, 0.0));

        let head_sprite1 = GMSpriteBuilder::new(resources.get_texture("tex_head1"))
            .with_position((0.0, 0.0))
            .with_animation(resources.get_animation("anim_head1"))
            .with_effect(sprite_effect1.clone())
            .build();

        let messages1 = vec![
            // Index 0 for velocity sprite effect:
            (0, "set_random_direction", GMData::F32F32(90.0, 270.0)), // sprite effect index, message, data
            (0, "set_random_speed", GMData::F32F32(3.0, 4.0)) // sprite effect index, message, data
        ];

        let particle_effect1 = GMPESimple::new(messages1);

        let particle_manager1 = GMParticleManagerBuilder::new(head_sprite1)
            .with_position((X_OFFSET, window_height / 2.0))
            .with_wait_time(1.0, 1.5)
            .with_run_time(1.0, 2.0)
            .with_max_num_of_particles(30)
            .with_effect(particle_effect1)
            .build();

        // Particle Manager 2
        let sprite_effect2 = GMSERotation1::new(10.0);

        let head_sprite2 = GMSpriteBuilder::new(resources.get_texture("tex_head1"))
            .with_position((0.0, 0.0))
            .with_animation(resources.get_animation("anim_head1"))
            // Sprite effect velocity, index 0:
            .with_effect(sprite_effect1)
            // Sprite effect velocity, index 1:
            .with_effect(sprite_effect2)
            .build();

        let messages2 = vec![
            // Index 0 for velocity sprite effect:
            (0, "set_random_direction", GMData::F32F32(270.0, 450.0)), // sprite effect index, message, data
            (0, "set_random_speed", GMData::F32F32(3.0, 4.0)), // sprite effect index, message, data
            // Now index 1 for for the rotation sprite effect:
            (1, "set_random_speed", GMData::F32F32(-10.0, 10.0)) // sprite effect index, message, data
        ];

        let particle_effect2 = GMPESimple::new(messages2);

        let particle_manager2 = GMParticleManagerBuilder::new(head_sprite2)
            .with_position((window_width - X_OFFSET, window_height / 2.0))
            .with_wait_time(1.0, 1.5)
            .with_run_time(1.0, 2.0)
            .with_max_num_of_particles(30)
            .with_effect(particle_effect2)
            .build();

        Self {
            title,
            particle_manager1,
            particle_manager2,
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

        self.particle_manager1.update(context);
        self.particle_manager2.update(context);
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.title.draw(context);

        self.particle_manager1.draw(context);
        self.particle_manager2.draw(context);
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
