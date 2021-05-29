use crate::font::GMBitmapFont;
use crate::text::GMText;

use std::f32::consts;

pub type GMTextEffect = Box<dyn GMTextEffectT>;
pub trait GMTextEffectT {
    fn draw(&self, text: &GMText, font: &GMBitmapFont);

    fn update(&mut self);

    fn get_extend(&self, text: &GMText, font: &GMBitmapFont) -> (f32, f32);
}

pub struct GMStaticText {}

impl GMStaticText {
    pub fn new() -> GMStaticText {
        GMStaticText{}
    }
}

impl GMTextEffectT for GMStaticText {
    fn draw(&self, text: &GMText, font: &GMBitmapFont) {
        let mut current_x = text.px;
        let mut current_y = text.py;

        for c in text.content.chars() {
            let (offset_x, offset_y) = font.draw_char(c, current_x, current_y);
            if text.horizontal {
                current_x += offset_x;
            } else {
                current_y += offset_y;
            }
        }
    }

    fn update(&mut self) {

    }

    fn get_extend(&self, text: &GMText, font: &GMBitmapFont) -> (f32, f32) {
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for c in text.content.chars() {
            let (extend_x, extend_y) = font.get_extend(c);
            if text.horizontal {
                max_width += extend_x;
                max_height = max_height.max(extend_y);
            } else {
                max_height += extend_y;
                max_width = max_width.max(extend_x);
            }
        }

        (max_width, max_height)
    }
}

pub struct GMWave {
    pub(crate) phase: f32,
    pub(crate) amplitude: f32,
    pub(crate) frequency: f32,
}

impl GMWave {
    pub fn new(amplitude: f32, frequency: f32) -> GMWave {
        GMWave {
            phase: 0.0,
            amplitude,
            frequency,
        }
    }
}

impl GMTextEffectT for GMWave {
    fn draw(&self, text: &GMText, font: &GMBitmapFont) {
        let mut current_x = text.px;

        for (c, i) in text.content.chars().zip(0..) {
            let phase = self.phase + (i as f32 * self.frequency);
            let current_y = text.py + (phase.sin() * self.amplitude);
            let (offset_x, _) = font.draw_char(c, current_x, current_y);
            current_x += offset_x;
        }
    }

    fn update(&mut self) {
        self.phase += self.frequency;

        const LIMIT: f32 = 2.0 * consts::PI;
        if self.phase > LIMIT {
            self.phase -= LIMIT;
        }
    }

    fn get_extend(&self, text: &GMText, font: &GMBitmapFont) -> (f32, f32) {
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for c in text.content.chars() {
            let (extend_x, extend_y) = font.get_extend(c);
            max_width += extend_x;
            max_height = max_height.max(extend_y);
        }

        max_height += 2.0 * self.amplitude;

        (max_width, max_height)
    }
}

pub struct GMRotZ {
    pub(crate) phase: f32,
    pub(crate) amplitudes: Vec<f32>,
    pub(crate) frequency: f32,
}

impl GMRotZ {
    pub fn new(frequency: f32, text: &GMText, font: &GMBitmapFont) -> GMRotZ {
        let mut amplitudes = Vec::new();

        let mut max_width: f32 = 0.0;

        for c in text.content.chars() {
            amplitudes.push(max_width);
            let (extend_x, _) = font.get_extend(c);
            max_width += extend_x;
        }

        let half_width = max_width / 2.0;
        for v in amplitudes.iter_mut() {
            *v -= half_width;
        }

        GMRotZ {
            phase: 0.0,
            amplitudes,
            frequency,
        }
    }
}

impl GMTextEffectT for GMRotZ {
    fn draw(&self, text: &GMText, font: &GMBitmapFont) {
        let current_y = text.py;

        for (c, amplitude) in text.content.chars().zip(&self.amplitudes) {
            let current_x = text.px + (self.phase.sin() * amplitude);
            let _ = font.draw_char(c, current_x, current_y);
        }
    }

    fn update(&mut self) {
        self.phase += self.frequency;

        const LIMIT: f32 = 2.0 * consts::PI;
        if self.phase > LIMIT {
            self.phase -= LIMIT;
        }
    }

    fn get_extend(&self, text: &GMText, font: &GMBitmapFont) -> (f32, f32) {
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for c in text.content.chars() {
            let (extend_x, extend_y) = font.get_extend(c);
            max_width += extend_x;
            max_height = max_height.max(extend_y);
        }

        (max_width, max_height)
    }
}

pub struct GMSelected1 {
    // TODO:
}

impl GMSelected1 {
    pub fn new() -> GMSelected1 {
        GMSelected1{}
    }
}

impl GMTextEffectT for GMSelected1 {
    fn draw(&self, text: &GMText, font: &GMBitmapFont) {
        // TODO:
    }

    fn update(&mut self) {
        // TODO:
    }

    fn get_extend(&self, text: &GMText, font: &GMBitmapFont) -> (f32, f32) {
        // TODO:
        todo!()
    }
}
