
use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::GMAlign;
use green_moon_2d::object::{GMObjectManager, GMTarget};

#[derive(Debug)]
struct TextScene1 {
    object_manager: GMObjectManager,
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
        text.align = GMAlign::BottomCenter;
        text.reset_chars();

        object_manager.add_draw_object("title", text, 0, 0);
        object_manager.add_group("title", "style1");

        // Set up description:
        text = GMBitmapText::new(font1, (512.0, 200.0), "PRESS 1 2 3");
        text.align = GMAlign::BottomCenter;
        text.reset_chars();

        object_manager.add_draw_object("description1", text, 0, 0);
        object_manager.add_group("description1", "style1");

        text = GMBitmapText::new(font1, (512.0, 300.0), "TO CHANGE FONT");
        text.align = GMAlign::BottomCenter;
        text.reset_chars();

        object_manager.add_draw_object("description2", text, 0, 0);
        object_manager.add_group("description2", "style1");

        text = GMBitmapText::new(font1, (512.0, 350.0), "THIS IS FONT 1");
        text.align = GMAlign::BottomCenter;
        text.reset_chars();

        object_manager.add_draw_object("description3", text, 0, 0);
        object_manager.add_group("description3", "style1");

        text = GMBitmapText::new(font1, (512.0, 400.0), "CUDDLY");
        text.align = GMAlign::BottomCenter;
        text.reset_chars();

        object_manager.add_draw_object("description4", text, 0, 0);
        object_manager.add_group("description4", "style1");

        Self {
            object_manager,
        }
    }

    fn change_text_and_font(&mut self, text1: &str, text2: &str, font: &str, context: &mut GMContext) {
        self.object_manager.send_custom_message2(&"description3".into(), "set_text".into(), text1.into(), context);
        self.object_manager.send_custom_message2(&"description4".into(), "set_text".into(), text2.into(), context);

        self.object_manager.send_custom_message2(&GMTarget::Group("style1".to_string()), "set_font2".into(), font.into(), context);    
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

/*             self.object_manager.send_custom_message2(&"description3".into(), "set_text".into(), "THIS IS FONT 1".into(), context);
            self.object_manager.send_custom_message2(&"description4".into(), "set_text".into(), "CUDDLY".into(), context);

            self.object_manager.send_custom_message2(&GMTarget::Group("style1".to_string()), "set_font2".into(), "font_cuddly".into(), context); */

/*             self.descriptions[2].set_text("THIS IS FONT 1");
            self.descriptions[3].set_text("CUDDLY");

            for text in self.descriptions.iter_mut() {
                text.set_font2(&self.font1);
            } */
        }

        if context.event(GMEventCode::Key2Up) {
            self.change_text_and_font("THIS IS FONT 2", "BLAGGER", "font_blagger", context);

/*             self.object_manager.send_custom_message2(&"description3".into(), "set_text".into(), "THIS IS FONT 2".into(), context);
            self.object_manager.send_custom_message2(&"description4".into(), "set_text".into(), "BLAGGER".into(), context);

            self.object_manager.send_custom_message2(&GMTarget::Group("style1".to_string()), "set_font2".into(), "font_blagger".into(), context); */

/*             self.descriptions[2].set_text("THIS IS FONT 2");
            self.descriptions[3].set_text("BLAGGER");

            for text in self.descriptions.iter_mut() {
                text.set_font2(&self.font2);
            } */
        }

        if context.event(GMEventCode::Key3Up) {
            self.change_text_and_font("THIS IS FONT 3", "BBC", "font_bbc", context);

/*             self.object_manager.send_custom_message2(&"description3".into(), "set_text".into(), "THIS IS FONT 3".into(), context);
            self.object_manager.send_custom_message2(&"description4".into(), "set_text".into(), "BBC".into(), context);

            self.object_manager.send_custom_message2(&GMTarget::Group("style1".to_string()), "set_font2".into(), "font_bbc".into(), context); */

/*             self.descriptions[2].set_text("THIS IS FONT 3");
            self.descriptions[3].set_text("BBC");

            for text in self.descriptions.iter_mut() {
                text.set_font2(&self.font3);
            } */
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
