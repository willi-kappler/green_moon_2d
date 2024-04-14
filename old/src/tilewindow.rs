
use crate::tilemap::GMTileMap;
use crate::sprite::GMSprite;

use macroquad::camera::{Camera2D, set_camera, set_default_camera};
use macroquad::texture::{RenderTarget, render_target, draw_texture};
use macroquad::math::vec2;
use macroquad::color::colors;

// TODO:
// - add shake() method
// - add border() method for window


pub struct GMTileWindow {
    tilemap: GMTileMap,
    screen_x: f32,
    screen_y: f32,
    window_width: f32,
    window_height: f32,
    world_x: f32,
    world_y: f32,
    world_size_x: f32,
    world_size_y: f32,
    buffer: RenderTarget,
    camera: Camera2D,
}

impl GMTileWindow {
    pub fn new(tilemap: &GMTileMap, screen_x: f32, screen_y: f32, window_width: f32, window_height: f32) -> Self {
        let world_size_x = (tilemap.get_width() as f32) * tilemap.get_tile_width();
        let world_size_y = (tilemap.get_height() as f32) * tilemap.get_tile_height();

        let buffer = render_target(window_width as u32, window_height as u32);
        let camera = Camera2D {
            target: vec2(window_width / 2.0, window_height / 2.0),
            zoom: vec2(2.0 / window_width, 2.0 / window_height),
            render_target: Some(buffer),
            ..Camera2D::default()
        };

        Self {
            tilemap: tilemap.clone(),
            screen_x,
            screen_y,
            window_width,
            window_height,
            world_x: 0.0,
            world_y: 0.0,
            world_size_x,
            world_size_y,
            buffer,
            camera,
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
        self.set_buffer_and_camera();
        self.update_buffer();
    }
    pub fn set_window_height(&mut self, window_height: f32) {
        self.window_height = window_height;
        self.set_buffer_and_camera();
        self.update_buffer();
    }
    pub fn set_world_x(&mut self, x: f32) {
        self.world_x = x;
        self.check_world_x();
        self.update_buffer();
    }
    pub fn set_world_y(&mut self, y: f32) {
        self.world_y = y;
        self.check_world_y();
        self.update_buffer();
    }
    pub fn draw_old(&self) {
        self.tilemap.draw_old(self.screen_x, self.screen_y, self.world_x, self.world_y, self.window_width, self.window_height);
        // self.camera = Camera2D::from_display_rect(Rect::new(self.screen_x, self.screen_y, self.window_width, self.window_height));
        // set_camera(&self.camera);
        // draw
        // set_default_camera();
    }
    pub fn draw(&self) {
        draw_texture(self.buffer.texture, self.screen_x, self.screen_y, colors::WHITE);
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

        self.update_buffer();
    }
    pub fn get_tile(&self, tx: usize, ty: usize) -> u32 {
        self.tilemap.get_tile(tx, ty)
    }
    pub fn get_tile_pixel(&self, px: f32, py: f32) -> u32 {
        let tx = (px / self.tilemap.get_tile_width()).floor() as usize;
        let ty = (py / self.tilemap.get_tile_height()).floor() as usize;

        self.tilemap.get_tile(tx, ty)
    }
    pub fn move_up(&mut self, step: f32) {
        self.world_y -= step;

        if self.world_y < 0.0 {
            self.world_y = 0.0;
        }

        self.update_buffer();
    }
    pub fn move_down(&mut self, step: f32) {
        self.world_y += step;

        if self.world_y > self.world_size_y - self.window_height {
            self.world_y = self.world_size_y - self.window_height;
        }

        self.update_buffer();
    }
    pub fn move_left(&mut self, step: f32) {
        self.world_x -= step;

        if self.world_x < 0.0 {
            self.world_x = 0.0;
        }

        self.update_buffer();
    }
    pub fn move_right(&mut self, step: f32) {
        self.world_x += step;

        if self.world_x > self.world_size_x - self.window_width {
            self.world_x = self.world_size_x - self.window_width;
        }

        self.update_buffer();
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
    fn set_buffer_and_camera(&mut self) {
        self.buffer = render_target(self.window_width as u32, self.window_height as u32);
        self.camera = Camera2D {
            target: vec2(self.window_width / 2.0, self.window_height / 2.0),
            zoom: vec2(2.0 / self.window_width, 2.0 / self.window_height),
            render_target: Some(self.buffer),
            ..Camera2D::default()
        };
    }
    fn update_buffer(&self) {
        set_camera(&self.camera);
        self.tilemap.draw(self.world_x, self.world_y, self.window_width, self.window_height);
        set_default_camera();
    }
}
