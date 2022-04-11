
use std::rc::Rc;
use std::any::Any;
use std::fmt::{self, Debug, Formatter};

use crate::animation::{GMAnimationT};
use crate::context::GMContext;
use crate::draw_object::{GMDrawT, GMDrawMessage, GMDrawAnswer};
use crate::GMError;
use crate::movement::{GMMovementT, GMMovementInner, GMMovementMessage, GMMovementAnswer};
use crate::texture::GMTexture;


#[derive(Debug)]
pub enum GMSpriteMessage {
    AddMovement(Box<dyn GMMovementT>),
    RemoveMovement(usize),
    SetMovement(usize, Box<dyn GMMovementT>),
    SetMovementActive(usize, bool),
    CustomMovementMessage(usize, GMMovementMessage),
    AddEffect(Box<dyn GMSpriteEffectT>),
    RemoveEffect(usize),
    SetEffect(usize, Box<dyn GMSpriteEffectT>),
    SetEffectActive(usize, bool),
    CustomEffectMessage(usize, GMSpriteEffectMessage),
    CustomContextProperty(String, Box<dyn Any>),
}

#[derive(Debug)]
pub struct GMSpriteInner {
    pub texture: Rc<GMTexture>,
    pub movement_inner: GMMovementInner,
    pub active: bool,
    pub animation: Box<dyn GMAnimationT>,
    pub z_index: i32,
    pub messages: Vec<GMSpriteMessage>,
    pub movement_answers: Vec<(usize, GMMovementAnswer)>,
    pub sprite_effect_answers: Vec<(usize, GMSpriteEffectAnswer)>,
}

impl Clone for GMSpriteInner {
    fn clone(&self) -> Self {
        Self {
            texture: self.texture.clone(),
            movement_inner: self.movement_inner.clone(),
            active: self.active.clone(),
            animation: self.animation.clone(),
            z_index: self.z_index.clone(),
            messages: Vec::new(), // Don't clone messages
            movement_answers: Vec::new(), // Don't clone answers
            sprite_effect_answers: Vec::new(), // Don't clone answers
        }
    }
}

impl GMSpriteInner {
    pub fn new(texture: Rc<GMTexture>, movement_inner: GMMovementInner, active: bool, animation: Box<dyn GMAnimationT>) -> Self {
        Self {
            texture,
            movement_inner,
            active,
            animation,
            z_index: 0,
            messages: Vec::new(),
            movement_answers: Vec::new(),
            sprite_effect_answers: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.animation.update();
    }

    pub fn draw(&self, context: &mut GMContext) {
        let index = self.animation.frame_index();
        let x = self.movement_inner.x;
        let y = self.movement_inner.y;

        self.texture.draw(x, y, index, context);
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

#[derive(Debug)]
pub struct GMSprite {
    pub sprite_inner: GMSpriteInner,
    pub movements: Vec<Box<dyn GMMovementT>>,
    pub effects: Vec<Box<dyn GMSpriteEffectT>>,
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
            effects: vec![Box::new(GMSpriteEffectStatic::new())],
        }
    }
}

impl GMDrawT for GMSprite {
    fn update(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        if self.sprite_inner.active {
            self.sprite_inner.update();

            for movement in self.movements.iter_mut() {
                movement.update(&mut self.sprite_inner.movement_inner, context);
            }

            for effect in self.effects.iter_mut() {
                effect.update(&mut self.sprite_inner, context);
            }

            self.sprite_inner.movement_answers.clear();
            self.sprite_inner.sprite_effect_answers.clear();
            self.sprite_inner.messages.reverse();
            while let Some(message) = self.sprite_inner.messages.pop() {
                match message {
                    GMSpriteMessage::AddMovement(new_movement) => {
                        self.movements.push(new_movement);
                    }
                    GMSpriteMessage::RemoveMovement(index) => {
                        self.movements.remove(index);
                    }
                    GMSpriteMessage::SetMovement(index, new_movement) => {
                        self.movements[index] = new_movement;
                    }
                    GMSpriteMessage::SetMovementActive(index, active) => {
                        self.movements[index].send_message(GMMovementMessage::SetActive(active))?;
                    }
                    GMSpriteMessage::CustomMovementMessage(index, message) => {
                        let answer = self.movements[index].send_message(message)?;
                        self.sprite_inner.movement_answers.push((index, answer));
                    }
                    GMSpriteMessage::AddEffect(new_effect) => {
                        self.effects.push(new_effect);
                    }
                    GMSpriteMessage::RemoveEffect(index) => {
                        self.effects.remove(index);
                    }
                    GMSpriteMessage::SetEffect(index, new_effect) => {
                        self.effects[index] = new_effect;
                    }
                    GMSpriteMessage::SetEffectActive(index, active) => {
                        self.effects[index].set_active(active);
                    }
                    GMSpriteMessage::CustomEffectMessage(index, message) => {
                        let answer = self.effects[index].send_message(message)?;
                        self.sprite_inner.sprite_effect_answers.push((index, answer));
                    }
                    GMSpriteMessage::CustomContextProperty(_name, _value) => {
                        // TODO: add message to context
                        todo!();
                    }
                }
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

    fn get_z_index(&self) -> i32 {
        self.sprite_inner.z_index
    }

    fn set_z_index(&mut self, z_index: i32) {
        self.sprite_inner.z_index = z_index;
    }

    fn get_movement_inner_ref(&self) -> &GMMovementInner {
        &self.sprite_inner.movement_inner
    }

    fn get_movement_inner_mut_ref(&mut self) -> &mut GMMovementInner {
        &mut self.sprite_inner.movement_inner
    }

    fn box_clone(&self) -> Box<dyn GMDrawT> {
        let result = GMSprite {
            sprite_inner: self.sprite_inner.clone(),
            movements: self.movements.clone(),
            effects: self.effects.clone(),
        };

        Box::new(result)
    }

    fn send_message(&mut self, message: GMDrawMessage) -> Result<GMDrawAnswer, GMError> {
        match message {
            GMDrawMessage::GetMovementsRef => {
                Ok(GMDrawAnswer::MovementsRef(&self.movements))
            }
            GMDrawMessage::GetMovementsMutRef => {
                Ok(GMDrawAnswer::MovementsMutRef(&mut self.movements))
            }
            GMDrawMessage::GetSpriteInnerRef => {
                Ok(GMDrawAnswer::SpriteInnerRef(&self.sprite_inner))
            }
            GMDrawMessage::GetSpriteInnerMutRef => {
                Ok(GMDrawAnswer::SpriteInnerMutRef(&mut self.sprite_inner))
            }
            GMDrawMessage::GetSpriteEffectsRef => {
                Ok(GMDrawAnswer::SpriteEffectsRef(&self.effects))
            }
            GMDrawMessage::GetSpriteEffectsMutRef => {
                Ok(GMDrawAnswer::SpriteEffectsMutRef(&mut self.effects))
            }
            _ => {
                Err(GMError::UnexpectedDrawMessage(message))
            }
        }
    }
}

#[derive(Debug)]
pub enum GMSpriteEffectMessage {
    DoSomeStuff,
}

#[derive(Debug)]
pub enum GMSpriteEffectAnswer {
    None,

}

pub trait GMSpriteEffectT {
    fn update(&mut self, sprite_inner: &mut GMSpriteInner, context: &mut GMContext);

    fn draw(&self, sprite_inner: &GMSpriteInner, context: &mut GMContext);

    fn set_active(&mut self, active: bool);

    fn box_clone(&self) -> Box<dyn GMSpriteEffectT>;

    fn send_message(&mut self, message: GMSpriteEffectMessage) -> Result<GMSpriteEffectAnswer, GMError>;
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

impl GMSpriteEffectStatic {
    pub fn new() -> Self {
        Self {
            active: true,
        }
    }
}

impl GMSpriteEffectT for GMSpriteEffectStatic {
    fn update(&mut self, _sprite_inner: &mut GMSpriteInner, _context: &mut GMContext) {}

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

    fn send_message(&mut self, message: GMSpriteEffectMessage) -> Result<GMSpriteEffectAnswer, GMError> {
        match message {
            _ => {
                Ok(GMSpriteEffectAnswer::None)
            }
        }
    }
}
