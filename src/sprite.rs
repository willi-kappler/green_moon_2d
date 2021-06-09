
use crate::animation::GMAnimationT;
use crate::spritesheet::GMSpriteSheet;

use std::rc::Rc;

pub struct GMSprite {
    pub(crate) sheet: Rc<GMSpriteSheet>,
    pub(crate) animation: Box<dyn GMAnimationT>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) visible: bool,
    pub(crate) active: bool,
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
    pub fn draw(&self) {
        if self.visible {
            let rect = self.animation.get_rect();
            self.sheet.draw(&rect, self.x, self.y);
        }
    }
    pub fn update(&mut self) {
        if self.active {
            self.animation.next_frame();
        }
    }
}
