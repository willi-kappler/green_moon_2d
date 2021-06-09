
use crate::animation::GMAnimationT;
use crate::spritesheet::GMSpriteSheet;

use std::rc::Rc;

pub struct GMSprite {
    sheet: Rc<GMSpriteSheet>,
    animation: Box<dyn GMAnimationT>,
    x: f32,
    y: f32,
    visible: bool,
    active: bool,
}

impl GMSprite {
    pub fn new(sheet: &Rc<GMSpriteSheet>, animation: Box<dyn GMAnimationT>, x: f32, y: f32) -> Self {
        Self {
            sheet: sheet.clone(),
            animation,
            x,
            y,
            visible: true,
            active: true,
        }
    }

    pub fn clone_sprite(&self) -> Self {
        Self {
            sheet: self.sheet.clone(),
            animation: self.animation.clone_animation(),
            x: self.x,
            y: self.y,
            visible: self.visible,
            active: self.active,
        }
    }
}
