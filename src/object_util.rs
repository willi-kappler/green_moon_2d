

use crate::context::GMContext;
use crate::object::{GMMessage, GMValue, GMObjectT, GMObjectManager, GMTarget};
use crate::timer::GMTimer;
use crate::util::{error_panic};


#[derive(Clone, Debug)]
pub struct GMForewardToElement {
    pub target: GMTarget,
    pub elements: Vec<usize>,
}

impl GMForewardToElement {
    pub fn new<T: Into<GMTarget>>(target: T, elements: Vec<usize>) -> Self {
        Self {
            target: target.into(),
            elements,
        }
    }
}

impl GMObjectT for GMForewardToElement {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetTarget(target) => {
                self.target = target
            }
            GMMessage::SetElementIndices(elements) => {
                self.elements = elements
            }
            GMMessage::GetTarget => {
                return GMValue::Target(self.target.clone())
            }
            GMMessage::GetElementIndices => {
                return GMValue::ElementIndices(self.elements.clone())
            }
            _ => {
                if self.elements.is_empty() {
                    return object_manager.send_message(self.target.clone(), GMMessage::ToAllElements(Box::new(message)), context);
                } else {
                    let mut new_message = Vec::new();

                    for element in self.elements.iter() {
                        new_message.push(GMMessage::ToElementN(*element, Box::new(message.clone())));
                    }
    
                    return object_manager.send_message(self.target.clone(), GMMessage::Multiple(new_message), context);    
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
        Self {
            target: target.into(),
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
                return GMValue::Target(self.target.clone())
            }
            _ => {
                object_manager.send_message(self.target.clone(), message, context);
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
    pub fn new<T: Into<GMTarget>>(message: GMMessage, targets: T, timeout: f32, repeat: bool) -> Self {
        Self {
            message,
            target: targets.into(),
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
            GMMessage::SetTimeout(timeout) => {
                self.timer.duration = timeout;
            }
            GMMessage::SetRepeat(repeat) => {
                self.repeat = repeat;
            }
            GMMessage::GetMessage => {
                return GMValue::Message(Box::new(self.message.clone()))
            }
            GMMessage::GetTarget => {
                return GMValue::Target(self.target.clone())
            }
            GMMessage::GetTimeout => {
                return GMValue::Timeout(self.timer.duration)
            }
            GMMessage::GetRepeat => {
                return GMValue::Repeat(self.repeat)
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

            object_manager.send_message(self.target.clone(), self.message.clone(), context);
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
                return GMValue::Message(Box::new(self.message.clone()))
            }
            GMMessage::GetTarget => {
                return GMValue::Target(self.target.clone())
            }
            GMMessage::Trigger => {
                object_manager.send_message(self.target.clone(), message, context);
                /*
                for target in self.targets.iter() {
                    object_manager.send_multi_message(target, self.messages.clone(), context);
                }
                */
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
        Self {
            pairs
        }
    }
}

impl GMObjectT for GMTriggerPair {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Trigger => {
                for (target, message) in self.pairs.iter() {
                    object_manager.send_message(target.clone(), message.clone(), context);
                }
            }
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
