use std::rc::Rc;
use std::cell::RefCell;

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
    pub(crate) text_effect: Rc<RefCell<GMTextEffectWrapper>>,
}

impl GMText {
    pub fn new(name: &str, content: &str, font: Rc<GMBitmapFont>, px: f32, py: f32) -> GMText {
        let text_context = GMTextContext {
            content: content.to_string(),
            font,
            px,
            py,
        };

        let text_effect = GMTextEffectWrapper::new("static_h", GMStaticText{});

        GMText {
            name: name.to_string(),
            text_context,
            text_effect: Rc::new(RefCell::new(text_effect)),
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

    pub fn set_text_effect(&mut self, text_effect: Rc<RefCell<GMTextEffectWrapper>>) {
        self.text_effect = text_effect;
    }

    pub fn get_text_effect(&self) -> Rc<RefCell<GMTextEffectWrapper>> {
        self.text_effect.clone()
    }

    pub fn get_context(&self) -> &GMTextContext {
        &self.text_context
    }

    pub fn draw(&self) {
        let text_effect = self.text_effect.borrow();
        text_effect.draw(&self.text_context);
    }

    pub fn update(&mut self) {
        let mut text_effect = self.text_effect.borrow_mut();
        text_effect.update(&self.text_context);
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
        let mut current_y = text_context.py;
        let font = text_context.get_font();

        for c in text_context.content.chars() {
            let (offset_x, _) = font.draw_char(c, current_x, current_y);
            current_x += offset_x;
        }
    }
}

pub struct GMStaticTextV {}

impl GMTextEffect for GMStaticTextV {
    fn draw(&self, text_context: &GMTextContext) {
        let mut current_x = text_context.px;
        let mut current_y = text_context.py;
        let font = text_context.get_font();

        for c in text_context.content.chars() {
            let (_, offset_y) = font.draw_char(c, current_x, current_y);
            current_y += offset_y;
        }
    }
}
