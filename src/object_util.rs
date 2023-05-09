
use std::fmt;
use std::rc::Rc;

use log::debug;

use crate::context::GMContext;
use crate::curve::GMCurveT;
use crate::interpolation::{GMInterpolateF32, GMInterpolateVec2D};
use crate::math::GMVec2D;
use crate::message::GMMessage;
use crate::object_manager::GMObjectManager;
use crate::object::GMObjectT;
use crate::state::GMState;
use crate::target::GMTarget;
use crate::timer::GMTimer;
use crate::util::{error_panic, GMRepetition};
use crate::value::GMValue;


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
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
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
            GMMessage::Multiple(messages) => {
                self.send_multi_message(messages, context, object_manager);
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
pub struct GMTimedMultiMessage {
    pub items: Vec<(GMTimer, bool, GMTarget, GMMessage)>,
}

impl GMTimedMultiMessage {
    pub fn new(mut items: Vec<(f32, bool, GMTarget, GMMessage)>) -> Self {

        Self {
            items: items.drain(0..).map(|(duration, repeat, target, message)| (GMTimer::new(duration), repeat, target, message)).collect(),
        }
    }
}

impl GMObjectT for GMTimedMultiMessage {
    fn send_message(&mut self, message: GMMessage, _context: &mut GMContext, _object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Custom0(name) if name == "reset_all_timers" => {
                for item in self.items.iter_mut() {
                    item.0.start();
                }
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_items" => {
                let mut items = (*value.downcast::<Vec<(f32, bool, GMTarget, GMMessage)>>().unwrap()).clone();
                self.items = items.drain(0..).map(|(duration, repeat, target, message)| (GMTimer::new(duration), repeat, target, message)).collect();
            }
            GMMessage::Custom1(name, GMValue::USize(index)) if name == "reset_timer" => {
                self.items[index].0.start();
            }
            GMMessage::Custom2(name, GMValue::F32(duration), GMValue::USize(index)) if name == "set_timeout" => {
                self.items[index].0.duration = duration;
            }
            GMMessage::Custom2(name, GMValue::Bool(repeat), GMValue::USize(index)) if name == "set_repeat" => {
                self.items[index].1 = repeat;
            }
            GMMessage::Custom2(name, GMValue::Target(target), GMValue::USize(index)) if name == "set_target" => {
                self.items[index].2 = target;
            }
            GMMessage::Custom2(name, GMValue::Message(message), GMValue::USize(index)) if name == "set_message" => {
                self.items[index].3 = *message;
            }
            _ => {
                error_panic(&format!("Wrong message for GMTimedMultiMessage::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        for (timer, repeat, target, message) in self.items.iter_mut() {
            if timer.finished() {
                if *repeat {
                    timer.start();
                }

                object_manager.send_message(&target, message.clone(), context);
            }
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
    pub state: GMState,
    pub func: fn(context: &mut GMContext, object_manager: &GMObjectManager, state: &mut GMState),
}

impl GMTimedFunc {
    pub fn new(timeout: f32, repeat: bool, func: fn(context: &mut GMContext,
            object_manager: &GMObjectManager, state: &mut GMState)) -> Self {
        debug!("GMTimedFunc::new(), timeout: {}, repeat: {}", timeout, repeat);

        Self {
            timer: GMTimer::new(timeout),
            repeat,
            state: GMState::new(),
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
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
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
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_func" => {
                let func = *value.downcast::<fn(context: &mut GMContext,
                        object_manager: &GMObjectManager, state: &mut GMState)>().unwrap();
                self.func = func;
            }
            GMMessage::Multiple(messages) => {
                self.send_multi_message(messages, context, object_manager);
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

            (self.func)(context, object_manager, &mut self.state);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct GMTrigger {
    pub state: GMState,
    pub func: fn(context: &mut GMContext, object_manager: &GMObjectManager, state: &mut GMState),
}

impl GMTrigger {
    pub fn new(func: fn(context: &mut GMContext, object_manager: &GMObjectManager, state: &mut GMState)) -> Self {
        debug!("GMTrigger::new()");

        Self {
            state: GMState::new(),
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
                (self.func)(context, object_manager, &mut self.state)
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_func" => {
                let func = *value.downcast::<fn(context: &mut GMContext,
                    object_manager: &GMObjectManager, state: &mut GMState)>().unwrap();
                self.func = func;
            }
            GMMessage::Multiple(messages) => {
                self.send_multi_message(messages, context, object_manager);
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
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_single_pair" => {
                let (index, target, message) = (*value.downcast::<(usize, GMTarget, GMMessage)>().unwrap()).clone();
                self.pairs[index] = (target, message);
            }
            GMMessage::Multiple(messages) => {
                self.send_multi_message(messages, context, object_manager);
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

#[derive(Clone)]
pub struct GMValueInterpolateF32 {
    pub interpolation: GMInterpolateF32,
    pub auto_update: bool,
    pub state: GMState,
    pub func: fn(value: f32, context: &mut GMContext, object_manager: &GMObjectManager, state: &mut GMState),
}

impl GMValueInterpolateF32 {
    pub fn new(start: f32, end: f32, speed: f32, func: fn(value: f32, context: &mut GMContext,
            object_manager: &GMObjectManager, state: &mut GMState)) -> Self {
        debug!("GMValueInterpolateF32::new(), start: {}, end: {}, speed: {}", start, end, speed);

        let interpolation = GMInterpolateF32::new(start, end, speed, 0.0);

        Self {
            interpolation,
            auto_update: true,
            state: GMState::new(),
            func,
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
                (self.func)(value, context, object_manager, &mut self.state);
            }
            GMMessage::Custom0(name) if name == "calculate_diff" => {
                self.interpolation.calculate_diff();
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
                let func = *value.downcast::<fn(value: f32, context: &mut GMContext,
                    object_manager: &GMObjectManager, state: &mut GMState)>().unwrap();
                self.func = func;
            }
            GMMessage::Multiple(messages) => {
                self.send_multi_message(messages, context, object_manager);
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
            (self.func)(value, context, object_manager, &mut self.state);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct GMValueInterpolateVec2D {
    pub interpolation: GMInterpolateVec2D,
    pub auto_update: bool,
    pub state: GMState,
    pub func: fn(value: GMVec2D, context: &mut GMContext, object_manager: &GMObjectManager, state: &mut GMState),
}

impl GMValueInterpolateVec2D {
    pub fn new<U: Into<GMVec2D>, V: Into<GMVec2D>>(start: U, end: V, speed: f32, func: fn(value: GMVec2D,
            context: &mut GMContext, object_manager: &GMObjectManager, state: &mut GMState)) -> Self {
        let start = start.into();
        let end = end.into();

        debug!("GMValueInterpolateVec2D::new(), start: {}, end: {}, speed: {}", start, end, speed);

        let interpolation = GMInterpolateVec2D::new(start, end, speed, 0.0);

        Self {
            interpolation,
            auto_update: true,
            state: GMState::new(),
            func,
        }
    }
}

impl fmt::Debug for GMValueInterpolateVec2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMValueInterpolateVec2D, start: {}, end: {}, speed: {}", self.interpolation.start, self.interpolation.end, self.interpolation.speed)
    }
}

impl GMObjectT for GMValueInterpolateVec2D {
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
                (self.func)(value, context, object_manager, &mut self.state);
            }
            GMMessage::Custom0(name) if name == "calculate_diff" => {
                self.interpolation.calculate_diff();
            }
            GMMessage::Custom1(name, GMValue::Vec2D(start)) if name == "set_start" => {
                self.interpolation.start = start;
            }
            GMMessage::Custom1(name, GMValue::Vec2D(end)) if name == "set_end" => {
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
                let func = *value.downcast::<fn(value: GMVec2D, context: &mut GMContext,
                    object_manager: &GMObjectManager, state: &mut GMState)>().unwrap();
                self.func = func;
            }
            GMMessage::Multiple(messages) => {
                self.send_multi_message(messages, context, object_manager);
            }
            _ => {
                error_panic(&format!("Wrong message for GMValueInterpolateVec2D::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager) {
        if self.auto_update {
            self.interpolation.update();
            let value = self.interpolation.get_current_value();
            (self.func)(value, context, object_manager, &mut self.state);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct GMMapMessage {
    pub target: GMTarget,
    pub state: GMState,
    pub func: fn(message: GMMessage, state: &mut GMState) -> GMMessage,
}

impl GMMapMessage {
    pub fn new<T: Into<GMTarget>>(target: T, func: fn(message: GMMessage, state: &mut GMState) -> GMMessage) -> Self {
        let target = target.into();
        debug!("GMMapMessage::new(), target: {:?}", target);

        Self {
            target,
            state: GMState::new(),
            func,
        }
    }
}

impl fmt::Debug for GMMapMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMMapMessage, target: {:?}", self.target)
    }
}

impl GMObjectT for GMMapMessage {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Keep(keep_message) => {
                match *keep_message {
                    GMMessage::GetTarget => {
                        return self.target.clone().into();
                    }
                    GMMessage::SetTarget(target) => {
                        self.target = target;
                    }
                    GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_func" => {
                        let func = *value.downcast::<fn(message: GMMessage, state: &mut GMState) -> GMMessage>().unwrap();
                        self.func = func;
                    }
                    GMMessage::Multiple(mut messages) => {
                        // Wrap all messages in "keep" messages and use recursive call:
                        let messages: Vec<GMMessage> = messages.drain(0..).map(|m| GMMessage::Keep(Box::new(m))).collect();
                        self.send_multi_message(messages, context, object_manager);
                    }
                    _ => {
                        error_panic(&format!("Wrong message for GMMapMessage::send_message: {:?}", keep_message))
                    }
                }
            }
            _ => {
                let new_message = (self.func)(message, &mut self.state);
                object_manager.send_message(&self.target, new_message, context);
            }
        }
        GMValue::None
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct GMCustomSend {
    pub state: GMState,
    pub func: fn(message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager, state: &mut GMState) -> GMValue,
}

impl GMCustomSend {
    pub fn new(func: fn(message: GMMessage, context: &mut GMContext,
            object_manager: &GMObjectManager, state: &mut GMState) -> GMValue) -> Self {
        Self {
            state: GMState::new(),
            func,
        }
    }
}

impl fmt::Debug for GMCustomSend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMCustomSend")
    }
}

impl GMObjectT for GMCustomSend {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        (self.func)(message, context, object_manager, &mut self.state)
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
