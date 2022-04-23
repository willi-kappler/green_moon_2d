
use std::rc::Rc;
use std::fmt::{self, Debug, Formatter};

use crate::animation::{GMAnimationT};
use crate::context::{GMUpdateContext, GMDrawContext};
use crate::draw_object::{GMDrawObjectT, GMDrawObjectCommon};
use crate::GMError;
use crate::texture::GMTexture;

#[derive(Debug, Clone)]
pub struct GMSpriteInner {
    pub texture: Rc<GMTexture>,
    pub animations: Vec<Box<dyn GMAnimationT>>,
    pub current_animation: usize,
    pub flip_x: bool,
    pub flip_y: bool,
    pub draw_object_common: GMDrawObjectCommon,
}

impl GMSpriteInner {
    pub fn new(texture: Rc<GMTexture>, name: &str, x: f32, y: f32, animation: Box<dyn GMAnimationT>) -> Self {
        let (width, height) = texture.get_unit_dimension();

        Self {
            texture,
            animations: vec![animation],
            draw_object_common: GMDrawObjectCommon::new(name, x, y, width, height),
            ..Default::default()
        }
    }

    pub fn update(&mut self, context: &mut GMUpdateContext) {
        if self.draw_object_common.active {
            self.animations[self.current_animation].update();
            self.draw_object_common.update(context);
        }
    }

    pub fn draw(&self, context: &mut GMDrawContext) {
        let index = self.animations[self.current_animation].frame_index();
        let x = self.draw_object_common.movement_common.x;
        let y = self.draw_object_common.movement_common.y;

        self.texture.draw_ex(x, y, index, self.draw_object_common.movement_common.angle, self.flip_x, self.flip_y, context);
    }
}

impl Default for GMSpriteInner {
    fn default() -> Self {
        Self {
            texture: Rc::new(GMTexture::default()),
            animations: Vec::new(),
            current_animation: 0,
            flip_x: false,
            flip_y: false,
            draw_object_common: GMDrawObjectCommon::default(),
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
    pub fn new(texture: Rc<GMTexture>, name: &str, x: f32, y: f32, animation: Box<dyn GMAnimationT>) -> Self {
        let sprite_inner = GMSpriteInner::new(texture, name, x, y, animation);

        Self {
            sprite_inner, effects: Vec::new(),
        }
    }
}

impl GMDrawObjectT for GMSprite {
    fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        self.sprite_inner.update(context);

        if self.sprite_inner.draw_object_common.active {
            for effect in self.effects.iter_mut() {
                effect.update(&mut self.sprite_inner, context);
            }
        }

        Ok(())
    }

    fn draw(&self, context: &mut GMDrawContext) -> Result<(), GMError> {
        if self.sprite_inner.draw_object_common.active {
            for effect in self.effects.iter() {
                effect.draw(&self.sprite_inner, context);
            }
        }

        Ok(())
    }

    fn get_common_ref(&self) -> &GMDrawObjectCommon {
        &self.sprite_inner.draw_object_common
    }

    fn get_common_mut_ref(&mut self) -> &mut GMDrawObjectCommon {
        &mut self.sprite_inner.draw_object_common
    }

    fn box_clone(&self) -> Box<dyn GMDrawObjectT> {
        let result = self.clone();

        Box::new(result)
    }
}

pub trait GMSpriteEffectT {
    fn update(&mut self, sprite_inner: &mut GMSpriteInner, context: &mut GMUpdateContext);

    fn draw(&self, sprite_inner: &GMSpriteInner, context: &mut GMDrawContext);

    fn box_clone(&self) -> Box<dyn GMSpriteEffectT>;
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
    fn update(&mut self, sprite_inner: &mut GMSpriteInner, context: &mut GMUpdateContext) {
        if self.active {
            sprite_inner.update(context)
        }
    }

    fn draw(&self, sprite_inner: &GMSpriteInner, context: &mut GMDrawContext) {
        if self.active {
            sprite_inner.draw(context);
        }
    }

    fn box_clone(&self) -> Box<dyn GMSpriteEffectT> {
        let result = self.clone();

        Box::new(result)
    }
}
