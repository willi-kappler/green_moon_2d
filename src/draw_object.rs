


use std::any::Any;

use crate::movement::GMMovementInner;
use crate::sprite::GMSpriteInner;
use crate::animation::GMAnimationT;
use crate::GMError;


#[derive(Debug)]
pub enum GMDrawMessage {
    GetMovementInnerRef,
    GetMovementInnerMutRef,

    GetSpriteInnerRef,
    GetSpriteInnerMutRef,

    GetAnimationRef,
    GetAnimationMutRef,

    CustomProperty(String, Box<dyn Any>),
}

#[derive(Debug)]
pub enum GMDrawAnswer<'a> {
    None,

    MovementInnerRef(&'a GMMovementInner),
    MovementInnerMutRef(&'a mut GMMovementInner),

    SpriteInnerRef(&'a GMSpriteInner),
    SpriteInnerMutRef(&'a mut GMSpriteInner),

    AnimationRef(&'a Box<dyn GMAnimationT>),
    AnimationMutRef(&'a mut Box<dyn GMAnimationT>),

    CustomProperty(String, Box<dyn Any>),
}


pub trait GMDrawT {
    fn update(&mut self) -> Result<(), GMError> {
        Ok(())
    }

    fn draw(&self);

    fn get_z_index(&self) -> i32 {
        0
    }

    fn box_clone(&self) -> Box<dyn GMDrawT>;

    fn send_message(&mut self, message: GMDrawMessage) -> Result<GMDrawAnswer, GMError>;
}

impl Clone for Box<dyn GMDrawT> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
