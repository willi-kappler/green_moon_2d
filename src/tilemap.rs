
use crate::tileset::GMTileSet;

use std::rc::Rc;

#[derive(Clone)]
pub struct GMTileMap {
    tileset: Rc<GMTileSet>,
    map: Vec<u32>,
    // width and height are number of tiles, not pixels!
    width: usize,
    height: usize,
    range_mapping: Vec<(u32, u32, String)>,
}

impl GMTileMap {
    pub fn new(tileset: Rc<GMTileSet>, width: usize, height: usize, map: &[u32]) -> Self {
        Self {
            tileset,
            map: map.to_vec(),
            width,
            height,
            range_mapping: Vec::new(),
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
    pub fn set_range_mapping(&mut self, range_mapping: &Vec<(u32, u32, String)>) {
        self.range_mapping = range_mapping.clone();
    }
    pub fn get_tile_width(&self) -> f32 {
        self.tileset.get_tile_width()
    }
    pub fn get_tile_height(&self) -> f32 {
        self.tileset.get_tile_height()
    }
    pub fn draw_old(&self, screen_x: f32, screen_y: f32, world_x: f32, world_y: f32, window_width: f32, window_height: f32) {
        let tile_width = self.get_tile_width();
        let tile_height = self.get_tile_height();
        let tile_x1 = (world_x / tile_width).floor() as usize;
        let tile_y1 = (world_y / tile_height).floor() as usize;
        let tile_x2 = ((world_x + window_width) / tile_width).floor() as usize;
        let tile_y2 = ((world_y + window_height) / tile_height).floor() as usize;
        let mut tile_x = tile_x1;
        let mut tile_y = tile_y1;

        loop {
            let tile_id = self.get_tile(tile_x, tile_y);
            let sx = (tile_x as f32) * tile_width;
            let sy = (tile_y as f32) * tile_height;
            let dx1 = sx - world_x;
            let dy1 = sy - world_y;
            let dx2 = window_width - sx + tile_width - world_x;
            let dy2 = window_height - sy + tile_height - world_y;

            if dx1 < 0.0 {
                if dy1 < 0.0 {
                    self.tileset.draw_part(tile_id, screen_x, screen_y, -dx1, -dy1, tile_width + dx1, tile_height + dy1);
                } else if dy2 < 0.0 {
                    self.tileset.draw_part(tile_id, screen_x, sy, -dx1, 0.0, tile_width + dx1, tile_height + dy2);
                } else {
                    self.tileset.draw_part(tile_id, screen_x, sy, -dx1, 0.0, tile_width + dx1, tile_height);
                }
            } else if dx2 < 0.0 {
                if dy1 < 0.0 {
                    self.tileset.draw_part(tile_id, sx, screen_y, 0.0, -dy1, tile_width + dx2, tile_height + dy1);
                } else if dy2 < 0.0 {
                    self.tileset.draw_part(tile_id, sx, sy, 0.0, 0.0, tile_width + dx2, tile_height + dy2);
                } else {
                    self.tileset.draw_part(tile_id, sx, sy, 0.0, 0.0, tile_width + dx2, tile_height);                    
                }
            } else {
                if dy1 < 0.0 {
                    self.tileset.draw_part(tile_id, sx, screen_y, 0.0, -dy1, tile_width, tile_height + dy1);
                } else if dy2 < 0.0 {
                    self.tileset.draw_part(tile_id, sx, sy, 0.0, 0.0, tile_width, tile_height + dy2);
                } else {
                    self.tileset.draw(tile_id, sx + screen_x, sy + screen_y);
                }
            }

            tile_x += 1;

            if tile_x > tile_x2 {
                tile_x = tile_x1;

                tile_y += 1;

                if tile_y > tile_y2 {
                    break;
                }
            }
        }
    }
    pub fn draw(&self, world_x: f32, world_y: f32, window_width: f32, window_height: f32) {
        let tile_width = self.get_tile_width();
        let tile_height = self.get_tile_height();
        let tile_x1 = (world_x / tile_width).floor() as usize;
        let tile_y1 = (world_y / tile_height).floor() as usize;
        let tile_x2 = ((world_x + window_width) / tile_width).floor() as usize;
        let tile_y2 = ((world_y + window_height) / tile_height).floor() as usize;
        let mut tile_x = tile_x1;
        let mut tile_y = tile_y1;
        let start_x = ((tile_x as f32) * tile_width) - world_x;
        let mut sx = start_x;
        let mut sy = ((tile_y as f32) * tile_height) - world_y;

        loop {
            let tile_id = self.get_tile(tile_x, tile_y);

            self.tileset.draw(tile_id, sx, sy);

            tile_x += 1;
            sx += tile_width;

            if tile_x > tile_x2 {
                tile_x = tile_x1;
                sx = start_x;

                tile_y += 1;
                sy += tile_height;

                if tile_y > tile_y2 {
                    break;
                }
            }
        }
}
    pub fn tile_in_range(&self, x: usize, y: usize, low_id: u32, high_id: u32) -> bool {
        let tile_id = self.get_tile(x, y);
        (low_id <= tile_id) && (tile_id <= high_id)
    }
    pub fn get_tile_type(&self, x: usize, y: usize) -> &str {
        for (low_id, high_id, name) in self.range_mapping.iter() {
            if self.tile_in_range(x, y, *low_id, *high_id) {
                return name
            }
        }

        "gm_unknown"
    }
}
