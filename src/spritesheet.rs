use crate::resource_manager::GMName;
use crate::error::GMError;

use macroquad::texture::{Texture2D, draw_texture_ex, load_texture, DrawTextureParams};
use macroquad::color::colors;
use macroquad::math::Rect;

#[derive(Clone, Debug, PartialEq)]
pub struct GMSpriteSheet {
    name: String,
    data: Texture2D,
}

impl GMSpriteSheet {
    pub async fn new(name: &str, file_name: &str) -> Result<GMSpriteSheet, GMError> {
        let data = load_texture(file_name).await?;

        let sprite_sheet = GMSpriteSheet {
            name: name.to_string(),
            data,
        };

        Ok(sprite_sheet)
    }

    pub fn draw(&self, source: &Rect, x: f32, y: f32) {
        let params = DrawTextureParams {
            source: Some(*source),
            .. Default::default()
        };

        draw_texture_ex(self.data, x, y, colors::BLANK, params);
    }
}

impl GMName for GMSpriteSheet {
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
