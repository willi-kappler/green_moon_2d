use crate::error::GMError;

use macroquad::texture::{Texture2D, draw_texture_ex, load_texture, DrawTextureParams};
use macroquad::color::colors;
use macroquad::math::Rect;

use std::collections::HashMap;
use std::rc::Rc;

pub struct GMTileSet {
    data: Texture2D,
    mapping: HashMap<u32, Rect>,
}

impl GMTileSet {
    pub async fn new(file_name: &str) -> Result<Self, GMError> {
        let data = load_texture(file_name).await?;

        let tile_set = Self {
            data,
            mapping: HashMap::new(),
        };

        Ok(tile_set)
    }
    pub async fn new_rc(file_name: &str) -> Result<Rc<Self>, GMError> {
        let tile_set = GMTileSet::new(file_name).await?;
        Ok(Rc::new(tile_set))
    }
    pub fn set_mapping(&mut self, mapping: HashMap<u32, Rect>) {
        self.mapping = mapping;
    }
    pub fn set_mapping_array(&mut self, tile_width: f32, tile_height: f32, mapping: &[u32]) {
        let mut current_x = 0.0;
        let mut current_y = 0.0;

        for id in mapping.iter() {
            let rect = Rect::new(current_x, current_y, tile_width, tile_height);
            self.mapping.insert(*id, rect);
            current_x += tile_width;
            if current_x >= self.data.width() {
                current_x = 0.0;
                current_y += tile_height;
            }
        }
    }
    pub fn draw(&self, tile_id: u32, x: f32, y: f32) {
        let source = self.mapping[&tile_id];
        let params = DrawTextureParams {
            source: Some(source),
            .. Default::default()
        };

        draw_texture_ex(self.data, x, y, colors::WHITE, params);
    }
    pub fn get_extend(&self, tile_id: u32) -> (f32, f32) {
        let rect = self.mapping[&tile_id];
        (rect.w, rect.h)
    }
}
