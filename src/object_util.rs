

use crate::context::GMContext;
use crate::object::{GMMessage, GMValue, GMObjectT, GMObjectManager};
use crate::timer::GMTimer;

#[derive(Clone, Debug)]
pub struct GMForewardToElement {
    pub target: String,
    pub element: usize,
}

impl GMForewardToElement {
    pub fn new(target: String, element: usize) -> Self {
        Self { target, element }
    }
}

impl GMObjectT for GMForewardToElement {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }
            GMMessage::SetElement(element) => {
                self.element = element
            }
            GMMessage::GetTarget => {
                return GMValue::Target(self.target.clone())
            }
            GMMessage::GetElement => {
                return GMValue::Element(self.element)
            }
            _ => {
                return object_manager.send_message(&self.target, GMMessage::ToElement(self.element, Box::new(message)), context)
            }
        }

        GMValue::None
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMultiTarget {
    pub targets: Vec<String>,
}

impl GMMultiTarget {
    pub fn new(targets: Vec<String>) -> Self {
        Self { targets }
    }
}

impl GMObjectT for GMMultiTarget {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTargets(targets) => {
                self.targets = targets
            }
            GMMessage::GetTargets => {
                return GMValue::Targets(self.targets.clone())
            }
            _ => {
                for target in self.targets.iter() {
                    object_manager.send_message(target, message.clone(), context);
                }
            }
        }

        GMValue::None
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

