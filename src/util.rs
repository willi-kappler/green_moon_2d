


use nanorand::{WyRand, Rng};
use log::{error, debug};


use crate::message::GMMessage;
use crate::object_base::GMValueBoolBase;
use crate::value::GMValue;

// const ALPHA_NUM: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
// Not possible yet...
// const ALPHA_NUM2: Vec<char> = ALPHA_NUM.chars().collect();


#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum GMAlign {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum GMRepetition {
    OnceForward,
    OnceBackward,
    LoopForward,
    LoopBackward,
    PingPongForward,
    PingPongBackward,
    // TODO: PinPongForwardCount, PinPongBackwardCount
}

impl GMRepetition {
    pub fn reverse(&mut self) {
        match self {
            GMRepetition::OnceForward => {
                *self = GMRepetition::OnceBackward;
            }
            GMRepetition::OnceBackward => {
                *self = GMRepetition::OnceForward;
            }
            GMRepetition::LoopForward => {
                *self = GMRepetition::LoopBackward;
            }
            GMRepetition::LoopBackward => {
                *self = GMRepetition::LoopForward;
            }
            GMRepetition::PingPongForward => {
                *self = GMRepetition::PingPongBackward;
            }
            GMRepetition::PingPongBackward => {
                *self = GMRepetition::PingPongForward;
            }
        }
    }

    pub fn send_message(&mut self, message: GMMessage) -> GMValue {
        match message {
            GMMessage::GetRepetition => {
                (*self).into()
            }
            GMMessage::SetRepetition(repetition) => {
                *self = repetition;
                GMValue::None
            }
            GMMessage::Custom0(name) if name == "reverse" => {
                self.reverse();
                GMValue::None
            }
            _ => {
                GMValue::unknown(message)
            }
        }
    }
}

impl From<&str> for GMRepetition {
    fn from(value: &str) -> Self {
        match value {
            "once_forward" => GMRepetition::OnceForward,
            "once_backward" => GMRepetition::OnceBackward,
            "loop_forward" => GMRepetition::LoopForward,
            "loop_backward" => GMRepetition::LoopBackward,
            "ping_pong_forward" => GMRepetition::PingPongForward,
            "ping_pong_backward" => GMRepetition::PingPongBackward,
            _ => {
                error_panic(&format!("Unknown repetition: '{}'", value));
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct GMFlipXY {
    pub x: GMValueBoolBase,
    pub y: GMValueBoolBase,
}

impl GMFlipXY {
    pub fn new() -> Self {
        Self {
            x: GMValueBoolBase::new(false, "flipx"),
            y: GMValueBoolBase::new(false, "flipy"),
        }
    }

    pub fn send_message(&mut self, message: GMMessage) -> GMValue {
        match message {
            GMMessage::Custom0(name) if name == "get_flipxy" => {
                (self.x.value, self.y.value).into()
            }
            GMMessage::Custom0(name) if name == "toggle_flipxy" => {
                self.x.value = !self.x.value;
                self.y.value = !self.y.value;
                GMValue::None
            }
            GMMessage::Custom2(name, GMValue::Bool(x), GMValue::Bool(y)) if name == "set_flipxy" => {
                self.x.value = x;
                self.y.value = y;
                GMValue::None
            }
            _ => {
                self.x.send_message(message)
                    .handle(|m| self.y.send_message(m))
            }
        }
    }
}

#[track_caller]
pub fn error_panic(message: &str) -> ! {
    debug!("error_panic() called from: '{}'", std::panic::Location::caller());
    error!("{}", message);
    panic!("{}", message);
}

pub fn random_range_f32(min: f32, max: f32) -> f32 {
    assert!(min <= max, "random_range_f32(), min must be smaller than max: '{}' >= '{}'", min, max);
    let length = max - min;
    let mut rng = WyRand::new();
    let result = min + (rng.generate::<f32>() * length);
    result
}

// There is a better way to do this...
/*
pub fn random_string(len: usize) -> String {
    let mut vec = Vec::with_capacity(len);
    let mut rng = WyRand::new();
    let chars = ALPHA_NUM.chars().collect::<Vec<char>>();

    for i in 0..len {
        let j = rng.generate_range(0..=ALPHA_NUM.len());
        vec[i] = chars[j];
    }

    String::from_iter(vec)
}
*/
