
use log::debug;

use crate::context::GMContext;
use crate::object::GMObjectT;
use crate::timer::GMTimer;
use crate::util::{error_panic};
use crate::message::GMMessage;
use crate::value::GMValue;
use crate::target::GMTarget;
use crate::object_manager::GMObjectManager;


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

    fn forward_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        if self.elements.is_empty() {
            let new_message = GMMessage::ToAllChildren(message.into());
            return object_manager.send_message(&self.target, new_message, context);
        } else {
            let mut new_messages = Vec::new();

            for value in self.elements.iter() {
                if let GMValue::USize(element) = value {
                    let value2 = message.clone().into();
                    let new_message = GMMessage::ToChild(*element, value2);
                    new_messages.push(new_message);
                }
            }

            return object_manager.send_message(&self.target, new_messages.into(), context);
        }
    }

    fn keep_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }
            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::Custom0(name) if name == "get_element_indices" => {
                let result: GMValue = self.elements.clone().into();
                return result
            }
            GMMessage::Custom1(name, GMValue::Multiple(values)) if name == "set_element_indices" => {
                self.elements.clear();

                for element in values {
                    if let GMValue::USize(_) = element {
                        self.elements.push(element);
                    }
                }
            }
            GMMessage::Multiple(messages) => {
                let mut result = Vec::new();

                for message in messages.iter() {
                    result.push(self.keep_message(message.clone(), context, object_manager));
                }

                return result.into()
                    }
            _ => {
                error_panic(&format!("Wrong message for GMForewardToElement::send_message: {:?}", message))
            }
        }

        GMValue::None
    }
}

impl GMObjectT for GMForewardToElement {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Keep(self_message) => {
                self.keep_message(*self_message, context, object_manager)
            }
            GMMessage::Forward(fwd_message) => {
                self.forward_message(*fwd_message, context, object_manager)
            }
            _ => {
                self.forward_message(message, context, object_manager)
            }
        }
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

    fn keep_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }
            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::Multiple(messages) => {
                let mut result = Vec::new();

                for message in messages.iter() {
                    result.push(self.keep_message(message.clone(), context, object_manager));
                }

                return result.into()
                    }
            _ => {
                error_panic(&format!("Wrong message for GMOtherTarget::send_message: {:?}", message))
            }
        }

        GMValue::None
    }
}

impl GMObjectT for GMOtherTarget {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Keep(self_message) => {
                self.keep_message(*self_message, context, object_manager)
            }
            GMMessage:: Forward(fwd_message) => {
                object_manager.send_message(&self.target, *fwd_message, context)
            }
            _ => {
                object_manager.send_message(&self.target, message, context)
            }
        }
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
            GMMessage::Custom0(name) if name == "get_timeout" => {
                let value = self.timer.duration.into();
                return value
            }
            GMMessage::Custom0(name) if name == "get_repeat" => {
                let value = self.repeat.into();
                return value
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == "set_timeout" => {
                self.timer.duration = value;
            }
            GMMessage::Custom1(name, GMValue::Bool(value)) if name == "set_repeat" => {
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
            GMMessage::Custom0(name) if name == "trigger" => {
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
            GMMessage::Custom0(name) if name == "trigger" => {
                let mut result = Vec::new();

                for (target, message) in self.pairs.iter() {
                    result.push(object_manager.send_message(&target, message.clone(), context));
                }

                return result.into()
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_pairs" => {
                self.pairs = (*value.downcast::<Vec<(GMTarget, GMMessage)>>().unwrap()).clone();
            }
            // TODO: add "get_pairs"
            _ => {
                error_panic(&format!("Wrong message for GMTriggerPair::send_message: {:?}", message))
            }
        }

        GMValue::None
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

    fn forward_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        let mut result = Vec::new();

        for _ in 0..self.factor {
            result.push(message.clone());
        }

        return object_manager.send_message(&self.target, result.into(), context)
    }

    fn keep_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }

            GMMessage::GetTarget => {
                return self.target.clone().into()
            }
            GMMessage::Custom0(name) if name == "get_factor" => {
                let value = self.factor.into();
                return value
            }
            GMMessage::Custom1(name, GMValue::U32(value)) if name == "set_factor" => {
                self.factor = value;
            }
            GMMessage::Multiple(messages) => {
                let mut result = Vec::new();

                for message in messages.iter() {
                    result.push(self.keep_message(message.clone(), context, object_manager));
                }

                return result.into()
            }
            _ => {
                error_panic(&format!("Wrong message for GMForewardToElement::send_message: {:?}", message))
            }
        }

        GMValue::None
    }
}

impl GMObjectT for GMMultiply {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Keep(self_message) => {
                self.keep_message(*self_message, context, object_manager)
            }
            GMMessage:: Forward(fwd_message) => {
                self.forward_message(*fwd_message, context, object_manager)
            }
            _ => {
                self.forward_message(message, context, object_manager)
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
