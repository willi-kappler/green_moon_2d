
use std::rc::Rc;
use std::fmt::{self, Debug, Formatter};

use crate::animation::{GMAnimationT};
use crate::context::GMContext;
use crate::draw_object::{GMDrawT, GMDrawRefType, GMDrawMutRefType};
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
}

#[derive(Debug)]
pub struct GMSpriteInner {
    pub texture: Rc<GMTexture>,
    pub movement_inner: GMMovementInner,
    pub active: bool,
    pub animation: Box<dyn GMAnimationT>,
    pub z_index: i32,
    pub group: u64,
    pub state: u64,
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
            group: self.group,
            state: self.state,
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
            group: 0,
            state: 0,
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

#[derive(Debug, Clone)]
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

    pub fn update(&mut self, context: &mut GMContext) -> Result<(), GMError> {
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
                }
            }
        }

        Ok(())
    }

    pub fn draw(&self, context: &mut GMContext) {
        if self.sprite_inner.active {
            for effect in self.effects.iter() {
                effect.draw(&self.sprite_inner, context);
            }
        }
    }

    pub fn get_z_index(&self) -> i32 {
        self.sprite_inner.z_index
    }

    pub fn set_z_index(&mut self, z_index: i32) {
        self.sprite_inner.z_index = z_index;
    }

    pub fn get_movement_inner_ref(&self) -> &GMMovementInner {
        &self.sprite_inner.movement_inner
    }

    pub fn get_movement_inner_mut_ref(&mut self) -> &mut GMMovementInner {
        &mut self.sprite_inner.movement_inner
    }
}

pub struct GMSpriteObject {
    pub sprite: GMSprite,
}

impl GMSpriteObject {
    pub fn new(sprite: GMSprite) -> Self {
        Self {
            sprite
        }
    }
}

impl GMDrawT for GMSpriteObject {
    fn update(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        self.sprite.update(context)
    }

    fn draw(&self, context: &mut GMContext) {
        self.sprite.draw(context)
    }

    fn get_z_index(&self) -> i32 {
        self.sprite.get_z_index()
    }

    fn set_z_index(&mut self, z_index: i32) {
        self.sprite.set_z_index(z_index);
    }

    fn box_clone(&self) -> Box<dyn GMDrawT> {
        let result = GMSpriteObject {
            sprite: self.sprite.clone(),
        };

        Box::new(result)
    }

    fn cast_ref(&self) -> GMDrawRefType {
        GMDrawRefType::Sprite(&self.sprite)
    }

    fn cast_mut_ref(&mut self) -> GMDrawMutRefType {
        GMDrawMutRefType::Sprite(&mut self.sprite)
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
