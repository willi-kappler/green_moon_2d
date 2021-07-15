use crate::tileset::GMTileSet;
use crate::behavior::GMKeyValue;

use log::error;

use std::rc::Rc;

pub trait GMBorderT {
    fn draw(&self);
    fn update(&mut self);
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn set_property(&mut self, data: &GMKeyValue);
}

pub struct GMBorder {
    border: Box<dyn GMBorderT>,
}

impl GMBorder {
    pub fn new<T: 'static + GMBorderT>(border: T) -> Self {
        Self {
            border: Box::new(border),
        }
    }
    pub fn draw(&self) {
        self.border.draw();
    }
    pub fn update(&mut self) {
        self.border.update();
    }
    pub fn set_x(&mut self, x: f32) {
        self.border.set_x(x);
    }
    pub fn set_y(&mut self, y: f32) {
        self.border.set_y(y);
    }
    pub fn set_property(&mut self, data: &GMKeyValue) {
        self.border.set_property(data)
    }
}

pub struct GMBorderSingleTile {
    tileset: Rc<GMTileSet>,
    tile_id: u32,
    width: u32,
    height: u32,
    x: f32,
    y: f32,
}

impl GMBorderSingleTile {
    pub fn new(tileset: &Rc<GMTileSet>, tile_id: u32, width: u32, height: u32, x: f32, y: f32) -> Self {
        Self {
            tileset: tileset.clone(),
            tile_id,
            // Number of tiles
            width,
            // Number of tiles
            height,
            x,
            y,
        }
    }
    pub fn set_tile_id(&mut self, tile_id: u32) {
        self.tile_id = tile_id;
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

impl GMBorderT for GMBorderSingleTile {
    fn draw(&self) {
        let tile_width = self.tileset.get_tile_width();
        let tile_height = self.tileset.get_tile_height();

        let border_width = ((self.width - 1) as f32) * tile_width;
        let border_height = ((self.height - 1) as f32) * tile_height;

        let tile_id = self.tile_id;

        // Draw top and bottom line
        for tile_x in 0..self.width {
            let screen_x = self.x + ((tile_x as f32) * tile_width);
            self.tileset.draw(tile_id, screen_x, self.y);
            self.tileset.draw(tile_id, screen_x, self.y + border_height);
        }

        // Draw left and right border
        for tile_y in 1..(self.height - 1) {
            let screen_y = self.y + ((tile_y as f32) * tile_height);
            self.tileset.draw(tile_id, self.x, screen_y);
            self.tileset.draw(tile_id, self.x + border_width, screen_y);
        }
    }
    fn update(&mut self) {
    }
    fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    fn set_property(&mut self, data: &GMKeyValue) {
        if data.key == "tile_id" {
            match data.value.downcast_ref::<u32>() {
                Some(a) => {
                    self.set_tile_id(*a);
                }
                None => {
                    error!("GMBorderSingleTile::set_property(), '{}', could not downcast value to u32", data.key)
                }
            }
        }
    }
}

pub struct GMBorder9Tiles {
    tileset: Rc<GMTileSet>,
    tile_top_left: u32,
    tile_top: u32,
    tile_top_right: u32,
    tile_right: u32,
    tile_bottom_right: u32,
    tile_bottom: u32,
    tile_bottom_left: u32,
    tile_left: u32,
    width: u32,
    height: u32,
    x: f32,
    y: f32,
}

impl GMBorder9Tiles {
    pub fn new(tileset: &Rc<GMTileSet>, width: u32, height: u32, x: f32, y: f32) -> Self {
        Self {
            tileset: tileset.clone(),
            tile_top_left: 0,
            tile_top: 0,
            tile_top_right: 0,
            tile_right: 0,
            tile_bottom_right: 0,
            tile_bottom: 0,
            tile_bottom_left: 0,
            tile_left: 0,
            // Number of tiles
            width,
            // Number of tiles
            height,
            x,
            y,
        }
    }
    pub fn set_corner_tiles(&mut self, top_left: u32, top_right: u32, bottom_right: u32, bottom_left: u32) {
        self.tile_top_left = top_left;
        self.tile_top_right = top_right;
        self.tile_bottom_right = bottom_right;
        self.tile_bottom_left = bottom_left;

    }
    pub fn set_edge_tiles(&mut self, top: u32, right: u32, bottom: u32, left: u32) {
        self.tile_top = top;
        self.tile_right = right;
        self.tile_bottom = bottom;
        self.tile_left = left;
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

impl GMBorderT for GMBorder9Tiles {
    fn draw(&self) {
    }
    fn update(&mut self) {
    }
    fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    fn set_property(&mut self, data: &GMKeyValue) {
        if data.key == "corner" {
            match data.value.downcast_ref::<(u32, u32, u32, u32)>() {
                Some((top_left, top_right, bottom_right, bottom_left)) => {
                    self.set_corner_tiles(*top_left, *top_right, *bottom_right, *bottom_left)
                }
                None => {
                    error!("GMBorderSingleTile::set_property(), '{}', could not downcast value to u32", data.key)
                }
            }
        }
    }
}
