use crate::error::GMError;

use macroquad::texture::{Texture2D, draw_texture_ex, load_texture, DrawTextureParams};
use macroquad::color::colors;
use macroquad::math::Rect;

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
    pub fn draw(&self, source: &Rect, x: f32, y: f32) {
        let params = DrawTextureParams {
            source: Some(*source),
            .. Default::default()
        };

        draw_texture_ex(self.data, x, y, colors::WHITE, params);
    }
}
