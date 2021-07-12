use crate::error::GMError;

use macroquad::texture::{Texture2D, draw_texture_ex, load_texture, DrawTextureParams};
use macroquad::color::colors;
use macroquad::math::Rect;

use std::collections::HashMap;
use std::rc::Rc;

pub struct GMTileSet {
    data: Texture2D,
    mapping: HashMap<u32, (f32, f32)>,
    tile_width: f32,
    tile_height: f32,
}

impl GMTileSet {
    pub async fn new(file_name: &str, tile_width: f32, tile_height: f32) -> Result<Self, GMError> {
        let data = load_texture(file_name).await?;

        let tile_set = Self {
            data,
            mapping: HashMap::new(),
            tile_width,
            tile_height,
        };

        Ok(tile_set)
    }
    pub async fn new_rc(file_name: &str, tile_width: f32, tile_height: f32) -> Result<Rc<Self>, GMError> {
        let tile_set = GMTileSet::new(file_name, tile_width, tile_height).await?;
        Ok(Rc::new(tile_set))
    }
    pub fn set_mapping(&mut self, mapping: HashMap<u32, (f32, f32)>) {
        self.mapping = mapping;
    }
    pub fn draw(&self, tile_id: u32, screenx: f32, screeny: f32) {
        let (tilex, tiley) = self.mapping[&tile_id];
        let params = DrawTextureParams {
            source: Some(Rect::new(tilex, tiley, self.tile_width, self.tile_height)),
            .. Default::default()
        };

        draw_texture_ex(self.data, screenx, screeny, colors::WHITE, params);
    }
    pub fn get_tile_width(&self) -> f32 {
        self.tile_width
    }
    pub fn get_tile_height(&self) -> f32 {
        self.tile_height
    }
}
