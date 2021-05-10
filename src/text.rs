use std::rc::Rc;

use crate::context::GMContext;
use crate::font::GMBitmapFont;
use crate::error::GMError;
use crate::screen_buffer::GMScreenBuffer;

pub trait GMTextEffect {
    fn draw(&self, text_context: &GMTextContext, font: Rc<GMBitmapFont>, screen_buffer: &mut GMScreenBuffer) -> Result<(), GMError> {
        Ok(())
    }

    fn update(&mut self, text_context: &GMTextContext, context: &GMContext) {

    }
}

pub struct GMStaticTextH {

}

impl GMTextEffect for GMStaticTextH {
    fn draw(&self, text_context: &GMTextContext, font: Rc<GMBitmapFont>, screen_buffer: &mut GMScreenBuffer) -> Result<(), GMError>{
        let mut current_x = text_context.px;
        let current_y = text_context.py;
        let char_width = font.get_char_width();

        for c in text_context.content.chars() {
            let font_bitmap = font.get_bitmap(c);
            screen_buffer.blit_bitmap(font_bitmap, current_x, current_y);
            current_x += char_width;
        }

        Ok(())
    }
}

pub struct GMTextEffectWrapper {
    name: String,
    effect: Box<dyn GMTextEffect>,
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
    fn draw(&self, text_context: &GMTextContext, font: Rc<GMBitmapFont>, screen_buffer: &mut GMScreenBuffer) -> Result<(), GMError> {
        self.effect.draw(text_context, font, screen_buffer)
    }

    fn update(&mut self, text_context: &GMTextContext, context: &GMContext) {
        self.effect.update(text_context, context)
    }
}

pub struct GMTextEffectManager {
    effects: Vec<GMTextEffectWrapper>,
}

impl GMTextEffectManager {
    pub fn new() -> GMTextEffectManager {
        let static_text_h = GMTextEffectWrapper::new("static_h", GMStaticTextH{});

        GMTextEffectManager {
            effects: vec![static_text_h]
        }
    }

    pub fn get_text_effect(&self, name: &str) -> Result<&GMTextEffectWrapper, GMError> {
        for effect in self.effects.iter() {
            if name == effect.name {
                return Ok(effect)
            }
        }

        Err(GMError::TextEffectNotFound(name.to_string()))
    }
}

pub struct GMTextContext {
    content: String,
    font: String,
    px: u32,
    py: u32,
}

impl GMTextContext {
    pub fn set_text(&mut self, content: &str) {
        self.content = content.to_string();
    }

    pub fn set_position(&mut self, px: u32, py: u32) {
        self.px = px;
        self.py = py;
    }

    pub fn set_font_name(&mut self, font: &str) {
        self.font = font.to_string();
    }
}

pub struct GMText {
    text_context: GMTextContext,
    text_effect: String,
}

impl GMText {
    pub fn new(content: &str, font: &str, px: u32, py: u32) -> GMText {
        let text_context = GMTextContext {
            content: content.to_string(),
            font: font.to_string(),
            px,
            py,
        };

        GMText {
            text_context,
            text_effect: "static_h".to_string(),
        }
    }

    pub fn set_text(&mut self, content: &str) {
        self.text_context.set_text(content);
    }

    pub fn set_position(&mut self, px: u32, py: u32) {
        self.text_context.set_position(px, py);
    }

    pub fn set_font_name(&mut self, font: &str) {
        self.text_context.set_font_name(font);
    }

    pub fn get_font_name(&self) -> &str {
        &self.text_context.font
    }

    pub fn get_text_effect(&self) -> &str {
        &self.text_effect
    }

    pub fn get_context(&self) -> &GMTextContext {
        &self.text_context
    }
}
