
use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::bitmap_text_effects::{GMTEWave, GMTEShake, GMTERotateChars, GMTEScale};
use green_moon_2d::util::{GMDrawT, GMAlign};

#[derive(Debug)]
enum CurrentEffect {
    SineWave,
    Shake,
    Rotate,
    Scale,
    SineWaveRotate,
    ShakeScale,
}

#[derive(Debug)]
struct TextScene3 {
    title: GMBitmapText,
    descriptions: Vec<GMBitmapText>,
    effect_name: GMBitmapText,
    current_effect: CurrentEffect,
    sine_effect: GMTEWave,
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

        let sine_effect = GMTEWave::new(
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

        let current_effect = CurrentEffect::SineWave;

        Self {
            title,
            descriptions,
            effect_name,
            sine_effect,
            shake_effect,
            current_effect,
            rotate_effect,
            scale_effect,
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
            self.current_effect = CurrentEffect::SineWave;
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
            self.current_effect = CurrentEffect::SineWaveRotate;
        }

        if context.event(GMEventCode::Key6Up) {
            self.effect_name.set_text("--- SHAKE AND SCALE ---");
            self.current_effect = CurrentEffect::ShakeScale;
        }

        self.effect_name.reset_chars();

        match self.current_effect {
            CurrentEffect::SineWave => {
                self.sine_effect.update(&mut self.effect_name, context);
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
            CurrentEffect::SineWaveRotate => {
                self.sine_effect.update(&mut self.effect_name, context);
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
