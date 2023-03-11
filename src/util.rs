

use nanorand::{WyRand, Rng};
use log::{error, debug};

use crate::context::{GMContext};

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

pub trait GMDrawT {
    fn draw(&self, _context: &mut GMContext) {
    }
}

pub trait GMUpdateT {
    fn update(&mut self, _context: &mut GMContext) {
    }
}

pub trait GMActiveT {
    fn set_active(&mut self, active: bool);
    fn get_active(&self) -> bool;
}

pub trait GMVisibleT {
    fn set_visible(&mut self, visible: bool);
    fn get_visible(&self) -> bool;
}
