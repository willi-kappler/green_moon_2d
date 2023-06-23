


use nanorand::{WyRand, Rng};
use log::{error, debug};

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
    Fixed,
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
            GMRepetition::Fixed => {
                *self = GMRepetition::Fixed;
            }
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

    pub fn send_message(&mut self, method: &str, value: GMValue) -> GMValue {
        match method {
            "get" => {
                return (*self).into();
            }
            "set" => {
                *self = value.into_repetition();
            }
            "reverse" => {
                self.reverse();
            }
            _ => {
                error_panic(&format!("GMRepetition::send_message, unknown method: '{}'", method));
            }
        }

        GMValue::None
    }
}

impl From<&str> for GMRepetition {
    fn from(value: &str) -> Self {
        match value {
            "fixed" => GMRepetition::Fixed,
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

pub fn send_message_f32(v: &mut f32, method: &str, value: GMValue) -> GMValue {
    match method {
        "get" => {
            return (*v).into();
        }
        "set" => {
            *v = value.into_f32();
        }
        "add" => {
            *v += value.into_f32();
        }
        "mul" => {
            *v *= value.into_f32();
        }
        _ => {
            error_panic(&format!("send_message_f32, unknown method: '{}'", method));
        }
    }

    GMValue::None
}

pub fn send_message_usize(v: &mut usize, method: &str, value: GMValue) -> GMValue {
    match method {
        "get" => {
            return (*v).into();
        }
        "set" => {
            *v = value.into_usize();
        }
        "add" => {
            *v += value.into_usize();
        }
        "mul" => {
            *v *= value.into_usize();
        }
        _ => {
            error_panic(&format!("send_message_usize, unknown method: '{}'", method));
        }
    }

    GMValue::None
}

pub fn send_message_bool(v: &mut bool, method: &str, value: GMValue) -> GMValue {
    match method {
        "get" => {
            return (*v).into();
        }
        "set" => {
            *v = value.into_bool();
        }
        "toggle" => {
            *v = !(*v);
        }
        _ => {
            error_panic(&format!("send_message_bool, unknown method: '{}'", method));
        }
    }

    GMValue::None
}

pub fn send_message_str(v: &mut String, method: &str, value: GMValue) -> GMValue {
    match method {
        "get" => {
            return (*v).clone().into();
        }
        "set" => {
            *v = value.into_string();
        }
        _ => {
            error_panic(&format!("send_message_str, unknown method: '{}'", method));
        }
    }

    GMValue::None
}
