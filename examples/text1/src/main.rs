
use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::GMAlign;
use green_moon_2d::target::GMTarget;
use green_moon_2d::object_manager::GMObjectManager;


#[derive(Debug)]
struct TextScene1 {
    object_manager: GMObjectManager,
    target_group: GMTarget,
    target_d3: GMTarget,
    target_d4: GMTarget,
}

impl TextScene1 {
    fn new(engine: &GMEngine) -> Self {
        // Access to resources
        let resources = engine.get_resources();

        // Crate object manager
        let mut object_manager = GMObjectManager::new();

        // Get first font
        let font1 = resources.get_font("font_cuddly");

        // Set up title text:
        let mut text = GMBitmapText::new(font1, (512.0, 100.0), "TEXT 1");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object_group("title", text, 0, 0, "style1");

        // Set up description:
        text = GMBitmapText::new(font1, (512.0, 200.0), "PRESS 1 2 3");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object_group("description1", text, 0, 0, "style1");

        text = GMBitmapText::new(font1, (512.0, 300.0), "TO CHANGE FONT");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object_group("description2", text, 0, 0, "style1");

        text = GMBitmapText::new(font1, (512.0, 350.0), "THIS IS FONT 1");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object_group("description3", text, 0, 0, "style1");

        text = GMBitmapText::new(font1, (512.0, 400.0), "CUDDLY");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object_group("description4", text, 0, 0, "style1");

        Self {
            object_manager,
            target_group: GMTarget::Group("style1".to_string()),
            target_d3: "description3".into(),
            target_d4: "description4".into(),
        }
    }

    fn change_text_and_font(&mut self, text1: &str, text2: &str, font: &str, context: &mut GMContext) {
        self.object_manager.send_custom_message1(&self.target_d3, "set_text", text1, context);
        self.object_manager.send_custom_message1(&self.target_d4, "set_text", text2, context);
        self.object_manager.send_custom_message1(&self.target_group, "set_font2", font, context);
    }
}

impl GMSceneT for TextScene1 {
    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        if context.event(GMEventCode::Key1Up) {
            self.change_text_and_font("THIS IS FONT 1", "CUDDLY", "font_cuddly", context);
        }

        if context.event(GMEventCode::Key2Up) {
            self.change_text_and_font("THIS IS FONT 2", "BLAGGER", "font_blagger", context);
        }

        if context.event(GMEventCode::Key3Up) {
            self.change_text_and_font("THIS IS FONT 3", "BBC", "font_bbc", context);
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("text1.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let text1_scene = TextScene1::new(&engine);

    engine.add_scene("text1_scene", text1_scene);
    engine.run();
}
