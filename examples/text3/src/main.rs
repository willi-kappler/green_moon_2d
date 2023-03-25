
use std::fs::File;

use log::debug;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::bitmap_text_effects::{GMTEWave, GMTEShake, GMTERotateChars, GMTEScale};
use green_moon_2d::util::{GMDrawT, GMAlign};

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
    title: GMBitmapText,
    descriptions: Vec<GMBitmapText>,
    effect_name: GMBitmapText,
    current_effect: CurrentEffect,
    wave_effect: GMTEWave,
    shake_effect: GMTEShake,
    rotate_effect: GMTERotateChars,
    scale_effect: GMTEScale,
}

impl TextScene3 {
    fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();

        // Get font
        let font1 = resources.get_font("font_cuddly");

        // Set up title text:
        let mut title = GMBitmapText::new(font1, (512.0, 100.0), "TEXT 3");
        title.set_align2(GMAlign::BottomCenter);

        // Set up description:
        let mut descriptions = Vec::new();
        let mut text = GMBitmapText::new(font1, (512.0, 200.0), "PRESS NUMBERS");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        let mut text = GMBitmapText::new(font1, (512.0, 250.0), "TO CHANGE EFFECT");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        let mut text = GMBitmapText::new(font1, (512.0, 350.0), "PRESS CURSOR KEYS");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        let mut text = GMBitmapText::new(font1, (512.0, 400.0), "TO CHANGE VALUES");
        text.set_align2(GMAlign::BottomCenter);
        descriptions.push(text);

        let mut effect_name = GMBitmapText::new(font1, (512.0, 500.0), "---<<< SINE WAVE >>>---");
        effect_name.set_align2(GMAlign::BottomCenter);

        let wave_effect = GMTEWave::new(
            32.0, // amplitude
            0.1, // speed
            0.2 // offset
        );

        let shake_effect = GMTEShake::new(
            5.0, // radius
            0.2 // speed
        );

        let rotate_effect = GMTERotateChars::new(
            1.0, // speed
            10.0 // offset
        );

        let scale_effect = GMTEScale::new(
            0.2, // amplitude
            1.0, // base
            0.1, // speed
            0.2, // offset
        );

        let current_effect = CurrentEffect::Wave;

        Self {
            title,
            descriptions,
            effect_name,
            wave_effect,
            shake_effect,
            current_effect,
            rotate_effect,
            scale_effect,
        }
    }

    fn change_property1(&mut self, amount: f32) {
        debug!("Change property1: {}", amount);

        match self.current_effect {
            CurrentEffect::Wave => {
                self.wave_effect.inc_offset(0.05 * amount);
            }
            CurrentEffect::Shake => {
                self.shake_effect.inc_radius(0.1 * amount);
            }
            CurrentEffect::Rotate => {
                self.rotate_effect.inc_speed(0.1 * amount);
            }
            CurrentEffect::Scale => {
                self.scale_effect.inc_amplitude(0.01 * amount);
            }
            CurrentEffect::WaveRotate => {
                self.wave_effect.inc_amplitude(0.1 * amount);
            }
            CurrentEffect::ShakeScale => {
                self.shake_effect.inc_radius(0.1 * amount);
            }
        }
    }

    fn change_property2(&mut self, amount: f32) {
        debug!("Change property2: {}", amount);

        match self.current_effect {
            CurrentEffect::Wave => {
                self.wave_effect.inc_speed(0.01 * amount);
            }
            CurrentEffect::Shake => {
                self.shake_effect.inc_speed(0.01 * amount);
            }
            CurrentEffect::Rotate => {
                self.rotate_effect.inc_offset(0.1 * amount);
            }
            CurrentEffect::Scale => {
                self.scale_effect.inc_speed(0.01 * amount);
            }
            CurrentEffect::WaveRotate => {
                self.rotate_effect.inc_speed(0.1 * amount);
            }
            CurrentEffect::ShakeScale => {
                self.scale_effect.inc_speed(0.01 * amount);
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
            self.effect_name.set_text("---<<< SINE WAVE >>>---");
            self.current_effect = CurrentEffect::Wave;
        }

        if context.event(GMEventCode::Key2Up) {
            self.effect_name.set_text("..... SHAKE .....");
            self.current_effect = CurrentEffect::Shake;
        }

        if context.event(GMEventCode::Key3Up) {
            self.effect_name.set_text(">>>>> ROTATE <<<<<");
            self.current_effect = CurrentEffect::Rotate;
        }

        if context.event(GMEventCode::Key4Up) {
            self.effect_name.set_text("--<>() SCALE ()<>--");
            self.current_effect = CurrentEffect::Scale;
        }

        if context.event(GMEventCode::Key5Up) {
            self.effect_name.set_text("--- SINE WAVE ROTATE ---");
            self.current_effect = CurrentEffect::WaveRotate;
        }

        if context.event(GMEventCode::Key6Up) {
            self.effect_name.set_text("--- SHAKE AND SCALE ---");
            self.current_effect = CurrentEffect::ShakeScale;
        }

        if context.event(GMEventCode::KeyUpUp) {
            self.change_property1(1.0);
        }

        if context.event(GMEventCode::KeyDownUp) {
            self.change_property1(-1.0);
        }

        if context.event(GMEventCode::KeyRightUp) {
            self.change_property2(1.0);
        }

        if context.event(GMEventCode::KeyLeftUp) {
            self.change_property2(-1.0);
        }

        self.effect_name.reset_chars();

        match self.current_effect {
            CurrentEffect::Wave => {
                self.wave_effect.update(&mut self.effect_name, context);
            }
            CurrentEffect::Shake => {
                self.shake_effect.update(&mut self.effect_name, context);
            }
            CurrentEffect::Rotate => {
                self.rotate_effect.update(&mut self.effect_name, context);
            }
            CurrentEffect::Scale => {
                self.scale_effect.update(&mut self.effect_name, context);
            }
            CurrentEffect::WaveRotate => {
                self.wave_effect.update(&mut self.effect_name, context);
                self.rotate_effect.update(&mut self.effect_name, context);
            }
            CurrentEffect::ShakeScale => {
                self.shake_effect.update(&mut self.effect_name, context);
                self.scale_effect.update(&mut self.effect_name, context);
            }
        }
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.title.draw(context);

        for text in self.descriptions.iter() {
            text.draw(context);
        }

        self.effect_name.draw(context);
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
