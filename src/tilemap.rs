
use crate::tileset::GMTileSet;

use std::rc::Rc;

#[derive(Clone)]
pub struct GMTileMap {
    tileset: Rc<GMTileSet>,
    map: Vec<u32>,
    // width and height are number of tiles, not pixels!
    width: usize,
    height: usize,
}

impl GMTileMap {
    pub fn new(tileset: Rc<GMTileSet>, width: usize, height: usize, map: &[u32]) -> Self {
        Self {
            tileset,
            map: map.to_vec(),
            width,
            height,
        }
    }
    pub fn get_tile(&self, x: usize, y: usize) -> u32 {
        self.map[(y * (self.width as usize)) + x]
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }
    pub fn set_map(&mut self, map: &[u32]) {
        self.map = map.to_vec();
    }
    pub fn get_tile_width(&self) -> f32 {
        self.tileset.get_tile_width()
    }
    pub fn get_tile_height(&self) -> f32 {
        self.tileset.get_tile_height()
    }
    pub fn draw(&self, sx: f32, sy: f32, tx1: usize, tx2: usize, ty1: usize, ty2: usize) {
        let mut x = sx;
        let mut y = sy;
        let tile_width = self.tileset.get_tile_width();
        let tile_height = self.tileset.get_tile_height();

        for ty in ty1..ty2 {
            for tx in tx1..tx2 {
                let tile_id = self.get_tile(tx, ty);
                self.tileset.draw(tile_id, x, y);
                x += tile_width;
            }
            x = sx;
            y += tile_height;
        }
    }
}
