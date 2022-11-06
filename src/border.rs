

use crate::object_manager::GMObjectBaseT;
use crate::context::GMContext;
use crate::data::GMData;
use crate::util::error_panic;

pub struct GMBorderBase {

}

impl GMBorderBase {

}

impl GMObjectBaseT for GMBorderBase {
    fn send_message(&mut self, message: &str, data: GMData, context: &mut crate::GMContext) {
        match message {
            "" => {
                todo!()
            }
            _ => {
                error_panic(&format!("GMBorderBase::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn groups(&self) -> &std::collections::HashSet<String> {
        todo!()
    }

    fn update(&mut self, _context: &mut GMContext) {
    }

    fn draw(&self, _context: &mut GMContext) {
    }

    fn send_message2(&mut self, message: &str, context: &mut GMContext) {
        self.send_message(message, GMData::None, context);
    }
}

pub struct GMBorder {

}

pub struct GMBorderBuilder {

}
