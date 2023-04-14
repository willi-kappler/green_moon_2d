


use nanorand::{WyRand, Rng};
use log::{error, debug};

use crate::context::{GMContext};
use crate::math::{GMSize};

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
                error_panic(&format!("Unknown repetition: {}", value));
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
