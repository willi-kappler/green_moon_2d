
use std::rc::Rc;

use log::debug;

use crate::texture::GMTexture;
use crate::animation::{GMAnimationT, GMAnimationStatic};
use crate::context::GMContext;
use crate::math::GMVec2D;

#[derive(Debug)]
pub struct GMSprite {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,

    angle: f32,
    flip_x: bool,
    flip_y: bool,

    texture: Rc<GMTexture>,
    animation: Box<dyn GMAnimationT>,

    visible: bool,
    active: bool,
}

impl GMSprite {
    pub fn new(texture: Rc<GMTexture>, x: f32, y: f32) -> Self {
        debug!("GMSprite::new(), x: '{}', y: '{}'", x, y);

        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            angle: 0.0,
            flip_x: false,
            flip_y: false,
            texture,
            animation: Box::new(GMAnimationStatic::new(0)),
            visible: true,
            active: true,
        }
    }

    pub fn new_anim<T: 'static + GMAnimationT>(texture: Rc<GMTexture>, animation: T, x: f32, y: f32) -> Self {
        debug!("GMSprite::new(), x: '{}', y: '{}'", x, y);

        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            angle: 0.0,
            flip_x: false,
            flip_y: false,
            texture,
            animation: Box::new(animation),
            visible: true,
            active: true,
        }
    }

    pub fn set_animation(&mut self, animation: Box<dyn GMAnimationT>) {
        self.animation = animation;
    }

    pub fn set_position(&mut self, position: GMVec2D) {
        self.x = position.x;
        self.y = position.y;
    }

    pub fn update(&mut self, _context: &mut GMContext) {
        if self.active {
            self.animation.update();

            self.vx += self.ax;
            self.vy += self.ay;

            self.x += self.vx;
            self.y += self.vy;
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        if self.visible {
            let index = self.animation.texture_index();
            self.texture.draw_opt(self.x, self.y, index, self.angle, self.flip_x, self.flip_y, context);    
        }
    }
}
