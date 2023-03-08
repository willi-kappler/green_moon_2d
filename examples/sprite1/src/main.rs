

use std::fs::File;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::sprite::GMSpriteBuilder;
use green_moon_2d::interpolation::{GMInterpolatePosition, GMInterpolateRotation, GMInterpolateVec2D, GMInterpolateF32};
use green_moon_2d::util::GMRepetition;


#[derive(Debug)]
struct SpriteScene1 {
}

impl GMSceneT for SpriteScene1 {
    fn init(&mut self, context: &mut GMContext) {
        let resources = context.resources_mut();

        // Set some sprite properties:
        let bat1_texture = resources.get_texture("tex_bat1");
        let bat1_animation = resources.get_animation("anim_bat1");
        let bat1_position = (512.0, 200.0);

        let ghost1_texture = resources.get_texture("tex_ghost1");
        let ghost1_animation = resources.get_animation("anim_ghost1");
        let ghost1_position = (512.0, 250.0);

        let ice_cream1_texture = resources.get_texture("tex_ice_cream1");
        let ice_cream1_position = (100.0, 300.0);
        let mut ice_cream1_interpolate1 = GMInterpolateVec2D::new((100.0, 300.0), (900.0, 300.0), 4.0);
        ice_cream1_interpolate1.repetition = GMRepetition::PingPongForward;
        let ice_cream1_movement = GMInterpolatePosition(ice_cream1_interpolate1);
        let mut ice_cream1_interpolate2 = GMInterpolateF32::new(-30.0, 30.0, 2.0);
        ice_cream1_interpolate2.repetition = GMRepetition::PingPongForward;
        let ice_cream1_rotation = GMInterpolateRotation(ice_cream1_interpolate2);


        let world = context.world_mut();

        // Bat sprite:
        let _sprite = GMSpriteBuilder::new(bat1_texture, bat1_position)
            // Animation is added as a component to the sprite entity:
            .add_component(bat1_animation)
            // Creates a new entity and adds it to the world:
            .build(world);

        // Ghost sprite:
        let _sprite = GMSpriteBuilder::new(ghost1_texture, ghost1_position)
            .flip_x(true)
            .add_component(ghost1_animation)
            .build(world);

        // Ice cream1
        let _sprite = GMSpriteBuilder::new(ice_cream1_texture, ice_cream1_position)
            .add_component(ice_cream1_movement)
            .add_component(ice_cream1_rotation)
            .build(world);
    }

    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        // Queries all the entities who have an animation
        context.process_animations();
        context.interpolate_position();
        context.interpolate_rotation();
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
