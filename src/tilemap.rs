
use crate::tileset::GMTileSet;

use std::rc::Rc;

pub struct GMTileMap {
    tileset: Rc<GMTileSet>,
    map: Vec<u32>,
    // width and height are number of tiles, not pixels!
    width: usize,
    height: usize,
    wrap_around: bool,
}

impl GMTileMap {
    pub fn new(tileset: Rc<GMTileSet>, width: usize, height: usize) -> Self {
        Self {
            tileset,
            map: Vec::new(),
            width,
            height,
            wrap_around: false,
        }
    }
    pub fn get_tile(&self, x: usize, y: usize) -> u32 {
        self.map[(y * (self.width as usize)) + x]
    }
    pub fn draw(&self, x: f32, y: f32, width: f32, height: f32, offsetx: f32, offsety: f32) {
        let mut screenx = x;
        let mut screeny = y;

        let tile_width = self.tileset.get_tile_width();
        let tile_height = self.tileset.get_tile_height();

        let mut tilex = (offsetx / tile_width).floor() as usize;
        let mut tiley = (offsety / tile_height).floor() as usize;


        loop {
            let tile_id = self.get_tile(tilex, tiley);
            self.tileset.draw(tile_id, screenx, screeny);

            screenx += tile_width;
            tilex += 1;

            if screenx >= width {
                screenx = x;
                tilex = (offsetx / tile_width).floor() as usize;

                screeny += tile_height;
                tiley += 1;

                if screeny >= height {
                    break;
                }
            }

            if tilex == self.width {
                if self.wrap_around {
                    tilex = 0;
                } else {
                    tilex = self.width - 1;
                }
            }

            if tiley == self.height {
                if self.wrap_around {
                    tiley = 0;
                } else {
                    tiley = self.height - 1;
                }
            }
        }
    }
}
