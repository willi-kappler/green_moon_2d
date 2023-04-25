

use crate::context::GMContext;
use crate::object::{GMMessage, GMValue, GMObjectT, GMObjectManager};
use crate::timer::GMTimer;
use crate::util::{error_panic};


#[derive(Clone, Debug)]
pub struct GMForewardToElement {
    pub target: String,
    pub element: usize,
}

impl GMForewardToElement {
    pub fn new(target: String, element: usize) -> Self {
        Self {
            target,
            element
        }
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
                return object_manager.send_message(&self.target, GMMessage::ToElementN(self.element, vec![message]), context)
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
        Self {
            targets
        }
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

#[derive(Clone, Debug)]
pub struct GMTimedMessage {
    pub messages: Vec<GMMessage>,
    pub targets: Vec<String>,
    pub timer: GMTimer,
    pub repeat: bool,
}

impl GMTimedMessage {
    pub fn new(messages: Vec<GMMessage>, targets: Vec<String>, timeout: f32, repeat: bool) -> Self {
        Self {
            messages,
            targets,
            timer: GMTimer::new(timeout),
            repeat
        }
    }
}

impl GMObjectT for GMTimedMessage {
    fn send_message(&mut self, message: GMMessage, _context: &mut GMContext, _object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetMessages(messages) => {
                self.messages = messages
            }
            GMMessage::SetTargets(targets) => {
                self.targets = targets
            }
            GMMessage::SetTimeout(timeout) => {
                self.timer.duration = timeout;
            }
            GMMessage::SetRepeat(repeat) => {
                self.repeat = repeat
            }
            GMMessage::GetMessages => {
                return GMValue::Messages(self.messages.clone())
            }
            GMMessage::GetTargets => {
                return GMValue::Targets(self.targets.clone())
            }
            GMMessage::GetTimeout => {
                return GMValue::Timeout(self.timer.duration)
            }
            GMMessage::GetRepeat => {
                return GMValue::Repeat(self.repeat)
            }
            _ => {
                error_panic(&format!("Wrong message for GMTimedMessage::send_message: {:?}", message))            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        if self.timer.finished() {
            if self.repeat {
                self.timer.start();
            }

            for target in self.targets.iter() {
                object_manager.send_multi_message(target, self.messages.clone(), context);
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }

}

#[derive(Clone, Debug)]
pub struct GMTrigger {
    pub messages: Vec<GMMessage>,
    pub targets: Vec<String>,
}

impl GMTrigger {
    pub fn new(messages: Vec<GMMessage>, targets: Vec<String>) -> Self {
        Self {
            messages,
            targets
        }
    }
}

impl GMObjectT for GMTrigger {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetMessages(messages) => {
                self.messages = messages
            }
            GMMessage::SetTargets(targets) => {
                self.targets = targets
            }
            GMMessage::GetMessages => {
                return GMValue::Messages(self.messages.clone())
            }
            GMMessage::GetTargets => {
                return GMValue::Targets(self.targets.clone())
            }
            GMMessage::Trigger => {
                for target in self.targets.iter() {
                    object_manager.send_multi_message(target, self.messages.clone(), context);
                }
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
    pub messages: Vec<GMMessage>,
    pub targets: Vec<String>,
}

impl GMTriggerPair {
    pub fn new(messages: Vec<GMMessage>, targets: Vec<String>) -> Self {
        Self {
            messages,
            targets
        }
    }
}

impl GMObjectT for GMTriggerPair {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetMessages(messages) => {
                self.messages = messages
            }
            GMMessage::SetTargets(targets) => {
                self.targets = targets
            }
            GMMessage::GetMessages => {
                return GMValue::Messages(self.messages.clone())
            }
            GMMessage::GetTargets => {
                return GMValue::Targets(self.targets.clone())
            }
            GMMessage::Trigger => {
                for (message, target) in self.messages.iter().zip(self.targets.iter()) {
                    object_manager.send_message(&target, message.clone(), context);
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
