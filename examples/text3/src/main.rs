
use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::util::{GMAlign, error_panic};
use green_moon_2d::object_manager::GMObjectManager;
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::bitmap_text_effects::{GMTEWave, GMTEShake, GMTERotateChars, GMTEScale};


#[derive(Debug)]
enum CurrentEffect {
    Wave,
    Shake,
    Rotate,
    Scale,
    WaveRotate,
    ShakeScale,
}

#[derive(Debug)]
struct TextScene3 {
    object_manager: GMObjectManager,
    current_effect: CurrentEffect,
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

        text = GMBitmapText::new(font1, (512.0, 250.0), "TO CHANGE EFFECT");
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

        let effect = GMTEWave::new(
            "demo_text", // target
            32.0, // amplitude
            0.1, // speed
            0.2 // offset
        );

        object_manager.add_normal_object_group("wave1", effect, 0, "text_effects");

        let effect = GMTEShake::new(
            "demo_text", // target
            5.0, // radius
            0.2 // speed
        );

        object_manager.add_normal_object_group("shake1", effect, 0, "text_effects");
        object_manager.set_active("shake1", false);

        let effect = GMTERotateChars::new(
            "demo_text", // target
            1.0, // speed
            10.0 // offset
        );

        object_manager.add_normal_object_group("rotate1", effect, 1, "text_effects");
        object_manager.set_active("rotate1", false);

        let effect = GMTEScale::new(
            "demo_text", // target
            0.2, // amplitude
            1.0, // base
            0.1, // speed
            0.2, // offset
        );

        object_manager.add_normal_object_group("scale1", effect, 1, "text_effects");
        object_manager.set_active("scale1", false);

        Self {
            object_manager,
            current_effect: CurrentEffect::Wave,
        }
    }

    fn change_effect(&mut self, effect: CurrentEffect, context: &mut GMContext) {
        self.current_effect = effect;
        // Disable all text effects:
        self.object_manager.set_active_in_group("text_effects", false);

        // Enable only specific text effects:
        match self.current_effect {
            CurrentEffect::Wave => {
                self.object_manager.set_active("wave1", true);
                self.object_manager.send_custom_message1(&"demo_text".into(), "set_text2", "---<<< SINE WAVE >>>---".into(), context);
            }
            CurrentEffect::Shake => {
                self.object_manager.set_active("shake1", true);
                self.object_manager.send_custom_message1(&"demo_text".into(), "set_text2", "..... SHAKE .....".into(), context);
            }
            CurrentEffect::Rotate => {
                self.object_manager.set_active("rotate1", true);
                self.object_manager.send_custom_message1(&"demo_text".into(), "set_text2", ">>>>> ROTATE <<<<<".into(), context);
            }
            CurrentEffect::Scale => {
                self.object_manager.set_active("scale1", true);
                self.object_manager.send_custom_message1(&"demo_text".into(), "set_text2", "--<>() SCALE ()<>--".into(), context);
            }
            CurrentEffect::WaveRotate => {
                self.object_manager.set_active("wave1", true);
                self.object_manager.set_active("rotate1", true);
                self.object_manager.send_custom_message1(&"demo_text".into(), "set_text2", "--- SINE WAVE ROTATE ---".into(), context);
            }
            CurrentEffect::ShakeScale => {
                self.object_manager.set_active("shake1", true);
                self.object_manager.set_active("scale1", true);
                self.object_manager.send_custom_message1(&"demo_text".into(), "set_text2", "--- SHAKE AND SCALE ---".into(), context);
            }
        }
    }

    fn change_property(&mut self, property: u8, amount: f32, context: &mut GMContext) {
        match (&self.current_effect, property) {
            (CurrentEffect::Wave, 1) => {
                self.object_manager.send_custom_message1(&"wave1".into(), "add_amplitude", (amount * 0.5).into(), context);
            }
            (CurrentEffect::Wave, 2) => {
                self.object_manager.send_custom_message1(&"wave1".into(), "add_offset", (amount * 0.01).into(), context);
            }
            (CurrentEffect::Shake, 1) => {
                self.object_manager.send_custom_message1(&"shake1".into(), "add_radius", (amount * 0.1).into(), context);
            }
            (CurrentEffect::Shake, 2) => {
                self.object_manager.send_custom_message1(&"shake1".into(), "add_speed", (amount * 0.01).into(), context);
            }
            (CurrentEffect::Rotate, 1) => {
                self.object_manager.send_custom_message1(&"rotate1".into(), "add_speed", (amount * 0.1).into(), context);
            }
            (CurrentEffect::Rotate, 2) => {
                self.object_manager.send_custom_message1(&"rotate1".into(), "add_offset", (amount * 0.1).into(), context);
            }
            (CurrentEffect::Scale, 1) => {
                self.object_manager.send_custom_message1(&"scale1".into(), "add_speed", (amount * 0.1).into(), context);
            }
            (CurrentEffect::Scale, 2) => {
                self.object_manager.send_custom_message1(&"scale1".into(), "add_amplitude", (amount * 0.1).into(), context);
            }
            (CurrentEffect::WaveRotate, 1) => {
                self.object_manager.send_custom_message1(&"wave1".into(), "add_amplitude", (amount * 0.5).into(), context);
            }
            (CurrentEffect::WaveRotate, 2) => {
                self.object_manager.send_custom_message1(&"rotate1".into(), "add_speed", (amount * 0.1).into(), context);
            }
            (CurrentEffect::ShakeScale, 1) => {
                self.object_manager.send_custom_message1(&"shake1".into(), "add_radius", (amount * 0.1).into(), context);
            }
            (CurrentEffect::ShakeScale, 2) => {
                self.object_manager.send_custom_message1(&"scale1".into(), "add_speed", (amount * 0.1).into(), context);
            }
            _ => {
                error_panic(&format!("Unknown property id: {}", property));
            }
        }
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
            self.change_effect(CurrentEffect::Wave, context);
        }

        if context.event(GMEventCode::Key2Up) {
            self.change_effect(CurrentEffect::Shake, context);
        }

        if context.event(GMEventCode::Key3Up) {
            self.change_effect(CurrentEffect::Rotate, context);
        }

        if context.event(GMEventCode::Key4Up) {
            self.change_effect(CurrentEffect::Scale, context);
        }

        if context.event(GMEventCode::Key5Up) {
            self.change_effect(CurrentEffect::WaveRotate, context);
        }

        if context.event(GMEventCode::Key6Up) {
            self.change_effect(CurrentEffect::ShakeScale, context);
        }

        if context.event(GMEventCode::KeyUpUp) {
            self.change_property(1, 1.0, context)
        }

        if context.event(GMEventCode::KeyDownUp) {
            self.change_property(1, -1.0, context)
        }

        if context.event(GMEventCode::KeyLeftUp) {
            self.change_property(2, -1.0, context)
        }

        if context.event(GMEventCode::KeyRightUp) {
            self.change_property(2, 1.0, context)
        }

        // Reset positions each frame, so that the effects can work on a clean state:
        self.object_manager.send_custom_message0(&"demo_text".into(), "reset_positions", context);
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
