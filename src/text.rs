use crate::context::GMContext;
use crate::error::GMError;

pub trait GMTextEffect {
    fn draw(&mut self, text_context: &GMTextContext, context: &mut GMContext) -> Result<(), GMError> {
        Ok(())
    }

    fn update(&mut self, text_context: &GMTextContext, context: &mut GMContext) {

    }
}

pub struct GMStaticTextH {

}

impl GMTextEffect for GMStaticTextH {
    fn draw(&mut self, text_context: &GMTextContext, context: &mut GMContext) -> Result<(), GMError>{
        let font = context.get_font_by_name(&text_context.font)?;
        //let char_data: Vec<(u32, u32, &[u8])> = Vec::new();
        let mut current_x = text_context.px;
        let current_y = text_context.py;
        let char_width = font.get_char_width();

        for c in text_context.content.chars() {
            let font_bitmap = font.get_bitmap(c);
            //char_data.push((current_x, current_y, font_bitmap));
            context.blit_bitmap(font_bitmap, current_x, current_y);
            current_x += char_width;
        }


        Ok(())
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
    text_effect: Box<dyn GMTextEffect>,
}

impl GMText {
    pub fn new(content: &str, font: &str, px: u32, py: u32) -> GMText {
        let text_context = GMTextContext {
            content: content.to_string(),
            font: font.to_string(),
            px,
            py,
        };

        let text_effect: Box<dyn GMTextEffect> = Box::new(GMStaticTextH{});

        GMText {
            text_context,
            text_effect,
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

    pub fn draw(&mut self, context: &mut GMContext) -> Result<(), GMError>{
        self.text_effect.draw(&self.text_context, context)
    }

    pub fn update(&mut self, context: &mut GMContext) {
        self.text_effect.update(&self.text_context, context);
    }
}
