
use log::debug;

use crate::context::GMContext;
use crate::object::{GMMessage, GMValue, GMObjectT, GMObjectManager, GMTarget};
use crate::timer::GMTimer;
use crate::util::{error_panic};


#[derive(Clone, Debug)]
pub struct GMForewardToElement {
    pub target: GMTarget,
    pub elements: Vec<GMValue>,
}

impl GMForewardToElement {
    pub fn new<T: Into<GMTarget>>(target: T, elements: Vec<usize>) -> Self {
        let target = target.into();
        debug!("GMForewardToElement::new(), target: {:?}", target);

        Self {
            target,
            elements: elements.iter().map(|e| GMValue::USize(*e)).collect(),
        }
    }
}

impl GMObjectT for GMForewardToElement {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }
            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::Custom1(name) if name == "get_element_indices" => {
                let result: GMValue = self.elements.clone().into();
                return result
            }
            GMMessage::Custom2(name, GMValue::Multiple(values)) if name == "set_element_indices" => {
                self.elements.clear();

                for element in values {
                    if let GMValue::USize(_) = element {
                        self.elements.push(element);
                    }
                }
            }
            _ => {
                if self.elements.is_empty() {
                    let new_message = GMMessage::Custom2("to_all_elements".to_string(), message.into());
                    return object_manager.send_message(&self.target, new_message, context);
                } else {
                    let mut new_messages = Vec::new();

                    for element in self.elements.iter() {
                        let value2 = message.clone().into();
                        let new_message = ("to_element_n", (element.clone(), value2).into()).into();
                        new_messages.push(new_message);
                    }

                    return object_manager.send_message(&self.target, new_messages.into(), context);
                }
            }
        }

        GMValue::None
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMOtherTarget {
    pub target: GMTarget,
}

impl GMOtherTarget {
    pub fn new<T: Into<GMTarget>>(target: T) -> Self {
        let target = target.into();
        debug!("GMOtherTarget::new(), target: {:?}", target);

        Self {
            target,
        }
    }
}

impl GMObjectT for GMOtherTarget {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }
            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            _ => {
                return object_manager.send_message(&self.target, message, context);
            }
        }

        GMValue::None
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMTimedMessage {
    pub message: GMMessage,
    pub target: GMTarget,
    pub timer: GMTimer,
    pub repeat: bool,
}

impl GMTimedMessage {
    pub fn new<T: Into<GMTarget>>(message: GMMessage, target: T, timeout: f32, repeat: bool) -> Self {
        let target = target.into();
        debug!("GMTimedMessage::new(), target: {:?}, timeout: {}, repeat: {}, message: {:?}", target, timeout, repeat, message);

        Self {
            message,
            target,
            timer: GMTimer::new(timeout),
            repeat
        }
    }
}

impl GMObjectT for GMTimedMessage {
    fn send_message(&mut self, message: GMMessage, _context: &mut GMContext, _object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetMessage(message) => {
                self.message = *message;
            }
            GMMessage::SetTarget(target) => {
                self.target = target;
            }
            GMMessage::GetMessage => {
                return self.message.clone().into()
            }
            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::Custom1(name) if name == "get_timeout" => {
                let value = self.timer.duration.into();
                return value
            }
            GMMessage::Custom1(name) if name == "get_repeat" => {
                let value = self.repeat.into();
                return value
            }
            GMMessage::Custom2(name, GMValue::F32(value)) if name == "set_timeout" => {
                self.timer.duration = value;
            }
            GMMessage::Custom2(name, GMValue::Bool(value)) if name == "set_repeat" => {
                self.repeat = value;
            }
            _ => {
                error_panic(&format!("Wrong message for GMTimedMessage::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        if self.timer.finished() {
            if self.repeat {
                self.timer.start();
            }

            object_manager.send_message(&self.target, self.message.clone(), context);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }

}

#[derive(Clone, Debug)]
pub struct GMTrigger {
    pub message: GMMessage,
    pub target: GMTarget,
}

impl GMTrigger {
    pub fn new<T: Into<GMTarget>>(target: T, message: GMMessage) -> Self {
        let target = target.into();
        debug!("GMTrigger::new(), target: {:?}, message: {:?}", target, message);

        Self {
            target: target.into(),
            message,
        }
    }
}

impl GMObjectT for GMTrigger {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetMessage(messages) => {
                self.message = *messages
            }
            GMMessage::SetTarget(targets) => {
                self.target = targets
            }
            GMMessage::GetMessage => {
                return self.message.clone().into()
            }
            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::Custom1(name) if name == "trigger" => {
                return object_manager.send_message(&self.target, self.message.clone(), context)
            }
            _ => {
                error_panic(&format!("Wrong message for GMTrigger::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}



#[derive(Clone, Debug)]
pub struct GMTriggerPair {
    pub pairs: Vec<(GMTarget, GMMessage)>,
}

impl GMTriggerPair {
    pub fn new(pairs: Vec<(GMTarget, GMMessage)>) -> Self {
        debug!("GMTriggerPair::new()");

        Self {
            pairs
        }
    }
}

impl GMObjectT for GMTriggerPair {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Custom1(name) if name == "trigger" => {
                let mut result = Vec::new();

                for (target, message) in self.pairs.iter() {
                    result.push(object_manager.send_message(&target, message.clone(), context));
                }

                return result.into()
            }
            _ => {
                error_panic(&format!("Wrong message for GMTriggerPair::send_message: {:?}", message))
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMultiply {
    pub factor: u32,
    pub target: GMTarget,
}

impl GMMultiply {
    pub fn new<T: Into<GMTarget>>(target: T, factor: u32) -> Self {
        let target = target.into();
        debug!("GMMultiply::new(), target: {:?}, factor: {}", target, factor);

        Self {
            factor,
            target: target.into(),
        }
    }
}

impl GMObjectT for GMMultiply {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }

            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::Custom1(name) if name == "get_factor" => {
                let value = self.factor.into();
                return value
            }
            GMMessage::Custom2(name, GMValue::U32(value)) if name == "set_factor" => {
                self.factor = value;
            }
            _ => {
                let mut result = Vec::new();

                for _ in 0..self.factor {
                    result.push(message.clone());
                }

                return object_manager.send_message(&self.target, result.into(), context)
            }
        }

        GMValue::None
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
