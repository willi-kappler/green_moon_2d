use std::any::Any;
use std::cell::RefCell;

use crate::GMError;
use crate::context::{GMUpdateContext, GMDrawContext};
use crate::math::GMVec2D;

pub enum GMObjectMessage {
    SetPosition(GMVec2D),
    AddPosition(GMVec2D),
    GetPosition(GMVec2D),

    Custom(Box<dyn Any>),
}

pub enum GMObjectAnswer {
    None,
    Position(GMVec2D),

    Custom(Box<dyn Any>),
}

pub trait GMObjectT {
    fn get_z_index(&self) -> i32;

    fn get_name(&self) -> &str;

    fn update(&mut self, context: &mut GMUpdateContext);

    fn draw(&self, context: &mut GMDrawContext);

    fn send_message(&mut self, message: GMObjectMessage) -> GMObjectAnswer;
}


pub struct GMObjectManager {
    objects: Vec<RefCell<Box<dyn GMObjectT>>>,
}

impl GMObjectManager {
    pub(crate) fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub(crate) fn update(&self, context: &mut GMUpdateContext) {

    }


    pub(crate) fn draw(&self, context: &mut GMDrawContext) {

    }

    pub fn send_message(&mut self, receiver: &str, message: GMObjectMessage) -> Result<GMObjectAnswer, GMError> {
        todo!();
    }
}