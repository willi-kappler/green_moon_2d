use crate::error::GMError;

use std::collections::HashMap;
use std::rc::Rc;

use macroquad::texture::{Texture2D, draw_texture_ex, load_texture, DrawTextureParams};
use macroquad::color::colors;
use macroquad::math::Rect;

// TODO:
// - use GMFont instead of GMFontT
// 

pub trait GMFontT {
    fn draw(&self, c: char, x: f32, y: f32);
    fn get_extend(&self, c: char) -> (f32, f32);
}

#[derive(Clone)]
pub struct GMFont {
    font: Rc<dyn GMFontT>,
}

impl GMFont {
    pub fn new<T: 'static + GMFontT>(font: T) -> Self {
        Self {
            font: Rc::new(font),
        }
    }
    pub fn draw(&self, c: char, x: f32, y: f32) {
        self.font.draw(c, x, y);
    }
    pub fn get_extend(&self, c: char) -> (f32, f32) {
        self.font.get_extend(c)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMBitmapFont {
    data: Texture2D,
    mapping: HashMap<char, Rect>,
}

impl GMBitmapFont {
    pub async fn new(file_name: &str) -> Result<Self, GMError> {
        let data = load_texture(file_name).await?;

        let font = Self {
            data,
            mapping: HashMap::new(),
        };

        Ok(font)
    }
    pub async fn new_font(file_name: &str, char_width: f32, char_height: f32, char_order: &str) -> Result<GMFont, GMError> {
        let mut font = Self::new(file_name).await?;
        font.set_mapping_fixed(char_width, char_height, char_order);
        Ok(GMFont::new(font))
    }
    pub fn set_mapping(&mut self, mapping: HashMap<char, Rect>) {
        self.mapping = mapping;
    }
    pub fn set_mapping_fixed(&mut self, char_width: f32, char_height: f32, char_order: &str) {
        let mut current_x: f32 = 0.0;
        let mut current_y: f32 = 0.0;
        let mut mapping: HashMap<char, Rect> = HashMap::new();

        for c in char_order.chars() {
            mapping.insert(c, Rect::new(current_x, current_y, char_width, char_height));

            current_x += char_width;
            if current_x >= self.data.width() {
                current_x = 0.0;
                current_y += char_height;
            }
        }

        self.mapping = mapping;
    }
}

impl GMFontT for GMBitmapFont {
    fn draw(&self, c: char, x: f32, y: f32) {
        let rect = self.mapping[&c];
        let source = Some(rect);
        let params = DrawTextureParams {
            source, .. Default::default()
        };

        draw_texture_ex(self.data, x, y, colors::WHITE, params);
    }
    fn get_extend(&self, c: char) -> (f32, f32) {
        let rect = self.mapping[&c];
        (rect.w, rect.h)
    }
}
