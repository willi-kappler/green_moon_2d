
use std::rc::Rc;
use std::any::Any;

use crate::animation::{GMAnimationT};
use crate::draw_object::{GMDrawT, GMMessage, GMAnswer};
use crate::movement::{GMMovementT, GMMovementInner};
use crate::texture::GMTexture;


#[derive(Clone)]
pub enum GMSpriteCommand {
    AddMovement,
    RemoveMovement(usize),
    SetMovementActive(usize, bool),
    CustomMovementMessage(usize),
    AddEffect,
    RemoveEffect(usize),
    SetEffectActive(usize, bool),
    CustomEffectMessage(usize),
    CustomContextMessage,
}

pub struct GMSpriteInner {
    pub texture: Rc<GMTexture>,
    pub movement_inner: GMMovementInner,
    pub active: bool,
    pub animation: Box<dyn GMAnimationT>,
    pub z_index: i32,
    pub commands: Vec<GMSpriteCommand>,
}

impl Clone for GMSpriteInner {
    fn clone(&self) -> Self {
        Self {
            texture: self.texture.clone(),
            movement_inner: self.movement_inner.clone(),
            active: self.active,
            animation: self.animation.box_clone(),
            z_index: self.z_index,
            commands: self.commands.clone(),
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
            commands: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.animation.update();
    }

    pub fn draw(&self) {
        let index = self.animation.frame_index();
        let x = self.movement_inner.x;
        let y = self.movement_inner.y;

        self.texture.draw(x, y, index)
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

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
    fn update(&mut self) {
        if self.sprite_inner.active {
            self.sprite_inner.update();

            for movement in self.movements.iter_mut() {
                movement.update(&mut self.sprite_inner.movement_inner);
            }

            for effect in self.effects.iter_mut() {
                effect.update(&mut self.sprite_inner);
            }

            for command in self.sprite_inner.commands.iter() {
                match command {
                    GMSpriteCommand::AddMovement => {
                        todo!();
                    }
                    GMSpriteCommand::RemoveMovement(index) => {
                        self.movements.remove(*index);
                    }
                    GMSpriteCommand::SetMovementActive(index, active) => {
                        self.movements[*index].set_active(*active);
                    }
                    GMSpriteCommand::CustomMovementMessage(index) => {
                        todo!();
                    }
                    GMSpriteCommand::AddEffect => {
                        todo!();
                    }
                    GMSpriteCommand::RemoveEffect(index) => {
                        self.effects.remove(*index);
                    }
                    GMSpriteCommand::SetEffectActive(index, active) => {
                        self.effects[*index].set_active(*active);
                    }
                    GMSpriteCommand::CustomEffectMessage(index) => {
                        todo!();
                    }
                    GMSpriteCommand::CustomContextMessage => {
                        todo!();
                    }
                }
            }

            self.sprite_inner.commands.clear();
        }
    }

    fn draw(&self) {
        if self.sprite_inner.active {
            for effect in self.effects.iter() {
                effect.draw(&self.sprite_inner);
            }
        }
    }

    fn get_z_index(&self) -> i32 {
        self.sprite_inner.z_index
    }

    fn box_clone(&self) -> Box<dyn GMDrawT> {
        let result = GMSprite {
            sprite_inner: self.sprite_inner.clone(),
            movements: self.movements.iter().map(|m| m.box_clone()).collect(),
            effects: self.effects.iter().map(|e| e.box_clone()).collect(),
        };

        Box::new(result)
    }

    fn send_message1(&mut self, message: GMMessage) {
        todo!()
    }

    fn send_message2(&mut self, message: GMMessage) -> GMAnswer {
        todo!()
    }
}

pub trait GMSpriteEffectT {
    fn update(&mut self, sprite_inner: &mut GMSpriteInner);
    fn draw(&self, sprite_inner: &GMSpriteInner);
    fn set_active(&mut self, active: bool);
    fn box_clone(&self) -> Box<dyn GMSpriteEffectT>;
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

    fn draw(&self, sprite_inner: &GMSpriteInner) {
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
