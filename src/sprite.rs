
use std::rc::Rc;

use crate::texture::GMTexture;
use crate::animation::GMAnimationT;
use crate::movement::GMMovementT;

pub struct GMSprite {
    texture: Rc<GMTexture>,
    pub x: i32,
    pub y: i32,
    pub active: bool,
    animation: Box<dyn GMAnimationT>,
    movement: Box<dyn GMMovementT>,

}

