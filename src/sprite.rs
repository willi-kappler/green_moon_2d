
use std::rc::Rc;

use crate::texture::GMTexture;
use crate::animation::GMAnimationT;
use crate::movement::GMMovementT;

pub struct GMSprite {
    texture: Rc<GMTexture>,
    pub x: f32,
    pub y: f32,
    pub active: bool,
    animation: Box<dyn GMAnimationT>,
    movement: Box<dyn GMMovementT>,

}

