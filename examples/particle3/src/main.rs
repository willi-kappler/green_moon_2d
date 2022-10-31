

use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText, GMBitmapTextBuilder};
use green_moon_2d::sprite::{GMSpriteBuilder};
use green_moon_2d::sprite_effect::{GMSEVelocity};
use green_moon_2d::particle_manager::{GMParticleManager, GMParticleManagerBuilder};
use green_moon_2d::particle_effects::{GMPESpiral};
use green_moon_2d::util::{GMAlign};
use green_moon_2d::data::GMData;

#[derive(Debug)]
struct ParticleScene3 {
    title: GMBitmapText,
    particle_manager1: GMParticleManager,
    particle_manager2: GMParticleManager,
}

impl ParticleScene3 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();
        let window_height = engine.window_height();
        const X_OFFSET: f32 = 250.0;

        // Move title to the center of the window
        let title = GMBitmapTextBuilder::new(resources.get_font("font_cuddly"))
            .with_text("PARTICLE TEST 3")
            .with_position((window_width / 2.0, 32.0))
            .with_align(GMAlign::TopCenter)
            .build();

        // Particle Manager 1
        let sprite_effect1 = GMSEVelocity::new((0.0, 0.0));

        let particle_sprite1 = GMSpriteBuilder::new(resources.get_texture("tex_particle1"))
            .with_position((0.0, 0.0))
            .with_animation(resources.get_animation("anim_particle1"))
            .with_effect(sprite_effect1.clone())
            .build();

        let particle_effect1 = GMPESpiral::new2(10.0, 0.5);

        let particle_manager1 = GMParticleManagerBuilder::new(particle_sprite1)
            .with_position((X_OFFSET, window_height / 2.0))
            .with_run_time(15.0, 15.0)
            .with_max_num_of_particles(100)
            .with_effect(particle_effect1)
            .build();

        // Particle Manager 2
        let particle_sprite2 = GMSpriteBuilder::new(resources.get_texture("tex_particle2"))
            .with_position((0.0, 0.0))
            .with_animation(resources.get_animation("anim_particle1"))
            .with_effect(sprite_effect1)
            .build();

        let particle_effect2 = GMPESpiral::new2(-10.0, 0.5);

        let particle_manager2 = GMParticleManagerBuilder::new(particle_sprite2)
            .with_position((window_width - X_OFFSET, window_height / 2.0))
            .with_run_time(15.0, 15.0)
            .with_max_num_of_particles(100)
            .with_effect(particle_effect2)
            .build();

        Self {
            title,
            particle_manager1,
            particle_manager2,
        }
    }
}

impl GMSceneT for ParticleScene3 {
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("particle3.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let particle3_scene = ParticleScene3::new(&engine);

    engine.add_scene("particle3_scene", particle3_scene);
    engine.run();
}
