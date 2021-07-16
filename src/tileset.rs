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
    pub async fn new(file_name: &str, tile_width: f32, tile_height: f32, mapping: &HashMap<u32, (f32, f32)>) -> Result<Self, GMError> {
        let data = load_texture(file_name).await?;

        let tileset = Self {
            data,
            mapping: mapping.clone(),
            tile_width,
            tile_height,
        };

        Ok(tileset)
    }
    pub async fn new_rc(file_name: &str, tile_width: f32, tile_height: f32, mapping: &HashMap<u32, (f32, f32)>) -> Result<Rc<Self>, GMError> {
        let tileset = GMTileSet::new(file_name, tile_width, tile_height, mapping).await?;
        Ok(Rc::new(tileset))
    }
    pub fn get_tile_width(&self) -> f32 {
        self.tile_width
    }
    pub fn get_tile_height(&self) -> f32 {
        self.tile_height
    }
    pub fn set_tile_width(&mut self, tile_width: f32) {
        self.tile_width = tile_width;
    }
    pub fn set_tile_height(&mut self, tile_height: f32) {
        self.tile_height = tile_height;
    }
    pub fn set_mapping(&mut self, mapping: &HashMap<u32, (f32, f32)>) {
        self.mapping = mapping.clone();
    }
    pub fn draw(&self, tile_id: u32, screen_x: f32, screen_y: f32) {
        let (tile_x, tile_y) = self.mapping[&tile_id];
        let params = DrawTextureParams {
            source: Some(Rect::new(tile_x, tile_y, self.tile_width, self.tile_height)),
            .. Default::default()
        };

        draw_texture_ex(self.data, screen_x, screen_y, colors::WHITE, params);
    }
    pub fn draw_part(&self, tile_id: u32, screen_x: f32, screen_y: f32, offset_x: f32, offset_y: f32, width: f32, height: f32) {
        let (tile_x, tile_y) = self.mapping[&tile_id];
        let params = DrawTextureParams {
            source: Some(Rect::new(tile_x + offset_x, tile_y + offset_y, width, height)),
            .. Default::default()
        };

        draw_texture_ex(self.data, screen_x, screen_y, colors::WHITE, params);
    }
}
