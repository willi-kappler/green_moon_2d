use crate::resource_manager::GMName;
use crate::error::GMError;
use crate::text::GMOrientation;

use std::{collections::HashMap, hash::BuildHasherDefault};

use macroquad::texture::{Texture2D, draw_texture_ex, load_texture};
use macroquad::color::colors;
use macroquad::math::Rect;
use macroquad::texture::DrawTextureParams;

use fnv::{FnvHashMap, FnvHasher};

pub struct GMBitmapFont {
    pub(crate) name: String,
    pub(crate) x_offset: f32,
    pub(crate) y_offset: f32,
    pub(crate) mapping: HashMap<char, Rect, BuildHasherDefault<FnvHasher>>,
    pub(crate) data: Texture2D,
}

impl GMBitmapFont {
    pub async fn from_img_file(name: &str, x_offset: f32, y_offset: f32, char_mapping: Vec<(char, Rect)>, file_name: &str) -> Result<GMBitmapFont, GMError> {
        let data = load_texture(file_name).await?;
        let mut mapping = FnvHashMap::with_capacity_and_hasher(char_mapping.len(), Default::default());

        for (c, r) in char_mapping.iter() {
            mapping.insert(*c, *r);
        }

        let font = GMBitmapFont {
            name: name.to_string(),
            x_offset,
            y_offset,
            mapping,
            data
        };

        Ok(font)
    }

    pub async fn from_config_file(file_name: &str) -> Result<GMBitmapFont, GMError> {
        // TODO: Read from config file (TOML, JSON ?)
        //GMBitmapFont::from_img_file("", 0, 0, "").await
        todo!();
    }

    pub fn draw_char(&self, c: char, x: f32, y: f32, orientation: &GMOrientation) -> f32 {
        let rect = self.source_rect(c);
        let source = Some(rect);
        let params = DrawTextureParams {
            source, .. Default::default()
        };

        draw_texture_ex(self.data, x, y, colors::BLANK, params);
        match orientation {
            GMOrientation::Horizontal => {
                rect.w + self.x_offset
            }
            GMOrientation::Vertical => {
                rect.h + self.y_offset
            }
        }
    }

    fn source_rect(&self, c: char) -> Rect {
        match self.mapping.get(&c) {
            Some(rect) => {
                *rect
            }
            None => {
                Rect::new(0.0, 0.0, 32.0, 32.0)
            }
        }
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
