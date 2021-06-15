use crate::error::GMError;

use macroquad::texture::{Texture2D, draw_texture_ex, load_texture, DrawTextureParams};
use macroquad::color::colors;
use macroquad::math::Rect;

use std::rc::Rc;

pub struct GMSpriteSheet {
    data: Texture2D,
}

impl GMSpriteSheet {
    pub async fn new(file_name: &str) -> Result<Self, GMError> {
        let data = load_texture(file_name).await?;

        let sprite_sheet = Self {
            data,
        };

        Ok(sprite_sheet)
    }
    pub async fn new_rc(file_name: &str) -> Result<Rc<Self>, GMError> {
        let sheet = Self::new(file_name).await?;
        Ok(Rc::new(sheet))
    }
    pub fn draw(&self, source: &Rect, x: f32, y: f32) {
        self.draw_mirror(source, x, y, false, false)
    }
    pub fn draw_mirror(&self, source: &Rect, x: f32, y: f32, flip_x: bool, flip_y: bool) {
        let params = DrawTextureParams {
            source: Some(*source),
            flip_x,
            flip_y,
            .. Default::default()
        };

        draw_texture_ex(self.data, x, y, colors::WHITE, params);
    }
}
