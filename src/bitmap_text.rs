

use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Debug;
use std::any::Any;

use log::error;

use crate::texture::GMTexture;
use crate::context::GMContext;
use crate::math::GMVec2D;
use crate::util::error_panic;

#[derive(Debug, Clone)]
pub struct GMBitmapFont {
    texture: Rc<GMTexture>,
    mapping: HashMap<char, u32>,
}

impl GMBitmapFont {
    pub fn new(texture: Rc<GMTexture>, char_mapping: &str) -> Self {
        let mut mapping = HashMap::new();

        for (i, c) in char_mapping.chars().enumerate() {
            mapping.insert(c, i as u32);
        }

        Self {
            texture,
            mapping,
        }
    }

    pub fn get_char_dimensions(&self) -> (f32, f32) {
        self.texture.get_unit_dimension()
    }

    pub fn draw(&self, c: char, x: f32, y: f32, context: &mut GMContext) {
        self.draw_opt(c, x, y, 0.0, false, false, context);
    }

    pub fn draw_opt(&self, c: char, x: f32, y: f32, angle: f32, flip_x: bool, flip_y: bool, context: &mut GMContext) {
        match self.mapping.get(&c) {
            Some(index) => {
                self.texture.draw_opt(x, y, *index, angle, flip_x, flip_y, context);
            }
            None => {
                error_panic(&format!("GMBitmapFont::draw_opt(), Character '{}' not in map.", c));
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct GMBitmapText {
    pub font: Rc<GMBitmapFont>,
    pub text: String,
    pub position: GMVec2D,
    pub spacing_x: f32,
    pub spacing_y: f32,
    pub horizontal: bool,
}

impl GMBitmapText {
    pub fn new(font: Rc<GMBitmapFont>, text: &str, x: f32, y: f32) -> Self {
        Self {
            font,
            text: text.to_string(),
            position: GMVec2D::new(x, y),
            spacing_x: 0.0,
            spacing_y: 0.0,
            horizontal: true,
        }
    }

    pub fn new2(font: &str, text: &str, x: f32, y: f32, context: &GMContext) -> Self {
        Self::new(context.resources.get_font_clone(font), text, x, y)
    }
}

pub trait GMTextEffect {
    fn update(&mut self, _context: &mut GMContext) {
        error_panic("GMTextEffect::update() not implemented");
    }

    fn draw(&self, _context: &mut GMContext) {
        error_panic("GMTextEffect::draw() not implemented");
    }

    fn send_message(&mut self, _message: &str, _data: Option<Box<dyn Any>>, _context: &mut GMContext) {
        error_panic("GMTextEffect::send_message() not implemented");
    }
}

// TODO: Add text effects

