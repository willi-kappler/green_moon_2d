
use std::fmt;
use std::collections::VecDeque;

use log::debug;
use nanorand::{WyRand, Rng};

use crate::interpolation::{GMInterpolateF32, GMInterpolateVec2D};
use crate::math::GMVec2D;
use crate::object_manager::GMObjectManager;
use crate::object::GMObjectT;
use crate::target::GMTarget;
use crate::timer::GMTimer;
use crate::util::{error_panic, random_range_f32, send_message_bool, send_message_f32, send_message_usize};
use crate::value::GMValue;
use crate::message::{GMMessage, msgt0v, msgt1v};
use crate::context::GMContext;

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

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
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

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
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

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
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

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
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

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
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

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
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
    pub func: fn(message: GMMessage) -> GMMessage,
}

impl GMMapMessage {
    pub fn new<T: Into<GMTarget>>(target: T, func: fn(message: GMMessage) -> GMMessage) -> Self {
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
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            // Message for GMMapMessage
            "self" => {
                match method {
                    "set_func" => {
                        let func = message.value.into_generic::<fn(message: GMMessage) -> GMMessage>();
                        self.func = func;
                    }
                    _ => {
                        error_panic(&format!("GMMapMessage::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            // Message for GMMapMessage
            "self_target" => {
                return self.target.send_message(method, message.value);
            }
            // Message for target object
            _ => {
                // Put back the tag...
                message.tags.push_front(tag);

                let new_message = (self.func)(message);
                return object_manager.send_message(&self.target, new_message);
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
        // Maybe allow to change func ?
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
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "set_func" => {
                        let func = value.into_generic::<fn(object_manager: &GMObjectManager)>();
                        self.func = func;
                    }
                    _ => {
                        error_panic(&format!("GMCustomUpdate::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            _ => {
                error_panic(&format!("GMCustomUpdate::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }


    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        (self.func)(object_manager);
    }

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
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "update" => {
                        let position = self.calculate_center(object_manager);
                        object_manager.send_message(&self.target, msgt1v("position", "set", position));
                    }
                    _ => {
                        error_panic(&format!("GMCenterPosition::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, value);
            }
            "source" => {
                return self.source.send_message(method, value);
            }
            "auto_update" => {
                return send_message_bool(&mut self.auto_update, method, value);
            }
            _ => {
                error_panic(&format!("GMCenterPosition::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager, _context: &mut GMContext) {
        if self.auto_update {
            let position = self.calculate_center(object_manager);
            object_manager.send_message(&self.target, msgt1v("position", "set", position));
        }
    }

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
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "get_bounds" => {
                        return GMValue::from_any((self.minx, self.miny, self.maxx, self.maxy));
                    }
                    "set_bounds" => {
                        let (minx, miny, maxx, maxy) = value.into_generic::<(f32, f32, f32, f32)>();
                        self.minx = minx;
                        self.miny = miny;
                        self.maxx = maxx;
                        self.maxy = maxy;
                    }
                    "update" => {
                        let x = random_range_f32(self.minx, self.maxx);
                        let y = random_range_f32(self.miny, self.maxy);
                        object_manager.send_message(&self.target, msgt1v("position", "set", GMVec2D::new(x, y)));
                    }
                    _ => {
                        error_panic(&format!("GMRandomPosition::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, value);
            }
            "minx" => {
                return send_message_f32(&mut self.minx, method, value);
            }
            "miny" => {
                return send_message_f32(&mut self.miny, method, value);
            }
            "maxx" => {
                return send_message_f32(&mut self.maxx, method, value);
            }
            "maxy" => {
                return send_message_f32(&mut self.maxy, method, value);
            }
            _ => {
                error_panic(&format!("GMRandomPosition::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

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
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "update" => {
                        let value = object_manager.send_message(&self.source, msgt0v("position", "get"));
                        let mut positions: Vec<GMVec2D> = Vec::new();

                        match value {
                            GMValue::Vec2D(position) => {
                                positions.push(position)
                            }
                            GMValue::Multiple(values) => {
                                for value in values {
                                    if let GMValue::Vec2D(position) = value {
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
                    _ => {
                        error_panic(&format!("GMRandomPositionOf::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "target" => {
                return self.target.send_message(method, value);
            }
            "source" => {
                return self.source.send_message(method, value);
            }
            _ => {
                error_panic(&format!("GMRandomPositionOf::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
