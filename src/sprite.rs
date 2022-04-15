
use std::rc::Rc;
use std::fmt::{self, Debug, Formatter};
use std::any::Any;

use crate::animation::{GMAnimationT};
use crate::context::GMContext;
use crate::draw_object::{GMDrawT, GMDrawRefType, GMDrawMutRefType};
use crate::GMError;
use crate::movement::{GMMovementT, GMMovementInner};
use crate::texture::GMTexture;

#[derive(Debug, Clone)]
pub struct GMSpriteInner {
    pub texture: Rc<GMTexture>,
    pub movement_inner: GMMovementInner,
    pub active: bool,
    pub animations: Vec<Box<dyn GMAnimationT>>,
    pub current_animation: usize,
    pub movements: Vec<Box<dyn GMMovementT>>,
    pub z_index: i32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl GMSpriteInner {
    pub fn new(texture: Rc<GMTexture>, x: f32, y: f32, animation: Box<dyn GMAnimationT>) -> Self {
        let (width, height) = texture.get_unit_dimension();

        Self {
            texture,
            movement_inner: GMMovementInner::new(x, y, width, height),
            animations: vec![animation],
            ..Default::default()
        }
    }

    pub fn update(&mut self, context: &mut GMContext) {
        if self.active {
            self.animations[self.current_animation].update();
            for movement in self.movements.iter_mut() {
                movement.update(&mut self.movement_inner, context);
            }
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        let index = self.animations[self.current_animation].frame_index();
        let x = self.movement_inner.x;
        let y = self.movement_inner.y;

        self.texture.draw_ex(x, y, index, self.movement_inner.angle, self.flip_x, self.flip_y, context);
    }
}

impl Default for GMSpriteInner {
    fn default() -> Self {
        Self {
            texture: Rc::new(GMTexture::default()),
            movement_inner: GMMovementInner::default(),
            active: true,
            animations: Vec::new(),
            current_animation: 0,
            movements: Vec::new(),
            z_index: 0,
            flip_x: false,
            flip_y: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMSprite {
    pub sprite_inner: GMSpriteInner,
    pub effects: Vec<Box<dyn GMSpriteEffectT>>,
}

impl Default for GMSprite {
    fn default() -> Self {
        Self { sprite_inner: Default::default(), effects: Vec::new() }
    }
}

impl GMSprite {
    pub fn new(texture: Rc<GMTexture>, x: f32, y: f32, animation: Box<dyn GMAnimationT>) -> Self {
        let sprite_inner = GMSpriteInner::new(texture, x, y, animation);

        Self {
            sprite_inner, effects: Vec::new(),
        }
    }
}

impl GMDrawT for GMSprite {
    fn update(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        self.sprite_inner.update(context);

        if self.sprite_inner.active {
            for effect in self.effects.iter_mut() {
                effect.update(&mut self.sprite_inner, context);
            }
        }

        Ok(())
    }

    fn draw(&self, context: &mut GMContext) {
        if self.sprite_inner.active {
            for effect in self.effects.iter() {
                effect.draw(&self.sprite_inner, context);
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.sprite_inner.active = active;
    }

    fn get_z_index(&self) -> i32 {
        self.sprite_inner.z_index
    }

    fn set_z_index(&mut self, z_index: i32) {
        self.sprite_inner.z_index = z_index;
    }

    fn get_movmement_inner_ref(&self) -> & GMMovementInner {
        &self.sprite_inner.movement_inner
    }

    fn get_movmement_inner_mut_ref(&mut self) -> &mut GMMovementInner {
        &mut self.sprite_inner.movement_inner
    }

    fn box_clone(&self) -> Box<dyn GMDrawT> {
        let result = self.clone();

        Box::new(result)
    }

    fn cast_ref(&self) -> GMDrawRefType {
        GMDrawRefType::Sprite(self)
    }

    fn cast_mut_ref(&mut self) -> GMDrawMutRefType {
        GMDrawMutRefType::Sprite(self)
    }
}

pub enum GMSpriteEffectRefType<'a> {
    Static(&'a GMSpriteEffectStatic),

    Custom(&'a dyn Any)
}

pub enum GMSpriteEffectMutRefType<'a> {
    Static(&'a mut GMSpriteEffectStatic),

    Custom(&'a mut dyn Any)
}

pub trait GMSpriteEffectT {
    fn update(&mut self, _sprite_inner: &mut GMSpriteInner, _context: &mut GMContext) {}

    fn draw(&self, _sprite_inner: &GMSpriteInner, _context: &mut GMContext) {}

    fn set_active(&mut self, _active: bool) {}

    fn box_clone(&self) -> Box<dyn GMSpriteEffectT>;

    fn cast_ref(&self) -> GMSpriteEffectRefType;

    fn cast_mut_ref(&mut self) -> GMSpriteEffectMutRefType;
}

impl Clone for Box<dyn GMSpriteEffectT> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Debug for Box<dyn GMSpriteEffectT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GMSpriteEffectT")
    }
}

#[derive(Clone, Debug)]
pub struct GMSpriteEffectStatic {
    active: bool,
}

impl Default for GMSpriteEffectStatic {
    fn default() -> Self {
        Self { active: true }
    }
}

impl GMSpriteEffectT for GMSpriteEffectStatic {
    fn draw(&self, sprite_inner: &GMSpriteInner, context: &mut GMContext) {
        if self.active {
            sprite_inner.draw(context);
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMSpriteEffectT> {
        let result = self.clone();

        Box::new(result)
    }

    fn cast_ref(&self) -> GMSpriteEffectRefType {
        GMSpriteEffectRefType::Static(self)
    }

    fn cast_mut_ref(&mut self) -> GMSpriteEffectMutRefType {
        GMSpriteEffectMutRefType::Static(self)
    }
}
