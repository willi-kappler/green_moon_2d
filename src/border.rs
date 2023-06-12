

use crate::object::GMObjectT;
use crate::message::GMMessage;
use crate::value::GMValue;
use crate::object_manager::GMObjectManager;
use crate::context::GMContext;
use crate::math::GMVec2D;

pub enum GMBorderTiles {
    Tile1(Box<dyn GMObjectT>),
    Tile4,
    Tile9,
}

#[derive(Debug, Clone)]
pub struct GMBorder {
    top_left: GMVec2D,
    width: u32,
    height: u32,
}

impl GMObjectT for GMBorder {
    fn send_message(&mut self, message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager, context: &mut GMContext) {
    }

    fn draw(&self, context: &mut GMContext) {
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
