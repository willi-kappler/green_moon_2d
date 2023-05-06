
use std::fmt;
use std::rc::Rc;

use log::debug;

use crate::context::GMContext;
use crate::interpolation::{GMInterpolateF32, GMInterpolateVec2D, GMCurveT};
use crate::message::GMMessage;
use crate::object_manager::GMObjectManager;
use crate::object::GMObjectT;
use crate::target::GMTarget;
use crate::timer::GMTimer;
use crate::util::{error_panic, GMRepetition};
use crate::value::GMValue;

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

#[derive(Clone)]
pub struct GMTimedFunc {
    pub timer: GMTimer,
    pub repeat: bool,
    pub func: fn(context: &mut GMContext, object_manager: &GMObjectManager),
}

impl GMTimedFunc {
    pub fn new(timeout: f32, repeat: bool, func: fn(context: &mut GMContext, object_manager: &GMObjectManager)) -> Self {
        debug!("GMTimedFunc::new(), timeout: {}, repeat: {}", timeout, repeat);

        Self {
            timer: GMTimer::new(timeout),
            repeat,
            func,
        }
    }
}

impl fmt::Debug for GMTimedFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMTimedFunc, timeout: {}, repeat: {}", self.timer.duration, self.repeat)
    }
}

impl GMObjectT for GMTimedFunc {
    fn send_message(&mut self, message: GMMessage, _context: &mut GMContext, _object_manager: &GMObjectManager) -> GMValue {
        match message {
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

            (self.func)(context, object_manager);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct GMTrigger {
    pub func: fn(context: &mut GMContext, object_manager: &GMObjectManager),
}

impl GMTrigger {
    pub fn new(func: fn(context: &mut GMContext, object_manager: &GMObjectManager)) -> Self {
        debug!("GMTrigger::new()");

        Self {
            func,
        }
    }
}

impl fmt::Debug for GMTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMTrigger")
    }
}

impl GMObjectT for GMTrigger {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Custom0(name) if name == "trigger" => {
                (self.func)(context, object_manager)
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_func" => {
                let func = *value.downcast::<fn(context: &mut GMContext, object_manager: &GMObjectManager)>().unwrap();
                self.func = func;
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

#[derive(Clone)]
pub struct GMValueInterpolateF32 {
    pub interpolation: GMInterpolateF32,
    pub func: fn(value: f32, context: &mut GMContext, object_manager: &GMObjectManager),
    pub auto_update: bool,
}

impl GMValueInterpolateF32 {
    pub fn new(start: f32, end: f32, speed: f32, func: fn(value: f32, context: &mut GMContext, object_manager: &GMObjectManager)) -> Self {
        debug!("GMValueInterpolateF32::new(), start: {}, end: {}, speed: {}", start, end, speed);

        let interpolation = GMInterpolateF32::new(start, end, speed, 0.0);

        Self {
            interpolation,
            func,
            auto_update: true,
        }
    }
}

impl fmt::Debug for GMValueInterpolateF32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMValueInterpolateF32, start: {}, end: {}, speed: {}", self.interpolation.start, self.interpolation.end, self.interpolation.speed)
    }
}

impl GMObjectT for GMValueInterpolateF32 {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Custom0(name) if name == "get_start" => {
                return self.interpolation.start.into()
            }
            GMMessage::Custom0(name) if name == "get_end" => {
                return self.interpolation.end.into()
            }
            GMMessage::Custom0(name) if name == "get_speed" => {
                return self.interpolation.speed.into()
            }
            GMMessage::Custom0(name) if name == "get_step" => {
                return self.interpolation.current_step.into()
            }
            GMMessage::Custom0(name) if name == "get_value" => {
                return self.interpolation.get_current_value().into()
            }
            GMMessage::Custom0(name) if name == "get_repetition" => {
                return GMValue::Any(Rc::new(self.interpolation.repetition))
            }
            GMMessage::Custom0(name) if name == "get_curve" => {
                return GMValue::Any(Rc::new(self.interpolation.curve.clone()))
            }
            GMMessage::Custom0(name) if name == "reset" => {
                self.interpolation.reset();
            }
            GMMessage::Custom0(name) if name == "is_finished" => {
                return self.interpolation.is_finished().into()
            }
            GMMessage::Custom0(name) if name == "update" => {
                self.interpolation.update();
                let value = self.interpolation.get_current_value();
                (self.func)(value, context, object_manager);
            }
            GMMessage::Custom1(name, GMValue::F32(start)) if name == "set_start" => {
                self.interpolation.start = start;
            }
            GMMessage::Custom1(name, GMValue::F32(end)) if name == "set_end" => {
                self.interpolation.end = end;
            }
            GMMessage::Custom1(name, GMValue::F32(speed)) if name == "set_speed" => {
                self.interpolation.speed = speed;
            }
            GMMessage::Custom1(name, GMValue::F32(step)) if name == "set_step" => {
                self.interpolation.current_step = step;
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_repetition" => {
                let repetition = value.downcast::<GMRepetition>().unwrap();
                self.interpolation.repetition = (*repetition).clone();
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_curve" => {
                let curve = value.downcast::<Box<dyn GMCurveT>>().unwrap();
                self.interpolation.curve = (*curve).clone();
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_func" => {
                let func = *value.downcast::<fn(value: f32, context: &mut GMContext, object_manager: &GMObjectManager)>().unwrap();
                self.func = func;
            }
            _ => {
                error_panic(&format!("Wrong message for GMValueInterpolateF32::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        if self.auto_update {
            self.interpolation.update();
            let value = self.interpolation.get_current_value();
            (self.func)(value, context, object_manager);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

pub struct GMMapMessage {
    pub target: GMTarget,
}

pub struct GMMatchMessage {
    pub target: GMTarget,
}
