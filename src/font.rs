use crate::error::GMError;

use std::collections::HashMap;

use macroquad::texture::{Texture2D, draw_texture_ex, load_texture, DrawTextureParams};
use macroquad::color::colors;
use macroquad::math::Rect;

pub trait GMFontT {
    fn draw(&self, c: char, x: f32, y: f32);
    fn get_extend(&self, c: char) -> (f32, f32);
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMBitmapFont {
    pub(crate) data: Texture2D,
    pub(crate) mapping: HashMap<char, Rect>,
    pub(crate) char_width: f32,
    pub(crate) char_height: f32,
}

impl GMBitmapFont {
    pub async fn new(file_name: &str, char_width: f32, char_height: f32, char_order: &str) -> Result<Self, GMError> {
        let data = load_texture(file_name).await?;
        let mut mapping: HashMap<char, Rect> = HashMap::new();

        let mut current_x: f32 = 0.0;
        let mut current_y: f32 = 0.0;

        for c in char_order.chars() {
            mapping.insert(c, Rect::new(current_x, current_y, char_width, char_height));

            current_x += char_width;
            if current_x > data.width() {
                current_x = 0.0;
                current_y += char_height;
            }
        }

        let font = Self {
            data,
            mapping,
            char_width,
            char_height,
        };

        Ok(font)
    }

}

impl GMFontT for GMBitmapFont {
    fn draw(&self, c: char, x: f32, y: f32) {
        let rect = self.mapping[&c];
        let source = Some(rect);
        let params = DrawTextureParams {
            source, .. Default::default()
        };

        draw_texture_ex(self.data, x, y, colors::BLANK, params);
    }
    fn get_extend(&self, _c: char) -> (f32, f32) {
        (self.char_width, self.char_height)
    }
}
