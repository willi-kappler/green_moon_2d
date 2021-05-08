

use crate::context::{GMContext};

pub trait GMTextEffect {
    fn draw(&mut self, text_context: &GMTextContext, context: &mut GMContext) {

    }

    fn update(&mut self, text_context: &GMTextContext, context: &mut GMContext) {

    }
}

pub struct GMStaticText {

}

impl GMTextEffect for GMStaticText {
    fn draw(&mut self, text_context: &GMTextContext, context: &mut GMContext) {
        let font = context.get_font_by_name(text_context.get_font());
    }
}


pub struct GMTextContext {
    content: String,
    font: String,
    px: f32,
    py: f32,
}

impl GMTextContext {
    fn set_text(&mut self, content: &str) {
        self.content = content.to_string();
    }

    fn set_position(&mut self, px: f32, py: f32) {
        self.px = px;
        self.py = py;
    }

    fn set_font(&mut self, font: &str) {
        self.font = font.to_string();
    }

    fn get_font(&self) -> &str {
        &self.font
    }
}

pub struct GMText {
    text_context: GMTextContext,
    text_effect: Box<dyn GMTextEffect>,
}

impl GMText {
    pub fn new(content: &str, font: &str, px: f32, py: f32) -> GMText {
        let text_context = GMTextContext {
            content: content.to_string(),
            font: font.to_string(),
            px,
            py,
        };

        let text_effect: Box<dyn GMTextEffect> = Box::new(GMStaticText{});

        GMText {
            text_context,
            text_effect,
        }
    }

    pub fn set_text(&mut self, content: &str) {
        self.text_context.set_text(content);
    }

    pub fn set_position(&mut self, px: f32, py: f32) {
        self.text_context.set_position(px, py);
    }

    pub fn set_font(&mut self, font: &str) {
        self.text_context.set_font(font);
    }

    pub fn draw(&mut self, context: &mut GMContext) {
        self.text_effect.draw(&self.text_context, context);
    }

    pub fn update(&mut self, context: &mut GMContext) {
        self.text_effect.update(&self.text_context, context);
    }
}
