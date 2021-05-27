use std::rc::Rc;
use std::cell::RefCell;
use std::f32::consts;

use crate::font::GMBitmapFont;
use crate::resource_manager::GMName;

pub struct GMTextContext {
    pub(crate) content: String,
    pub(crate) font: Rc<GMBitmapFont>,
    pub(crate) px: f32,
    pub(crate) py: f32,
}

impl GMTextContext {
    pub fn set_text(&mut self, content: &str) {
        self.content = content.to_string();
    }

    pub fn set_position(&mut self, px: f32, py: f32) {
        self.px = px;
        self.py = py;
    }

    pub fn set_font(&mut self, font: Rc<GMBitmapFont>) {
        self.font = font;
    }

    pub fn get_font(&self) -> Rc<GMBitmapFont> {
        self.font.clone()
    }
}

pub struct GMText {
    pub(crate) name: String,
    pub(crate) text_context: GMTextContext,
    pub(crate) text_effect: GMTextEffectWrapper,
}

impl GMText {
    pub fn new(name: &str, content: &str, font: Rc<GMBitmapFont>, px: f32, py: f32) -> GMText {
        let text_context = GMTextContext {
            content: content.to_string(),
            font,
            px,
            py,
        };

        let text_effect = GMTextEffectWrapper::new("static_h", GMStaticTextH{});

        GMText {
            name: name.to_string(),
            text_context,
            text_effect: text_effect,
        }
    }

    pub fn set_text(&mut self, content: &str) {
        self.text_context.content = content.to_string();
    }

    pub fn set_position(&mut self, px: f32, py: f32) {
        self.text_context.px = px;
        self.text_context.py = py;
    }

    pub fn set_font(&mut self, font: Rc<GMBitmapFont>) {
        self.text_context.font = font;
    }

    pub fn set_text_effect(&mut self, text_effect: GMTextEffectWrapper) {
        self.text_effect = text_effect;
    }

    pub fn get_text_effect(&self) -> &GMTextEffectWrapper {
        &self.text_effect
    }

    pub fn get_context(&self) -> &GMTextContext {
        &self.text_context
    }

    pub fn get_extend(&self) -> (f32, f32) {
        self.text_effect.get_extend(&self.text_context)
    }

    pub fn draw(&self) {
        self.text_effect.draw(&self.text_context);
    }

    pub fn update(&mut self) {
        self.text_effect.update(&self.text_context);
    }
}

impl GMName for GMText {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }

    fn has_name(&self, name: &str) -> bool {
        self.name == name
    }

    fn has_prefix(&self, name: &str) -> bool {
        self.name.starts_with(name)
    }
}

pub trait GMTextEffect {
    fn draw(&self, text_context: &GMTextContext) {

    }

    fn update(&mut self, text_context: &GMTextContext) {

    }

    fn get_extend(&self, text_context: &GMTextContext) -> (f32, f32);
}

pub struct GMTextEffectWrapper {
    pub(crate) name: String,
    pub(crate) effect: Box<dyn GMTextEffect>,
}

impl GMTextEffectWrapper {
    pub fn new<T: 'static + GMTextEffect>(name: &str, effect: T) -> GMTextEffectWrapper {
        GMTextEffectWrapper {
            name: name.to_string(),
            effect: Box::new(effect),
        }
    }
}

impl GMTextEffect for GMTextEffectWrapper {
    fn draw(&self, text_context: &GMTextContext) {
        self.effect.draw(text_context)
    }

    fn update(&mut self, text_context: &GMTextContext) {
        self.effect.update(text_context)
    }

    fn get_extend(&self, text_context: &GMTextContext) -> (f32, f32) {
        self.effect.get_extend(text_context)
    }
}

impl GMName for GMTextEffectWrapper {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn has_name(&self, name: &str) -> bool {
        self.name == name
    }

    fn has_prefix(&self, name: &str) -> bool {
        self.name.starts_with(name)
    }
}

pub struct GMStaticTextH {}

impl GMTextEffect for GMStaticTextH {
    fn draw(&self, text_context: &GMTextContext) {
        let mut current_x = text_context.px;
        let current_y = text_context.py;
        let font = text_context.get_font();

        for c in text_context.content.chars() {
            let (offset_x, _) = font.draw_char(c, current_x, current_y);
            current_x += offset_x;
        }
    }

    fn get_extend(&self, text_context: &GMTextContext) -> (f32, f32) {
        let font = text_context.get_font();
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for c in text_context.content.chars() {
            let (extend_x, extend_y) = font.get_extend(c);
            max_width += extend_x;
            max_height = max_height.max(extend_y);
        }

        (max_width, max_height)
    }
}

pub struct GMStaticTextV {}

impl GMTextEffect for GMStaticTextV {
    fn draw(&self, text_context: &GMTextContext) {
        let current_x = text_context.px;
        let mut current_y = text_context.py;
        let font = text_context.get_font();

        for c in text_context.content.chars() {
            let (_, offset_y) = font.draw_char(c, current_x, current_y);
            current_y += offset_y;
        }
    }

    fn get_extend(&self, text_context: &GMTextContext) -> (f32, f32) {
        let font = text_context.get_font();
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for c in text_context.content.chars() {
            let (extend_x, extend_y) = font.get_extend(c);
            max_height += extend_y;
            max_width = max_width.max(extend_x);
        }

        (max_width, max_height)
    }
}

pub struct GMWaveH {
    pub(crate) phase: f32,
    pub(crate) amplitude: f32,
    pub(crate) frequency: f32,
}

impl GMWaveH {
    pub fn new(amplitude: f32, frequency: f32) -> GMWaveH {
        GMWaveH {
            phase: 0.0,
            amplitude,
            frequency,
        }
    }
}

impl GMTextEffect for GMWaveH {
    fn draw(&self, text_context: &GMTextContext) {
        let mut current_x = text_context.px;
        let font = text_context.get_font();

        for (c, i) in text_context.content.chars().zip(0..) {
            let phase = self.phase + (i as f32 * self.frequency);
            let current_y = text_context.py + (phase.sin() * self.amplitude);
            let (offset_x, _) = font.draw_char(c, current_x, current_y);
            current_x += offset_x;
        }
    }

    fn update(&mut self, _text_context: &GMTextContext) {
        self.phase += self.frequency;

        const LIMIT: f32 = 2.0 * consts::PI;
        if self.phase > LIMIT {
            self.phase -= LIMIT;
        }
    }

    fn get_extend(&self, text_context: &GMTextContext) -> (f32, f32) {
        let font = text_context.get_font();
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for c in text_context.content.chars() {
            let (extend_x, extend_y) = font.get_extend(c);
            max_width += extend_x;
            max_height = max_height.max(extend_y);
        }

        max_height += 2.0 * self.amplitude;

        (max_width, max_height)
    }
}
