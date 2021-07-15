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
}

impl GMBorderT for GMBorderSingleTile {
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
