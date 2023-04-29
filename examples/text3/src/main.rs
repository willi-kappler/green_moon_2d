
use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::util::GMAlign;
use green_moon_2d::object::{GMObjectManager, GMTarget};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::bitmap_text_effects::{GMTEWave};

#[derive(Debug)]
struct TextScene3 {
    object_manager: GMObjectManager,
    target: GMTarget,
}

impl TextScene3 {
    fn new(engine: &GMEngine) -> Self {
        // Access to resources
        let resources = engine.get_resources();

        // Crate object manager
        let mut object_manager = GMObjectManager::new();

        // Get first font
        let font1 = resources.get_font("font_cuddly");

        // Set up title text:
        let mut text = GMBitmapText::new(font1, (512.0, 100.0), "TEXT 3");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object("title", text, 0, 0);

        // Set up description:
        text = GMBitmapText::new(font1, (512.0, 200.0), "PRESS NUMBERS");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object("description1", text, 0, 0);

        text = GMBitmapText::new(font1, (512.0, 300.0), "TO CHANGE EFFECT");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object("description2", text, 0, 0);

        text = GMBitmapText::new(font1, (512.0, 350.0), "PRESS CURSOR KEYS");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object("description3", text, 0, 0);

        text = GMBitmapText::new(font1, (512.0, 400.0), "TO CHANGE VALUES");
        text.set_align(GMAlign::BottomCenter);

        object_manager.add_draw_object("description4", text, 0, 1);

        text = GMBitmapText::new(font1, (512.0, 500.0), "---<<< SINE WAVE >>>---");
        text.set_align(GMAlign::BottomCenter);

        // Draw index is 1, so it's drawn over the other text
        object_manager.add_draw_object("demo_text", text, 0, 1);

        let mut effect = GMTEWave::new(
            "demo_text", // target
            32.0, // amplitude
            0.1, // speed
            0.2 // offset
        );

        object_manager.add_normal_object_group("wave1", effect, 0, "text_effects");

        Self {
            object_manager,
            target: "demo_text".into(),
        }
    }

    fn change_effect(&mut self, effect: &str) {
        // Disable all text effects:
        self.object_manager.set_active_in_group("text_effects", false);
        // Enable only this text effect:
        self.object_manager.set_active(effect, true);
    }
}

impl GMSceneT for TextScene3 {
    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        if context.event(GMEventCode::Key1Up) {
            self.change_effect("wave1");
        }

        if context.event(GMEventCode::Key2Up) {
        }

        if context.event(GMEventCode::Key3Up) {
        }

        if context.event(GMEventCode::Key4Up) {
        }

        if context.event(GMEventCode::Key5Up) {
        }

        if context.event(GMEventCode::Key6Up) {
        }

        if context.event(GMEventCode::Key7Up) {
        }

        if context.event(GMEventCode::Key8Up) {
        }

        if context.event(GMEventCode::Key9Up) {
        }

        if context.event(GMEventCode::KeyUpUp) {
        }

        if context.event(GMEventCode::KeyDownUp) {
        }

        if context.event(GMEventCode::KeyLeftUp) {
        }

        if context.event(GMEventCode::KeyRightUp) {
        }

        self.object_manager.send_custom_message0(&self.target, "reset_positions", context);
        self.object_manager.update(context);
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.object_manager.draw(context);
    }
}

fn main() {
    let log_config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("text3.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let text3_scene = TextScene3::new(&engine);

    engine.add_scene("text3_scene", text3_scene);
    engine.run();
}
