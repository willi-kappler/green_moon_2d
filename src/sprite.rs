
use std::rc::Rc;

use log::debug;

use crate::texture::GMTexture;
use crate::animation::{GMAnimationT, GMAnimationStatic};

pub struct GMSprite {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,

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
            texture,
            animation: Box::new(GMAnimationStatic::new(0)),
            visible: true,
            active: true,
        }
    }
}
