use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::{GMAlign, GMRepetition};
use green_moon_2d::object_manager::GMObjectManager;
use green_moon_2d::sprite::GMSprite;
use green_moon_2d::border::GMBorder;
use green_moon_2d::message::{msgt1v};


#[derive(Debug)]
struct BorderScene1 {
    object_manager: GMObjectManager,
}

impl BorderScene1 {
    fn new(engine: &GMEngine) -> Self {
        // Access to resources
        let resources = engine.get_resources();

        // Crate object manager
        let mut object_manager = GMObjectManager::new();

        // Get first font
        let font1 = resources.get_font("font_cuddly");

        // Set up title text:
        let mut text = GMBitmapText::new(font1, (512.0, 60.0), "BORDER 1");
        text.set_align(GMAlign::BottomCenter);
        object_manager.add_draw_object("title", text, 0, 0);

        // Set up sprites and borders:

        // Border 1
        let texture = resources.get_texture("tex_border1");
        let mut animation = resources.get_animation("anim_border1");
        let sprite1 = GMSprite::new((0.0, 0.0), texture, animation.clone());
        // object_manager.add_draw_object("bat1", sprite, 0, 0);

        let border1 = GMBorder::new2((40.0, 80.0), 500.0, 120.0, sprite1.clone());
        object_manager.add_draw_object("border1", border1, 0, 0);
        object_manager.initialize_object("border1");

        // Border 2:
        let mut border2 = GMBorder::new2((40.0, 240.0), 500.0, 120.0, sprite1.clone());
        animation.current_frame = 1;
        let sprite2 = GMSprite::new((0.0, 0.0), texture, animation.clone());
        border2.set_corners(sprite2);
        object_manager.add_draw_object("border2", border2, 0, 0);
        object_manager.initialize_object("border2");


        Self {
            object_manager,
        }
    }
}

impl GMSceneT for BorderScene1 {
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("border1.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let border1_scene = BorderScene1::new(&engine);

    engine.add_scene("border1_scene", border1_scene);
    engine.run();
}
