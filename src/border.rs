

use crate::object::GMObjectT;
use crate::message::GMMessage;
use crate::value::GMValue;
use crate::object_manager::GMObjectManager;
use crate::context::GMContext;
use crate::math::{GMVec2D, GMRectangle};

#[derive(Debug, Clone)]
pub struct GMBorderBase {
    pub rectangle: GMRectangle,
}

impl GMObjectT for GMBorderBase {
    fn send_message(&mut self, message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        GMValue::None
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
