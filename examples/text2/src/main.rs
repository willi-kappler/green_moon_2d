
use std::fs::File;
use std::rc::Rc;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::GMAlign;
use green_moon_2d::object_manager::GMObjectManager;
use green_moon_2d::target::GMTarget;
use green_moon_2d::value::GMValue;

#[derive(Debug)]
struct TextScene2 {
    object_manager: GMObjectManager,
    target: GMTarget,
}

impl TextScene2 {
    fn new(engine: &GMEngine) -> Self {
        // Access to resources
        let resources = engine.get_resources();

        // Crate object manager
        let mut object_manager = GMObjectManager::new();

        // Get first font
        let font1 = resources.get_font("font_cuddly");

        // Set up title text:
        let mut text = GMBitmapText::new(font1, (512.0, 100.0), "TEXT 2");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object("title", text, 0, 0);

        // Set up description:
        text = GMBitmapText::new(font1, (512.0, 200.0), "PRESS 1 - 9");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object("description1", text, 0, 0);

        text = GMBitmapText::new(font1, (512.0, 300.0), "TO CHANGE ALIGNMENT");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object("description2", text, 0, 0);

        text = GMBitmapText::new(font1, (512.0, 350.0), "PRESS H FOR ORIENTATION");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object("description3", text, 0, 0);

        text = GMBitmapText::new(font1, (512.0, 450.0), "--- DEMO TEXT ---");
        text.set_align(GMAlign::BottomCenter);

        // Draw index is 1, so it's drawn over the other text
        object_manager.add_draw_object("demo_text", text, 0, 1);

        Self {
            object_manager,
            target: GMTarget::Single("demo_text".to_string()),
        }
    }

    fn change_alignment(&mut self, align: GMAlign, context: &mut GMContext) {
        self.object_manager.send_custom_message1(&self.target, "set_align2", GMValue::Any(Rc::new(align)), context);
    }
}

impl GMSceneT for TextScene2 {
    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        if context.event(GMEventCode::Key1Up) {
            self.change_alignment(GMAlign::TopLeft, context);
        }

        if context.event(GMEventCode::Key2Up) {
            self.change_alignment(GMAlign::TopCenter, context);
        }

        if context.event(GMEventCode::Key3Up) {
            self.change_alignment(GMAlign::TopRight, context);
        }

        if context.event(GMEventCode::Key4Up) {
            self.change_alignment(GMAlign::MiddleLeft, context);
        }

        if context.event(GMEventCode::Key5Up) {
            self.change_alignment(GMAlign::MiddleCenter, context);
        }

        if context.event(GMEventCode::Key6Up) {
            self.change_alignment(GMAlign::MiddleRight, context);
        }

        if context.event(GMEventCode::Key7Up) {
            self.change_alignment(GMAlign::BottomLeft, context);
        }

        if context.event(GMEventCode::Key8Up) {
            self.change_alignment(GMAlign::BottomCenter, context);
        }

        if context.event(GMEventCode::Key9Up) {
            self.change_alignment(GMAlign::BottomRight, context);
        }

        if context.event(GMEventCode::KeyHUp) {
            self.object_manager.send_custom_message0(&self.target, "toggle_horizontal2", context);
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("text2.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let text2_scene = TextScene2::new(&engine);

    engine.add_scene("text2_scene", text2_scene);
    engine.run();
}
