
use std::fmt;
use std::rc::Rc;
use std::ops::{Sub, Add, Mul};
use std::collections::VecDeque;

use log::debug;
use nanorand::{WyRand, Rng};

use crate::context::GMContext;
use crate::curve::GMCurveT;
use crate::interpolation::{GMInterpolate, GMInterpolateF32, GMInterpolateVec2D};
use crate::math::GMVec2D;
use crate::object_manager::GMObjectManager;
use crate::object::GMObjectT;
use crate::target::GMTarget;
use crate::timer::GMTimer;
use crate::util::{error_panic, GMRepetition, random_range_f32, send_message_bool, send_message_f32, send_message_usize};
use crate::value::GMValue;
use crate::message::{GMMessage, msgt0v, msgt1v};


#[derive(Clone, Debug)]
pub struct GMTimedBase {
    pub timer: GMTimer,
    pub repeat: bool,
}

impl GMTimedBase {
    pub fn send_message(&mut self, mut message: GMMessage) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value.clone();

        match tag.as_str() {
            "timeout" => {
                return self.timer.send_message(message);
            }
            "repeat" => {
                return send_message_bool(&mut self.repeat, method, value);
            }
            _ => {
                error_panic(&format!("GMTimedBase::send_message, unknown tag: '{}'", tag));
            }
        }
    }
}


#[derive(Clone, Debug)]
pub struct GMTimedMessage {
    pub message: GMMessage,
    pub target: GMTarget,
    pub timed_base: GMTimedBase,
}

impl GMTimedMessage {
    pub fn new<T: Into<GMTarget>>(target: T, timeout: f32, repeat: bool, message: GMMessage) -> Self {
        let target = target.into();
        debug!("GMTimedMessage::new(), target: '{:?}', timeout: '{}', repeat: '{}', message: '{:?}'", target, timeout, repeat, message);

        let timer = GMTimer::new(timeout);
        let timed_base = GMTimedBase { timer, repeat };

        Self {
            message,
            target,
            timed_base,
        }
    }
}

impl GMObjectT for GMTimedMessage {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "message" => {
                return self.message.send_message(message);
            }
            "target" => {
                return self.target.send_message(method, message.value);
            }
            "base" => {
                return self.timed_base.send_message(message);
            }
            _ => {
                error_panic(&format!("GMTimedMessage::send_message, unknown tag: '{}'", tag));
            }
        }
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        if self.timed_base.timer.finished() {
            if self.timed_base.repeat {
                self.timed_base.timer.start();
            }

            object_manager.send_message(&self.target, self.message.clone());
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
    pub fn new(items: Vec<(f32, bool, GMTarget, GMMessage)>) -> Self {
        debug!("GMTimedMultiMessage::new()");

        let mut result = Self {
            items: Vec::new(),
        };

        result.set_items(items);

        result
    }

    pub fn set_items(&mut self, mut items: Vec<(f32, bool, GMTarget, GMMessage)>) {
        self.items = items.drain(0..).map(|(duration, repeat, target,
            message)| (GMTimer::new(duration), repeat, target, message)).collect();
    }
}

impl GMObjectT for GMTimedMultiMessage {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag1 = message.next_tag();
        let tag2 = message.next_tag();
        let method = message.method.as_str();

        match tag1.as_str() {
            "" => {
                match method {
                    "reset_all_timers" => {
                        for item in self.items.iter_mut() {
                            item.0.start();
                        }
                    }
                    "get_items" => {
                        return GMValue::from_any(self.items.clone());
                    }
                    "set_items" => {
                        let items = message.value.into_generic::<Vec<(f32, bool, GMTarget, GMMessage)>>();
                        self.set_items(items);
                    }
                    "set_item" => {
                        // TODO: add code
                    }
                    "set_some_items" => {
                        // TODO: add code
                    }
                    _ => {
                        error_panic(&format!("GMTimedMultiMessage::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "timer" => {
                let (index, timer_value) = message.value.into_generic::<(usize, GMValue)>();
                return self.items[index].0.send_message(msgt1v(tag2, method, timer_value));
            }
            "repeat" => {
                let (index, repeat_value) = message.value.into_generic::<(usize, GMValue)>();
                return send_message_bool(&mut self.items[index].1, method, repeat_value);
            }
            "target" => {
                let (index, target_value) = message.value.into_generic::<(usize, GMValue)>();
                return self.items[index].2.send_message(method, target_value);
            }
            "message" => {
                let (index, msg_value) = message.value.into_generic::<(usize, GMValue)>();
                return self.items[index].3.send_message(msgt1v(tag2, method, msg_value));
            }
            _=> {
                error_panic(&format!("GMTimedMultiMessage::send_message, unknown tag: '{}'", tag1));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        for (timer, repeat, target, message) in self.items.iter_mut() {
            if timer.finished() {
                if *repeat {
                    timer.start();
                }

                object_manager.send_message(&target, message.clone());
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMTimedSeqMessage {
    pub items: Vec<(GMTimer, GMTarget, GMMessage)>,
    pub index: usize,
    pub repeat: bool,
}

impl GMTimedSeqMessage {
    pub fn new(mut items: Vec<(f32, GMTarget, GMMessage)>, repeat: bool) -> Self {
        debug!("GMTimedSeqMessage::new()");

        Self {
            items: items.drain(0..).map(|(duration, target, message)|
                (GMTimer::new(duration), target, message)).collect(),
            index: 0,
            repeat,
        }
    }

    pub fn set_items(&mut self, mut items: Vec<(f32, GMTarget, GMMessage)>) {
        self.items = items.drain(0..).map(|(duration, target, message)| (GMTimer::new(duration), target, message)).collect();
    }
}

impl GMObjectT for GMTimedSeqMessage {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag1 = message.next_tag();
        let tag2 = message.next_tag();
        let method = message.method.as_str();

        match tag1.as_str() {
            "" => {
                match method {
                    "reset_timer" => {
                        // Resets the current timer
                        self.items[self.index].0.start();
                    }
                    "get_items" => {
                        return GMValue::from_any(self.items.clone());
                    }
                    "set_items" => {
                        let items = message.value.into_generic::<Vec<(f32, GMTarget, GMMessage)>>();
                        self.set_items(items);
                    }
                    "set_item" => {
                        // TODO: add code
                    }
                    "set_some_items" => {
                        // TODO: add code
                    }
                    _ => {
                        error_panic(&format!("GMTimedMultiMessage::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "index" => {
                return send_message_usize(&mut self.index, method, message.value);
            }
            "repeat" => {
                return send_message_bool(&mut self.repeat, method, message.value);
            }
            "timer" => {
                let (index, timer_value) = message.value.into_generic::<(usize, GMValue)>();
                return self.items[index].0.send_message(msgt1v(tag2, method, timer_value));
            }
            "target" => {
                let (index, target_value) = message.value.into_generic::<(usize, GMValue)>();
                return self.items[index].1.send_message(method, target_value);
            }
            "message" => {
                let (index, msg_value) = message.value.into_generic::<(usize, GMValue)>();
                return self.items[index].2.send_message(msgt1v(tag2, method, msg_value));
            }
            _=> {
                error_panic(&format!("GMTimedMultiMessage::send_message, unknown tag: '{}'", tag1));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        if self.index < self.items.len() {
            let (timer, target, message) = &mut self.items[self.index];

            if timer.finished() {
                object_manager.send_message(target, message.clone());
                self.index += 1;

                if self.index < self.items.len() {
                    self.items[self.index].0.start();
                } else if self.repeat {
                    self.index = 0
                }
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct GMTimedFunc {
    pub func: fn(object_manager: &GMObjectManager),
    pub timed_base: GMTimedBase,
}

impl GMTimedFunc {
    pub fn new(timeout: f32, repeat: bool, func: fn(object_manager: &GMObjectManager)) -> Self {
        debug!("GMTimedFunc::new(), timeout: '{}', repeat: '{}'", timeout, repeat);

        let timer = GMTimer::new(timeout);
        let timed_base = GMTimedBase { timer, repeat };

        Self {
            func,
            timed_base,
        }
    }
}

impl fmt::Debug for GMTimedFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMTimedFunc, timeout: {}, repeat: {}", self.timed_base.timer.duration, self.timed_base.repeat)
    }
}

impl GMObjectT for GMTimedFunc {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "" => {
                match method {
                    "set_func" => {
                        let func = message.value.into_generic::<fn(object_manager: &GMObjectManager)>();
                        self.func = func;
                    }
                    _ => {
                        error_panic(&format!("GMTimedFunc::send_message, method: '{}', no tag", method));
                    }
                }
            }
            "base" => {
                return self.timed_base.send_message(message);
            }
            _ => {
                error_panic(&format!("GMTimedFunc::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        if self.timed_base.timer.finished() {
            if self.timed_base.repeat {
                self.timed_base.timer.start();
            }

            (self.func)(object_manager);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct GMTrigger {
    pub func: fn(bject_manager: &GMObjectManager),
}

impl GMTrigger {
    pub fn new(func: fn(object_manager: &GMObjectManager)) -> Self {
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
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "trigger" => {
                        (self.func)(object_manager)
                    }
                    "set_func" => {
                        let func = value.into_generic::<fn(object_manager: &GMObjectManager)>();
                        self.func = func;
                    }
                    _ => {
                        error_panic(&format!("GMTimedFunc::send_message, unknown method: '{}', no tag", method));
                    }
                }

            }
            _ => {
                error_panic(&format!("GMTimedFunc::send_message, unknown tag: '{}'", tag));
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
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "trigger" => {
                        let result: VecDeque<GMValue> = VecDeque::new();

                        for (target, message) in self.pairs.iter() {
                            object_manager.send_message(&target, message.clone());
                        }

                        return result.into();
                    }
                    "set_pairs" => {
                        let pairs = value.into_generic::<Vec<(GMTarget, GMMessage)>>();
                        self.pairs = pairs;
                    }
                    "set_single_pair" => {
                        let (index, target, message) = value.into_generic::<(usize, GMTarget, GMMessage)>();
                        self.pairs[index] = (target, message);
                    }
                    "set_some_pairs" => {
                        let mut pairs = value.into_generic::<Vec<(usize, GMTarget, GMMessage)>>();

                        for (index, target, message) in pairs.drain(0..) {
                            self.pairs[index] = (target, message);
                        }
                    }
                    "get_pairs" => {
                        return GMValue::from_any(self.pairs.clone());
                    }
                    _ => {
                        error_panic(&format!("GMTriggerPair::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            _ =>{
                error_panic(&format!("GMTriggerPair::send_message, unknown tag: '{}'", tag));
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
    pub func: fn(value: f32, object_manager: &GMObjectManager),
    pub auto_update: bool,
    pub interpolation: GMInterpolateF32,
}

impl GMValueInterpolateF32 {
    pub fn new(start: f32, end: f32, speed: f32, func: fn(value: f32, object_manager: &GMObjectManager)) -> Self {
        debug!("GMValueInterpolateF32::new(), start: '{}', end: '{}', speed: '{}'", start, end, speed);

        let interpolation = GMInterpolateF32::new(start, end, speed, 0.0);

        Self {
            func,
            auto_update: true,
            interpolation,
        }
    }
}

impl fmt::Debug for GMValueInterpolateF32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMValueInterpolateF32, start: '{}', end: '{}', speed: '{}'",
            self.interpolation.start,
            self.interpolation.end,
            self.interpolation.speed)
    }
}

impl GMObjectT for GMValueInterpolateF32 {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "" => {
                match method {
                    "update" => {
                        self.interpolation.update();
                        let value = self.interpolation.get_current_value();
                        (self.func)(value, object_manager);
                    }
                    "set_func" => {
                        let func = message.value.into_generic::<fn(value: f32, object_manager: &GMObjectManager)>();
                        self.func = func;
                    }
                    _ => {
                        error_panic(&format!("GMValueInterpolateF32::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "auto_update" => {
                return send_message_bool(&mut self.auto_update, method, message.value);
            }
            "interpolation" => {
                return self.interpolation.send_message(message);
            }
            _ => {
                error_panic(&format!("GMValueInterpolateF32::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        if self.auto_update {
            self.interpolation.update();
            let value = self.interpolation.get_current_value();
            (self.func)(value, object_manager);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct GMValueInterpolateVec2D {
    pub func: fn(value: GMVec2D, object_manager: &GMObjectManager),
    pub auto_update: bool,
    pub interpolation: GMInterpolateVec2D,
}

impl GMValueInterpolateVec2D {
    pub fn new(start: GMVec2D, end: GMVec2D, speed: f32, func: fn(value: GMVec2D, object_manager: &GMObjectManager)) -> Self {
        debug!("GMValueInterpolateVec2D::new(), start: '{}', end: '{}', speed: '{}'", start, end, speed);

        let interpolation = GMInterpolateVec2D::new(start, end, speed, 0.0);

        Self {
            func,
            auto_update: true,
            interpolation,
        }
    }
}

impl fmt::Debug for GMValueInterpolateVec2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMValueInterpolateVec2D, start: '{:?}', end: '{:?}', speed: '{}'",
            self.interpolation.start,
            self.interpolation.end,
            self.interpolation.speed)
    }
}

impl GMObjectT for GMValueInterpolateVec2D {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "" => {
                match method {
                    "update" => {
                        self.interpolation.update();
                        let value = self.interpolation.get_current_value();
                        (self.func)(value, object_manager);
                    }
                    "set_func" => {
                        let func = message.value.into_generic::<fn(value: GMVec2D, object_manager: &GMObjectManager)>();
                        self.func = func;
                    }
                    _ => {
                        error_panic(&format!("GMValueInterpolateVec2D::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "auto_update" => {
                return send_message_bool(&mut self.auto_update, method, message.value);
            }
            "interpolation" => {
                return self.interpolation.send_message(message);
            }
            _ => {
                error_panic(&format!("GMValueInterpolateVec2D::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager) {
        if self.auto_update {
            self.interpolation.update();
            let value = self.interpolation.get_current_value();
            (self.func)(value, object_manager);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}




#[derive(Clone)]
pub struct GMMapMessage {
    pub target: GMTarget,
    pub func: fn(tag: &str, message: &str, value: GMValue) -> (String, String, GMValue),
}

impl GMMapMessage {
    pub fn new<T: Into<GMTarget>>(target: T,
            func: fn(tag: &str, message: &str, value: GMValue) -> (String, String, GMValue)) -> Self {
        let target = target.into();
        debug!("GMMapMessage::new(), target: '{:?}'", target);

        Self {
            target,
            func,
        }
    }
}

impl fmt::Debug for GMMapMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMMapMessage, target: '{:?}'", self.target)
    }
}

impl GMObjectT for GMMapMessage {
    /*
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
                        let func = *value.downcast::<fn(message: GMMessage) -> GMMessage>().unwrap();
                        self.func = func;
                    }
                    GMMessage::Multiple(mut messages) => {
                        // Wrap all messages in "keep" messages and use recursive call:
                        let messages: Vec<GMMessage> = messages.drain(0..).map(|m| GMMessage::Keep(Box::new(m))).collect();
                        self.send_message_multiple(messages, context, object_manager);
                    }
                    _ => {
                        error_panic(&format!("Wrong message for GMMapMessage::send_message: '{:?}'", keep_message))
                    }
                }
            }
            _ => {
                let new_message = (self.func)(message);
                object_manager.send_message(&self.target, new_message, context);
            }
        }
        GMValue::None
    }
*/

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct GMCustomSend {
    pub func: fn(message: GMMessage, object_manager: &GMObjectManager) -> GMValue,
}

impl GMCustomSend {
    pub fn new(func: fn(message: GMMessage, object_manager: &GMObjectManager) -> GMValue) -> Self {
        debug!("GMCustomSend::new()");

        Self {
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
    fn send_message(&mut self, message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        (self.func)(message, object_manager)
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct GMCustomUpdate {
    pub func: fn(object_manager: &GMObjectManager),
}

impl GMCustomUpdate {
    pub fn new(func: fn(object_manager: &GMObjectManager)) -> Self {
        debug!("GMCustomUpdate::new()");

        Self {
            func,
        }
    }
}

impl fmt::Debug for GMCustomUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GMCustomSend")
    }
}

impl GMObjectT for GMCustomUpdate {
    /*
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_func" => {
                let func = *value.downcast::<fn(context: &mut GMContext, object_manager: &GMObjectManager)>().unwrap();
                self.func = func;
            }
            GMMessage::Multiple(messages) => {
                self.send_message_multiple(messages, context, object_manager);
            }
            _ => {
                error_panic(&format!("Wrong message for GMCustomUpdate::send_message: '{:?}'", message))
            }
        }

        GMValue::None
    }
*/

    fn update(&mut self, object_manager: &GMObjectManager) {
        (self.func)(object_manager);
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMMultiPositionTarget {
    target: GMTarget,
}

impl GMMultiPositionTarget {
    pub fn new(target: GMTarget) -> Self {
        debug!("GMMultiPositionTarget::new()");

        Self {
            target,
        }
    }
}

impl GMObjectT for GMMultiPositionTarget {
    /*
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::SetMultiPosition(mut positions) => {
                let messages: Vec<GMMessage> = positions.drain(0..).map(|p| GMMessage::SetPosition(p)).collect();
                object_manager.send_message_zip(&self.target, messages, context);
            }
            GMMessage::GetTarget => {
                return self.target.clone().into();
            }
            GMMessage::SetTarget(target) => {
                self.target = target
            }
            GMMessage::Multiple(messages) => {
                self.send_message_multiple(messages, context, object_manager);
            }
            _ => {
                error_panic(&format!("Wrong message for GMMultiPositionTarget::send_message: '{:?}'", message))
            }
        }

        GMValue::None
    }
    */

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMCenterPosition {
    pub target: GMTarget,
    pub source: GMTarget,
    pub auto_update: bool,
}

impl GMCenterPosition {
    pub fn new<E: Into<GMTarget>, F: Into<GMTarget>>(target: E, source: F) -> Self {
        let target = target.into();
        let source = source.into();
        debug!("GMCenterPosition::new(), target: '{:?}', source: '{:?}'", target, source);

        Self {
            target,
            source,
            auto_update: true,
        }
    }

    pub fn calculate_center(&self, object_manager: &GMObjectManager) -> GMVec2D {
        let values = object_manager.send_message(&self.source,
            msgt0v("position", "get"));

        let mut positions = GMVec2D::new(0.0, 0.0);

        match values {
            GMValue::Vec2D(position) => {
                position.into()
            }
            GMValue::Multiple(values) => {
                let mut count = 0;

                for value in values {
                    if let GMValue::Vec2D(position) = value {
                        positions += position;
                        count += 1;
                    }
                }

                let factor = 1.0 / (count as f32);
                positions * factor
            }
            _ => {
                positions
            }
        }
    }
}

impl GMObjectT for GMCenterPosition {
    /*
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::GetTarget => {
                return self.target.clone().into();
            }
            GMMessage::SetTarget(target) => {
                self.target = target;
            }
            GMMessage::Custom0(name) if name == "get_source" => {
                return self.source.clone().into();
            }
            GMMessage::Custom1(name, GMValue::Target(source)) if name == "set_source" => {
                self.source = source;
            }
            GMMessage::Custom0(name) if name == "get_auto_update" => {
                return self.auto_update.into()
            }
            GMMessage::Custom1(name, GMValue::Bool(auto_update)) if name == "set_auto_update" => {
                self.auto_update = auto_update;
            }
            GMMessage::Update => {
                let position = self.calculate_center(context, object_manager);
                object_manager.send_message(&self.target, GMMessage::SetPosition(position), context);
            }
            GMMessage::Multiple(messages) => {
                self.send_message_multiple(messages, context, object_manager);
            }
            _ => {
                error_panic(&format!("Wrong message for GMCenterPosition::send_message: '{:?}'", message))
            }
        }

        GMValue::None
    }
    */

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMRandomPosition {
    pub target: GMTarget,
    pub minx: f32,
    pub miny: f32,
    pub maxx: f32,
    pub maxy: f32,
}

impl GMRandomPosition {
    pub fn new<T: Into<GMTarget>>(target: T, minx: f32, miny: f32, maxx: f32, maxy: f32) -> Self {
        let target = target.into();
        debug!("GMRandomPosition::new(), target: '{:?}', minx: '{}', miny: '{}', maxx: '{}', maxy: '{}'",
            target, minx, miny, maxx, maxy);

        Self {
            target,
            minx,
            miny,
            maxx,
            maxy,
        }
    }
}

impl GMObjectT for GMRandomPosition {
    /*
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::GetTarget => {
                return self.target.clone().into();
            }
            GMMessage::SetTarget(target) => {
                self.target = target;
            }
            GMMessage::Custom0(name) if name == "get_minx" => {
                return self.minx.into()
            }
            GMMessage::Custom0(name) if name == "get_miny" => {
                return self.miny.into()
            }
            GMMessage::Custom0(name) if name == "get_maxx" => {
                return self.maxx.into()
            }
            GMMessage::Custom0(name) if name == "get_maxy" => {
                return self.maxy.into()
            }
            GMMessage::Custom0(name) if name == "get_bounds" => {
                let bounds = Rc::new((self.minx, self.miny, self.maxx, self.maxy));
                return GMValue::Any(bounds);
            }
            GMMessage::Custom1(name, GMValue::F32(minx)) if name == "set_minx" => {
                self.minx = minx;
            }
            GMMessage::Custom1(name, GMValue::F32(miny)) if name == "set_miny" => {
                self.miny = miny;
            }
            GMMessage::Custom1(name, GMValue::F32(maxx)) if name == "set_maxx" => {
                self.maxx = maxx;
            }
            GMMessage::Custom1(name, GMValue::F32(maxy)) if name == "set_maxy" => {
                self.maxy = maxy;
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_bounds" => {
                let (minx, miny, maxx, maxy) = (*value.downcast::<(f32, f32, f32, f32)>().unwrap()).clone();
                self.minx = minx;
                self.miny = miny;
                self.maxx = maxx;
                self.maxy = maxy;
            }
            GMMessage::Update => {
                let x = random_range_f32(self.minx, self.maxx);
                let y = random_range_f32(self.miny, self.maxy);
                object_manager.send_message(&self.target, GMMessage::SetPosition(GMVec2D::new(x, y)), context);
            }
            GMMessage::Multiple(messages) => {
                self.send_message_multiple(messages, context, object_manager);
            }
            _ => {
                error_panic(&format!("Wrong message for GMRandomPosition::send_message: '{:?}'", message))
            }
        }

        GMValue::None
    }
    */

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GMRandomPositionOf {
    pub target: GMTarget,
    pub source: GMTarget,
}

impl GMRandomPositionOf {
    pub fn new<E: Into<GMTarget>, F: Into<GMTarget>>(target: E, source: F) -> Self {
        let target = target.into();
        let source = source.into();
        debug!("GMRandomPositionOf::new(), target: '{:?}', source: '{:?}'", target, source);

        Self {
            target: target.into(),
            source: source.into(),
        }
    }
}

impl GMObjectT for GMRandomPositionOf {
    /*
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::GetTarget => {
                return self.target.clone().into();
            }
            GMMessage::SetTarget(target) => {
                self.target = target;
            }
            GMMessage::Custom0(name) if name == "get_source" => {
                return self.source.clone().into();
            }
            GMMessage::Custom1(name, GMValue::Target(source)) if name == "set_source" => {
                self.source = source;
            }
            GMMessage::Update => {
                let value = object_manager.send_message(&self.source, GMMessage::GetPosition, context);
                let mut positions: Vec<GMVec2D> = Vec::new();

                match value {
                    GMValue::Position(position) => {
                        positions.push(position)
                    }
                    GMValue::Multiple(values) => {
                        for value in values {
                            if let GMValue::Position(position) = value {
                                positions.push(position);
                            }
                        }
                    }
                    _ => {
                        // No positions given
                    }
                }

                let num_elements = positions.len();

                if num_elements > 0 {
                    let mut rng = WyRand::new();
                    let index = rng.generate_range(0..num_elements);
                    return positions[index].into();
                }
            }
            GMMessage::Multiple(messages) => {
                self.send_message_multiple(messages, context, object_manager);
            }
            _ => {
                error_panic(&format!("Wrong message for GMRandomPosition::send_message: '{:?}'", message))
            }
        }

        GMValue::None
    }
    */

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
