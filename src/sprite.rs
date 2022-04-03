
use std::rc::Rc;

use crate::texture::GMTexture;
use crate::animation::{GMAnimationT};
use crate::movement::{GMMovementT, GMMovementInner};


pub struct GMSpriteInner {
    pub texture: Rc<GMTexture>,
    pub movement_inner: GMMovementInner,
    pub active: bool,
    pub animation: Box<dyn GMAnimationT>,
}

impl GMSpriteInner {
    pub fn new(texture: Rc<GMTexture>, movement_inner: GMMovementInner, active: bool, animation: Box<dyn GMAnimationT>) -> Self {
        Self {
            texture,
            movement_inner,
            active,
            animation
        }
    }

    pub fn update(&mut self) {
        self.animation.update();
    }

    pub fn draw(&mut self) {
        let index = self.animation.frame_index();
        let x = self.movement_inner.x;
        let y = self.movement_inner.y;

        self.texture.draw(x, y, index)
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

pub trait GMSpriteT {
    fn update(&mut self);
    fn draw(&mut self);
    fn set_active(&mut self, active: bool);
}

pub trait GMSpriteEffectT {
    fn update(&mut self, sprite_inner: &mut GMSpriteInner);
    fn draw(&mut self, sprite_inner: &mut GMSpriteInner);
    fn set_active(&mut self, active: bool);
}


pub struct GMSprite {
    sprite_inner: GMSpriteInner,
    movements: Vec<Box<dyn GMMovementT>>,
    effects: Vec<Box<dyn GMSpriteEffectT>>,
}

impl GMSprite {
    pub fn new(texture: Rc<GMTexture>, x: f32, y: f32, animation: Box<dyn GMAnimationT>) -> Self {
        let (width, height) = texture.get_unit_dimension();

        let movement_inner = GMMovementInner::new(
            x,
            y,
            width,
            height,
        );

        let sprite_inner = GMSpriteInner::new(
            texture,
            movement_inner,
            true,
            animation,
        );

        Self {
            sprite_inner,
            movements: Vec::new(),
            effects: Vec::new(),
        }
    }
}

impl GMSpriteT for GMSprite {
    fn update(&mut self) {
        if self.sprite_inner.active {
            self.sprite_inner.update();

            for movement in self.movements.iter_mut() {
                movement.update(&mut self.sprite_inner.movement_inner);
            }

            for effect in self.effects.iter_mut() {
                effect.update(&mut self.sprite_inner);
            }
        }
    }

    fn draw(&mut self) {
        if self.sprite_inner.active {
            for effect in self.effects.iter_mut() {
                effect.draw(&mut self.sprite_inner);
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.sprite_inner.set_active(active);
    }
}
