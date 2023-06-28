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
struct BorderScene2 {
    object_manager: GMObjectManager,
}

impl BorderScene2 {
    fn new(engine: &GMEngine) -> Self {
        // Access to resources
        let resources = engine.get_resources();

        // Crate object manager
        let mut object_manager = GMObjectManager::new();

        // Get first font
        let font1 = resources.get_font("font_cuddly");

        // Set up title text:
        let mut text = GMBitmapText::new(font1, (512.0, 60.0), "BORDER 2");
        text.set_align(GMAlign::BottomCenter);
        object_manager.add_draw_object("title", text, 0, 0);

        // Set up sprites and borders:

        // Border 1, animated
        let texture1 = resources.get_texture("tex_border1");
        let animation1 = resources.get_animation("anim_border1");
        let sprite1 = GMSprite::new((0.0, 0.0), texture1, animation1);
        let border1 = GMBorder::new2((40.0, 80.0), 500.0, 120.0, sprite1.clone());
        object_manager.add_draw_object("border1", border1, 0, 0);
        object_manager.initialize_object("border1");


        Self {
            object_manager,
        }
    }
}

impl GMSceneT for BorderScene2 {
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("border2.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let border2_scene = BorderScene2::new(&engine);

    engine.add_scene("border2_scene", border2_scene);
    engine.run();
}
