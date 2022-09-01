

use std::fs::File;
use std::any::Any;

// use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::GMBitmapText;
use green_moon_2d::bitmap_text_effects::{GMTextEffectT, GMTextEffectEmpty, GMTextEffectWave,
    GMTextEffectShake, GMTextEffectRotateChars, GMTextEffectMultiple};
use green_moon_2d::util::GMAlign;


#[derive(Debug)]
struct TextScene3 {
    texts: Vec<GMBitmapText>,
    effects: Vec<Box<dyn GMTextEffectT>>,
    current_effect: usize,
}

impl TextScene3 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();

        let font = resources.get_font_clone("font_cuddly");

        const space: f32 = 50.0;
        let mut texts = Vec::new();

        texts.push(GMBitmapText::new(&font, "TEXT TEST 3", window_width / 2.0, 32.0 + (1.0 * space)));
        texts.push(GMBitmapText::new(&font, "PRESS NUMBER TO CHANGE EFFECT", 32.0, 32.0 + (5.0 * space)));
        texts.push(GMBitmapText::new(&font, "CURSOR TO CHANGE SETTING", 32.0, 32.0 + (6.0 * space)));

        // Move title to the center of the window
        texts[0].align(GMAlign::TopCenter);

        let mut effects: Vec<Box<dyn GMTextEffectT>> = Vec::new();

        effects.push(Box::new(GMTextEffectEmpty::new()));
        effects.push(Box::new(GMTextEffectWave::new(32.0, 0.1, 0.2)));
        effects.push(Box::new(GMTextEffectShake::new(5.0, 0.2)));
        effects.push(Box::new(GMTextEffectRotateChars::new(1.0, 10.0)));

        let mut combined = GMTextEffectMultiple::new();
        combined.add_text_effect(GMTextEffectWave::new(32.0, 0.1, 0.2));
        combined.add_text_effect(GMTextEffectShake::new(5.0, 0.2));
        effects.push(Box::new(combined));

        let mut combined = GMTextEffectMultiple::new();
        combined.add_text_effect(GMTextEffectWave::new(32.0, 0.1, 0.2));
        combined.add_text_effect(GMTextEffectRotateChars::new(1.0, 10.0));
        effects.push(Box::new(combined));

        Self {
            texts,
            effects,
            current_effect: 0,
        }
    }

    pub fn change_settings1(&mut self, delta: f32, context: &mut GMContext) {
        let effect = &mut self.effects[self.current_effect];

        match self.current_effect {
            1 => {
                effect.send_message_f32("add_speed", delta * 0.1, context);
            }
            2 => {
                effect.send_message_f32("add_speed", delta * 0.1, context);
            }
            3 => {
                effect.send_message_f32("add_speed", delta, context);
            }
            4 => {
                effect.send_message_multiple_f32("send_message", 0, "add_speed", delta * 0.1, context);
            }
            5 => {
                effect.send_message_multiple_f32("send_message", 0, "add_speed", delta * 0.1, context);
            }
            _ => {
                panic!("Unknown effect index: '{}'", self.current_effect);
            }
        }
    }

    pub fn change_settings2(&mut self, delta: f32, context: &mut GMContext) {
        let effect = &mut self.effects[self.current_effect];

        match self.current_effect {
            1 => {
                effect.send_message_f32("add_offset", delta, context);
            }
            2 => {
                effect.send_message_f32("add_radius", delta, context);
            }
            3 => {
                effect.send_message_f32("add_offset", delta * 10.0, context);
            }
            4 => {
                effect.send_message_multiple_f32("send_message", 1, "add_speed", delta * 0.1, context);
            }
            5 => {
                effect.send_message_multiple_f32("send_message", 1, "add_speed", delta * 0.1, context);
            }
            _ => {
                panic!("Unknown effect index: '{}'", self.current_effect);
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
            self.current_effect = 0;
        }

        if context.event(GMEventCode::Key2Up) {
            self.current_effect = 1;
        }

        if context.event(GMEventCode::Key3Up) {
            self.current_effect = 2;
        }

        if context.event(GMEventCode::Key4Up) {
            self.current_effect = 3;
        }

        if context.event(GMEventCode::Key5Up) {
            self.current_effect = 4;
        }

        if context.event(GMEventCode::Key6Up) {
            self.current_effect = 5;
        }

        if context.event(GMEventCode::KeyUpUp) {
            self.change_settings1(0.1, context);
        }

        if context.event(GMEventCode::KeyDownUp) {
            self.change_settings1(-0.1, context);
        }

        if context.event(GMEventCode::KeyRightUp) {
            self.change_settings2(0.1, context);
        }

        if context.event(GMEventCode::KeyLeftUp) {
            self.change_settings2(-0.1, context);
        }

        self.texts[0].reset_chars2();
        self.effects[self.current_effect].update(&mut self.texts[0], context);

    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        for text in self.texts.iter() {
            text.draw(context);
        }

        self.effects[self.current_effect].draw(&self.texts[0], context);
    }
}

fn main() {
    let config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, config, File::create("text3.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration("config.json");
    engine.init();
    engine.load_resources("resources.json");

    let text3_scene = TextScene3::new(&engine);

    engine.add_scene("text3_scene", text3_scene);
    engine.run();
}
