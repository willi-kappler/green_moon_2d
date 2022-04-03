
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

impl Clone for GMSpriteInner {
    fn clone(&self) -> Self {
        Self {
            texture: self.texture.clone(),
            movement_inner: self.movement_inner.clone(),
            active: self.active.clone(),
            animation: self.animation.box_clone() }
    }
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
    fn get_inner(&self) -> &GMSpriteInner;
    fn get_inner_mut(&mut self) -> &mut GMSpriteInner;
    fn get_movements(&self) -> &[Box<dyn GMMovementT>];
    fn get_movements_mut(&mut self) -> &mut [Box<dyn GMMovementT>];
    fn get_effects(&self) -> &[Box<dyn GMSpriteEffectT>];
    fn get_effects_mut(&mut self) -> &mut [Box<dyn GMSpriteEffectT>];
    fn box_clone(&self) -> Box<dyn GMSpriteT>;
}

pub trait GMSpriteEffectT {
    fn update(&mut self, sprite_inner: &mut GMSpriteInner);
    fn draw(&mut self, sprite_inner: &mut GMSpriteInner);
    fn set_active(&mut self, active: bool);
    fn box_clone(&self) -> Box<dyn GMSpriteEffectT>;
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

    fn get_inner(&self) -> &GMSpriteInner {
        &self.sprite_inner
    }

    fn get_inner_mut(&mut self) -> &mut GMSpriteInner {
        &mut self.sprite_inner
    }

    fn get_movements(&self) -> &[Box<dyn GMMovementT>] {
        &self.movements
    }

    fn get_movements_mut(&mut self)  -> &mut [Box<dyn GMMovementT>] {
        &mut self.movements
    }

    fn get_effects(&self) -> &[Box<dyn GMSpriteEffectT>] {
        &self.effects
    }

    fn get_effects_mut(&mut self) -> &mut [Box<dyn GMSpriteEffectT>] {
        &mut self.effects
    }

    fn box_clone(&self) -> Box<dyn GMSpriteT> {
        let result = GMSprite {
            sprite_inner: self.sprite_inner.clone(),
            movements: self.movements.iter().map(|m| m.box_clone()).collect(),
            effects: self.effects.iter().map(|e| e.box_clone()).collect(),
        };

        Box::new(result)
    }
}

pub struct GMSpriteEffectStatic {
    active: bool,
}

impl GMSpriteEffectStatic {
    pub fn new() -> Self {
        Self {
            active: true,
        }
    }
}

impl GMSpriteEffectT for GMSpriteEffectStatic {
    fn update(&mut self, _sprite_inner: &mut GMSpriteInner) {}

    fn draw(&mut self, sprite_inner: &mut GMSpriteInner) {
        if self.active {
            sprite_inner.draw();
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMSpriteEffectT> {
        let result = GMSpriteEffectStatic {
            active: self.active
        };

        Box::new(result)
    }
}
