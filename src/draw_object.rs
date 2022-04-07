


use std::any::Any;

use crate::movement::GMMovementInner;
use crate::sprite::GMSpriteInner;
use crate::GMError;


#[derive(Debug)]
pub enum GMDrawMessage {
    GetMovementInner,
    GetMovementInnerRef,
    GetMovementInnerMutRef,

    GetSpriteInner,
    GetSpriteInnerRef,
    GetSpriteInnerMutRef,

    CustomProperty(String, Box<dyn Any>),
}

#[derive(Debug)]
pub enum GMDrawAnswer<'a> {
    None,

    MovementInner(GMMovementInner),
    MovementInnerRef(&'a GMMovementInner),
    MovementInnerMutRef(&'a mut GMMovementInner),

    SpriteInner(GMSpriteInner),
    SpriteInnerRef(&'a GMSpriteInner),
    SpriteInnerMutRef(&'a mut GMSpriteInner),

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
