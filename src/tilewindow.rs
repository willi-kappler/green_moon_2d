
use crate::tilemap::GMTileMap;
use crate::sprite::GMSprite;

pub struct GMTileWindow {
    tile_map: GMTileMap,
    screen_x: f32,
    screen_y: f32,
    window_width: f32,
    window_height: f32,
    world_x: f32,
    world_y: f32,
    world_size_x: f32,
    world_size_y: f32,
}

impl GMTileWindow {
    pub fn new(tile_map: &GMTileMap, screen_x: f32, screen_y: f32, window_width: f32, window_height: f32) -> Self {
        let world_size_x = (tile_map.get_width() as f32) * tile_map.get_tile_width();
        let world_size_y = (tile_map.get_height() as f32) * tile_map.get_tile_height();

        Self {
            tile_map: tile_map.clone(),
            screen_x,
            screen_y,
            window_width,
            window_height,
            world_x: 0.0,
            world_y: 0.0,
            world_size_x,
            world_size_y,
        }
    }
    pub fn set_screen_x(&mut self, screen_x: f32) {
        self.screen_x = screen_x;
    }
    pub fn set_screen_y(&mut self, screen_y: f32) {
        self.screen_y = screen_y;
    }
    pub fn set_window_width(&mut self, window_width: f32) {
        self.window_width = window_width;
    }
    pub fn set_window_height(&mut self, window_height: f32) {
        self.window_height = window_height;
    }
    pub fn set_world_x(&mut self, x: f32) {
        self.world_x = x;
        self.check_world_x();
    }
    pub fn set_world_y(&mut self, y: f32) {
        self.world_y = y;
        self.check_world_y();
    }
    pub fn draw(&self) {
        self.tile_map.draw(self.screen_x, self.screen_y, self.world_x, self.world_y, self.window_width, self.window_height);
    }
    pub fn draw_sprite(&self, sprite: &GMSprite) {
        let sprite_x = sprite.get_x();
        let sprite_y = sprite.get_y();
        let (sprite_width, sprite_height) = sprite.get_extend();

        let mut screen_x = sprite_x - self.world_x;
        if screen_x + sprite_width < 0.0 || screen_x > self.window_width {
            return
        }

        let mut screen_y = sprite_y - self.world_y;
        if screen_y + sprite_height < 0.0 || screen_y > self.window_height {
            return
        }

        screen_x += self.screen_x;
        screen_y += self.screen_y;

        sprite.draw_at(screen_x, screen_y);
    }
    pub fn center(&mut self, px: f32, py: f32) {
        self.world_x = px - (self.window_width / 2.0);
        self.check_world_x();

        self.world_y = py - (self.window_height / 2.0);
        self.check_world_y();
    }
    pub fn get_tile(&self, tx: usize, ty: usize) -> u32 {
        self.tile_map.get_tile(tx, ty)
    }
    pub fn get_tile_pixel(&self, px: f32, py: f32) -> u32 {
        let tx = (px / self.tile_map.get_tile_width()).floor() as usize;
        let ty = (py / self.tile_map.get_tile_height()).floor() as usize;

        self.tile_map.get_tile(tx, ty)
    }
    pub fn move_up(&mut self, step: f32) {
        self.world_y -= step;

        if self.world_y < 0.0 {
            self.world_y = 0.0;
        }
    }
    pub fn move_down(&mut self, step: f32) {
        self.world_y += step;

        if self.world_y > self.world_size_y - self.window_height {
            self.world_y = self.world_size_y - self.window_height;
        }
    }
    pub fn move_left(&mut self, step: f32) {
        self.world_x -= step;

        if self.world_x < 0.0 {
            self.world_x = 0.0;
        }
    }
    pub fn move_right(&mut self, step: f32) {
        self.world_x += step;

        if self.world_x > self.world_size_x - self.window_width {
            self.world_x = self.world_size_x - self.window_width;
        }
    }
    fn check_world_x(&mut self) {
        if self.world_x < 0.0 {
            self.world_x = 0.0;
        } else if self.world_x > self.world_size_x - self.window_width {
            self.world_x = self.world_size_x - self.window_width;
        }
    }
    fn check_world_y(&mut self) {
        if self.world_y < 0.0 {
            self.world_y = 0.0;
        } else if self.world_y > self.world_size_y - self.window_height {
            self.world_y = self.world_size_y - self.window_height;
        }
    }
}
