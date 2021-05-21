use crate::resource_manager::GMName;
use crate::error::GMError;

use macroquad::texture::{Texture2D, draw_texture_ex, load_texture};
use macroquad::color::colors;
use macroquad::math::Rect;
use macroquad::texture::DrawTextureParams;
pub struct GMBitmapFont {
    pub(crate) name: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) char_width: u32,
    pub(crate) char_height: u32,
    pub(crate) data: Texture2D, // TODO: Use macroquad image
}

impl GMBitmapFont {
    pub async fn from_img_file(name: &str, char_width: u32, char_height: u32, file_name: &str) -> Result<GMBitmapFont, GMError> {
        // TODO: Read from image file
        let width = 0;
        let height = 0;
        let data = load_texture(file_name).await?;

        let font = GMBitmapFont {
            name: name.to_string(),
            width,
            height,
            char_width,
            char_height,
            data
        };

        Ok(font)
    }

    pub async fn from_config_file(file_name: &str) -> Result<GMBitmapFont, GMError> {
        // TODO: Read from config file (TOML, JSON ?)
        //GMBitmapFont::from_img_file("", 0, 0, "").await
        todo!();
    }

    pub fn draw_char(&self, c: char, x: u32, y: u32) -> u32 {
        let source = Some(self.source_rect(c));
        let params = DrawTextureParams {
            source, .. Default::default()
        };

        draw_texture_ex(self.data, x as f32, y as f32, colors::BLANK, params);
        self.char_width
    }

    fn source_rect(&self, c: char) -> Rect {
        // TODO: Calculate row and col for char
        // Rect {x: 0.0, y: 0.0, w: self.char_width as f32, h: self.char_height as f32}
        todo!();
    }

}

impl GMName for GMBitmapFont {
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
