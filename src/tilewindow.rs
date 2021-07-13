
use crate::tilemap::GMTileMap;

pub struct GMTileWindow {
    tile_map: GMTileMap,
    offset_tx: usize,
    offset_ty: usize,
    screen_x: f32,
    screen_y: f32,
    window_width: usize,
    window_height: usize,
}

impl GMTileWindow {
    pub fn new(tile_map: &GMTileMap, screen_x: f32, screen_y: f32, window_width: usize, window_height: usize) -> Self {
        Self {
            tile_map: tile_map.clone(),
            offset_tx: 0,
            offset_ty: 0,
            screen_x,
            screen_y,
            window_width,
            window_height,
        }
    }
    pub fn set_offset_tx(&mut self, offset_tx: usize) {
        self.offset_tx = offset_tx;
    }
    pub fn set_offset_ty(&mut self, offset_ty: usize) {
        self.offset_ty = offset_ty;
    }
    pub fn set_screen_x(&mut self, screen_x: f32) {
        self.screen_x = screen_x;
    }
    pub fn set_screen_y(&mut self, screen_y: f32) {
        self.screen_y = screen_y;
    }
    pub fn set_window_width(&mut self, window_width: usize) {
        self.window_width = window_width;
    }
    pub fn set_window_height(&mut self, window_height: usize) {
        self.window_height = window_height;
    }
    pub fn draw(&self) {
        let tx1 = self.offset_tx;
        let tx2 = self.offset_tx + self.window_width;
        let ty1 = self.offset_ty;
        let ty2 = self.offset_ty + self.window_height;
        self.tile_map.draw(self.screen_x, self.screen_y,tx1, tx2, ty1, ty2);
    }
    pub fn center_tile(&mut self, tx: usize, ty: usize) {
        let min_x = self.window_width / 2;
        let max_x = self.tile_map.get_width() - min_x;
        let min_y = self.window_height / 2;
        let max_y = self.tile_map.get_height() - min_y;

        if tx < min_x {
            self.offset_tx = 0;
        } else if tx > max_x {
            self.offset_tx = self.tile_map.get_width() - self.window_width;
        } else {
            self.offset_tx = tx - min_x;
        }
        if ty < min_y {
            self.offset_ty = 0;
        } else if ty > max_y {
            self.offset_ty = self.tile_map.get_height() - self.window_height;
        } else {
            self.offset_ty = ty - min_y;
        }
    }
}
