use std::rc::Rc;

use crate::context::GMContext;
use crate::font::GMBitmapFont;
use crate::error::GMError;
use crate::screen_buffer::GMScreenBuffer;
use crate::resource_manager::GMName;

pub trait GMTextEffect {
    fn draw(&self, text_context: &GMTextContext, screen_buffer: &mut GMScreenBuffer) -> Result<(), GMError> {
        Ok(())
    }

    fn update(&mut self, text_context: &GMTextContext, context: &GMContext) {

    }
}

pub struct GMStaticTextH {

}

impl GMTextEffect for GMStaticTextH {
    fn draw(&self, text_context: &GMTextContext, screen_buffer: &mut GMScreenBuffer) -> Result<(), GMError>{
        let mut current_x = text_context.px;
        let current_y = text_context.py;
        let char_width = text_context.font.char_width;
        let font = text_context.get_font();

        for c in text_context.content.chars() {
            let font_bitmap = font.get_bitmap(c as u8);
            screen_buffer.blit_bitmap(font_bitmap, current_x, current_y);
            current_x += char_width;
        }

        Ok(())
    }
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
    fn draw(&self, text_context: &GMTextContext, screen_buffer: &mut GMScreenBuffer) -> Result<(), GMError> {
        self.effect.draw(text_context, screen_buffer)
    }

    fn update(&mut self, text_context: &GMTextContext, context: &GMContext) {
        self.effect.update(text_context, context)
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

pub struct GMTextContext {
    pub(crate) content: String,
    pub(crate) font: Rc<GMBitmapFont>,
    pub(crate) px: u32,
    pub(crate) py: u32,
}

impl GMTextContext {
    pub fn set_text(&mut self, content: &str) {
        self.content = content.to_string();
    }

    pub fn set_position(&mut self, px: u32, py: u32) {
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
    pub(crate) text_context: GMTextContext,
    pub(crate) text_effect: Rc<GMTextEffectWrapper>,
}

impl GMText {
    pub fn new(content: &str, font: Rc<GMBitmapFont>, px: u32, py: u32) -> GMText {
        let text_context = GMTextContext {
            content: content.to_string(),
            font,
            px,
            py,
        };

        let text_effect = GMTextEffectWrapper::new("static_h", GMStaticTextH{});

        GMText {
            text_context,
            text_effect: Rc::new(text_effect),
        }
    }

    pub fn set_text(&mut self, content: &str) {
        self.text_context.content = content.to_string();
    }

    pub fn set_position(&mut self, px: u32, py: u32) {
        self.text_context.px = px;
        self.text_context.py = py;
    }

    pub fn set_font(&mut self, font: Rc<GMBitmapFont>) {
        self.text_context.font = font;
    }

    pub fn set_text_effect(&mut self, text_effect: Rc<GMTextEffectWrapper>) {
        self.text_effect = text_effect;
    }

    pub fn get_text_effect(&self) -> Rc<GMTextEffectWrapper> {
        self.text_effect.clone()
    }

    pub fn get_context(&self) -> &GMTextContext {
        &self.text_context
    }
}
