

use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::sprite::GMSpriteBuilder;


#[derive(Debug)]
struct SpriteScene1 {
}

impl GMSceneT for SpriteScene1 {
    fn init(&mut self, context: &mut GMContext) {
        let resources = context.resources_mut();

        // Set some sprite properties:
        let texture = resources.get_texture("tex_bat1");
        let animation = resources.get_animation("anim_bat1");
        let position = (512.0, 200.0);

        let _sprite = GMSpriteBuilder::new(texture.clone(), position)
            // Animation is added as a component to the sprite entity:
            .add_component(animation)
            // Creates a new entity and adds it to the world:
            .build(context.get_world_mut());


    }

    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        // Queries all the entities who have an animation
        context.process_animations();
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        // Queries all the entities who have a texture and draws that texture to the screen
        context.draw_textures();
    }
}

fn main() {
    let log_config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("sprite1.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let sprite1_scene = SpriteScene1 {};

    engine.add_scene("sprite1_scene", sprite1_scene);
    engine.run();
}
