
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

use sdl2::render::Texture;
use sdl2::rect::Rect;
use log::debug;
use hecs::World;

use crate::context::GMContext;
use crate::math::GMPosition;
use crate::util::GMVisible;

pub struct GMTexture {
    cols: u32,
    unit_width: u32,
    unit_height: u32,
    texture: Texture,
}

unsafe impl Sync for GMTexture {}
unsafe impl Send for GMTexture {}

impl Debug for GMTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GMTexture, cols: '{}', unit_width: '{}', uni_height: '{}'", self.cols, self.unit_width, self.unit_height)
    }
}

impl GMTexture {
    pub fn new(unit_width: u32, unit_height: u32, texture: Texture) -> Self {
        debug!("GMTexture::new(), unit_width: '{}', unit_height: '{}'", unit_width, unit_height);

        let query = texture.query();
        let cols = query.width / unit_width;

        Self {
            cols,
            unit_width,
            unit_height,
            texture,
        }
    }

    pub fn draw(&self, dx: f32, dy: f32, index: u32, context: &mut GMContext) {
        self.draw_opt(dx, dy, index, 0.0, 1.0, false, false, context);
    }

    pub fn draw_opt(&self, dx: f32, dy: f32, index: u32, angle: f32, scale: f32, flip_x: bool, flip_y: bool, context: &mut GMContext) {
        let yi = index / self.cols;
        let xi = index - (yi * self.cols);

        let sx = (xi * self.unit_width) as i32;
        let sy = (yi * self.unit_height) as i32;

        let src_rect = Rect::new(sx, sy, self.unit_width, self.unit_height);
        let new_width = ((self.unit_width as f32) * scale) as u32;
        let new_height = ((self.unit_height as f32) * scale) as u32;
        let dx = (dx as i32) - ((new_width / 2) as i32);
        let dy = (dy as i32) - ((new_height / 2) as i32);
        let dst_rect = Rect::new(dx, dy, new_width, new_height);

        context.draw_texture_opt(&self.texture, src_rect, dst_rect, angle as f64, flip_x, flip_y);
    }

    pub fn get_unit_dimension(&self) -> (f32, f32) {
        (self.unit_width as f32, self.unit_height as f32)
    }

    pub fn get_cols(&self) -> u32 {
        self.cols
    }

    pub fn get_unit_width(&self) -> u32 {
        self.unit_width
    }

    pub fn get_unit_height(&self) -> u32 {
        self.unit_height
    }
}

// ECS
#[derive(Clone, Debug)]
pub struct GMTextureIndex(pub u32);

#[derive(Clone, Debug)]
pub struct GMSharedTexture(pub Arc<GMTexture>);

pub fn gm_draw_textures(world: &mut World, context: &mut GMContext) {
    for (_e, (texture,
        position,
        index,
        visible
        )) in
        world.query::<(&GMSharedTexture,
            &GMPosition,
            &GMTextureIndex,
            &GMVisible
            )>().iter() {
        if visible.0 {
            let v = position.0;
            let x = v.x;
            let y = v.y;
            texture.0.draw(x, y, index.0, context);
        }
    }
}
