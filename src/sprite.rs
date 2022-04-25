
use std::rc::Rc;
use std::fmt::{self, Debug, Formatter};
use std::any::Any;

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
            current_animation: 0,
            flip_x: false,
            flip_y: false,
            draw_object_common: GMDrawObjectCommon::new(name, x, y, width, height),
        }
    }

    pub fn update(&mut self, context: &mut GMUpdateContext) {
        self.animations[self.current_animation].update();
        self.draw_object_common.update(context);
    }

    pub fn draw(&self, context: &mut GMDrawContext) {
        let index = self.animations[self.current_animation].frame_index();
        let x = self.draw_object_common.movement_common.x;
        let y = self.draw_object_common.movement_common.y;
        let angle = self.draw_object_common.movement_common.angle;

        self.texture.draw_ex(x, y, index, angle, self.flip_x, self.flip_y, context);
    }
}

#[derive(Debug, Clone)]
pub struct GMSprite {
    pub sprite_inner: GMSpriteInner,
    effect_manager: GMSpriteEffectManager,
}

impl GMSprite {
    pub fn new(texture: Rc<GMTexture>, name: &str, x: f32, y: f32, animation: Box<dyn GMAnimationT>) -> Self {
        let sprite_inner = GMSpriteInner::new(texture, name, x, y, animation);

        Self {
            sprite_inner,
            effect_manager: GMSpriteEffectManager::new(),
        }
    }
}

impl GMDrawObjectT for GMSprite {
    fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        
        if self.sprite_inner.draw_object_common.active {
            while let Some(message) = self.sprite_inner.draw_object_common.get_next_message() {
                let description = &message.description;

                match description.as_str() {
                    "sprite_effect" => {
                        self.effect_manager.send_effect_message();
                    }
                    "sprite_effect_manager" => {
                        self.effect_manager.send_message();
                    }
                    _ => {
                        todo!();
                    }
                }
            }

            self.effect_manager.update(&mut self.sprite_inner, context);
        }

        Ok(())
    }

    fn draw(&self, context: &mut GMDrawContext) -> Result<(), GMError> {
        if self.sprite_inner.draw_object_common.active {
            self.effect_manager.draw(&self.sprite_inner, context);
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


pub struct GMSpriteEffectManagerMessage {
    from: String,
    description: String,
    value: Box<dyn Any>,
}

#[derive(Debug, Clone)]
struct GMSpriteEffectManager {
    effects: Vec<Box<dyn GMSpriteEffectT>>,
}

impl GMSpriteEffectManager {
    fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    fn update(&mut self, sprite_inner: &mut GMSpriteInner, context: &mut GMUpdateContext) {
        for effect in self.effects.iter_mut() {
            effect.update(sprite_inner, context);
        }
    }

    fn draw(&self, sprite_inner: &GMSpriteInner, context: &mut GMDrawContext) {
        for effect in self.effects.iter() {
            effect.draw(sprite_inner, context);
        }
    }

    fn send_effect_message(&mut self) {
        todo!();
    }

    fn send_message(&mut self) {
        todo!();
    }
}

pub trait GMSpriteEffectT {
    fn update(&mut self, sprite_inner: &mut GMSpriteInner, context: &mut GMUpdateContext);

    fn draw(&self, sprite_inner: &GMSpriteInner, context: &mut GMDrawContext);

    fn get_common_ref(&self) -> &GMSpriteEffectCommon;

    fn get_common_mut_ref(&mut self) -> &mut GMSpriteEffectCommon;

    fn box_clone(&self) -> Box<dyn GMSpriteEffectT>;
}

impl Clone for Box<dyn GMSpriteEffectT> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Debug for Box<dyn GMSpriteEffectT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GMSpriteEffect: '{}'", self.get_common_ref().name)
    }
}

#[derive(Clone, Debug)]
pub struct GMSpriteEffectCommon {
    pub active: bool,
    pub name: String,
}

impl GMSpriteEffectCommon {
    pub fn new(name: &str) -> Self {
        Self {
            active: true,
            name: name.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GMSpriteEffectDefault {
    common: GMSpriteEffectCommon,
}

impl GMSpriteEffectDefault {
    pub fn new(name: &str) -> Self {
        Self {
            common: GMSpriteEffectCommon::new(name),
        }
    }
}

impl GMSpriteEffectT for GMSpriteEffectDefault {
    fn update(&mut self, sprite_inner: &mut GMSpriteInner, context: &mut GMUpdateContext) {
        if self.common.active {
            sprite_inner.update(context);
        }
    }

    fn draw(&self, sprite_inner: &GMSpriteInner, context: &mut GMDrawContext) {
        if self.common.active {
            sprite_inner.draw(context);
        }
    }

    fn get_common_ref(&self) -> &GMSpriteEffectCommon {
        &self.common
    }

    fn get_common_mut_ref(&mut self) -> &mut GMSpriteEffectCommon {
        &mut self.common
    }

    fn box_clone(&self) -> Box<dyn GMSpriteEffectT> {
        let result = self.clone();

        Box::new(result)
    }
}


#[derive(Clone, Debug)]
pub struct GMSpriteEffectTarget {
    common: GMSpriteEffectCommon,
    group_name: String,
}


impl GMSpriteEffectT for GMSpriteEffectTarget {
    fn update(&mut self, sprite_inner: &mut GMSpriteInner, context: &mut GMUpdateContext) {
        if self.common.active {
            todo!();
        }
    }

    fn draw(&self, sprite_inner: &GMSpriteInner, context: &mut GMDrawContext) {
    }

    fn get_common_ref(&self) -> &GMSpriteEffectCommon {
        &self.common
    }

    fn get_common_mut_ref(&mut self) -> &mut GMSpriteEffectCommon {
        &mut self.common
    }

    fn box_clone(&self) -> Box<dyn GMSpriteEffectT> {
        let result = self.clone();

        Box::new(result)
    }
}
