

use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText, GMBitmapTextBuilder};
use green_moon_2d::bitmap_text_effects::{GMTEWave, GMTEShake, GMTERotateChars, GMTEScale};
use green_moon_2d::util::GMAlign;


#[derive(Debug)]
struct TextScene3 {
    texts: Vec<GMBitmapText>,
    effects: Vec<GMBitmapText>,
    current_effect: usize,
}

impl TextScene3 {
    pub fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();
        let window_width = engine.window_width();

        let font = resources.get_font("font_cuddly");

        const SPACE: f32 = 50.0;
        let mut texts = Vec::new();

        texts.push(GMBitmapTextBuilder::new(&font)
            .with_text("PRESS NUMBER TO CHANGE EFFECT")
            .with_position((32.0, 32.0 + (5.0 * SPACE)))
            .build());

        texts.push(GMBitmapTextBuilder::new(&font)
            .with_text("CURSOR TO CHANGE SETTING")
            .with_position((32.0, 32.0 + (6.0 * SPACE)))
            .build());

        let mut effects: Vec<GMBitmapText> = Vec::new();

        // Move title to the center of the window
        let mut effect = GMBitmapTextBuilder::new(&font)
            .with_text("TEXT TEST 3")
            .with_position((window_width / 2.0, 32.0 + (1.0 * SPACE)))
            .with_align(GMAlign::TopCenter)
            .build();

        effects.push(effect.clone());

        effect.effects.set_effects(vec![Box::new(GMTEWave::new(
            32.0, // amplitude
            0.1, // speed
            0.2 // offset
            ))]);
        effects.push(effect.clone());

        effect.effects.set_effects(vec![Box::new(GMTEShake::new(
            5.0, // radius
            0.2 // speed
            ))]);
        effects.push(effect.clone());

        effect.effects.set_effects(vec![Box::new(GMTERotateChars::new(
            1.0, // speed
            10.0 // offset
            ))]);
        effects.push(effect.clone());

        effect.effects.set_effects(vec![Box::new(GMTEScale::new(
            0.2, // amplitude
            1.0, // base
            0.1, // speed
            0.2, // offset
            ))]);
        effects.push(effect.clone());

        effect.effects.set_effects(vec![
            Box::new(GMTEWave::new(
                32.0, // amplitude
                0.1, // speed
                0.2 // offset
                )),
            Box::new(GMTEShake::new(
                5.0, // radius
                0.2 // offset
                ))]);
        effects.push(effect.clone());

        effect.effects.set_effects(vec![
            Box::new(GMTEWave::new(
                32.0, // amplitude
                0.1, // speed
                0.2 // offset
                )),
            Box::new(GMTERotateChars::new(
                1.0, // speed
                10.0 // offset
                ))]);
        effects.push(effect.clone());

        effect.effects.set_effects(vec![
            Box::new(GMTEWave::new(
                32.0, // amplitude
                0.1, // speed
                0.2 // offset
                )),
            Box::new(GMTEScale::new(
                0.2, // amplitude
                1.0, // base
                0.1, // speed
                0.2, // offset
                    ))]);
        effects.push(effect.clone());

        Self {
            texts,
            effects,
            current_effect: 0,
        }
    }

    pub fn change_settings1(&mut self, delta: f32, context: &mut GMContext) {
        let effect = &mut self.effects[self.current_effect];

        match self.current_effect {
            0 => {
                // Nothing to do
            }
            1 => {
                effect.effects.send_message(0, "add_speed", (delta * 0.1).into(), context);
            }
            2 => {
                effect.effects.send_message(0, "add_speed", (delta * 0.1).into(), context);
            }
            3 => {
                effect.effects.send_message(0, "add_speed", delta.into(), context);
            }
            4 => {
                effect.effects.send_message(0, "add_speed", (delta * 0.1).into(), context);
            }
            5 => {
                effect.effects.send_message(0, "add_speed", (delta * 0.1).into(), context);
            }
            6 => {
                effect.effects.send_message(0, "add_speed", (delta * 0.1).into(), context);
            }
            7 => {
                effect.effects.send_message(0, "add_speed", (delta * 0.1).into(), context);
            }
            _ => {
                panic!("Unknown effect index: '{}'", self.current_effect);
            }
        }
    }

    pub fn change_settings2(&mut self, delta: f32, context: &mut GMContext) {
        let effect = &mut self.effects[self.current_effect];

        match self.current_effect {
            0 => {
                // Nothing to do
            }
            1 => {
                effect.effects.send_message(0, "add_offset", delta.into(), context);
            }
            2 => {
                effect.effects.send_message(0, "add_radius", delta.into(), context);
            }
            3 => {
                effect.effects.send_message(0, "add_offset", (delta * 10.0).into(), context);
            }
            4 => {
                effect.effects.send_message(0, "add_offset", delta.into(), context);
            }
            5 => {
                effect.effects.send_message(1, "add_speed", (delta * 0.1).into(), context);
            }
            6 => {
                effect.effects.send_message(1, "add_speed", (delta * 0.1).into(), context);
            }
            7 => {
                effect.effects.send_message(1, "add_offset", delta.into(), context);
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

        if context.event(GMEventCode::Key7Up) {
            self.current_effect = 6;
        }

        if context.event(GMEventCode::Key8Up) {
            self.current_effect = 7;
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

        let text = &mut self.effects[self.current_effect];

        text.base.reset_chars2();
        text.update(context);

    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        for text in self.texts.iter() {
            text.draw(context);
        }

        self.effects[self.current_effect].draw(context);
    }
}

fn main() {
    let config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, config, File::create("text3.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let text3_scene = TextScene3::new(&engine);

    engine.add_scene("text3_scene", text3_scene);
    engine.run();
}
