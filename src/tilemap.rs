
use crate::tileset::GMTileSet;

use std::rc::Rc;

pub struct GMTileMap {
    pub(crate) tile_set: Rc<GMTileSet>,
    pub(crate) map: Vec<u32>,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl GMTileMap {
    pub fn new(tile_set: &Rc<GMTileSet>) -> Self {
        Self {
            tile_set: tile_set.clone(),
            map: Vec::new(),
            width: 0,
            height: 0,
        }
    }
    pub fn set_map(&mut self, map: &[u32], width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.map = map.to_vec();
    }
    pub async fn load_map(&mut self, file_name: &str) {
        todo!();
    }
    pub fn set_tile_set(&mut self, tile_set: &Rc<GMTileSet>) {
        self.tile_set = tile_set.clone();
    }
    pub fn draw(&self) {
        todo!();
    }
}
