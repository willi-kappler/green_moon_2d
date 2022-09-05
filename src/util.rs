
use std::any::Any;

use log::{error, debug};

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

#[track_caller]
pub fn error_panic(message: &str) -> ! {
    debug!("error_panic() called from: {}", std::panic::Location::caller());
    error!("{}", message);
    panic!("{}", message);
}

pub fn extract_f32_value(message: &str, data: Box<dyn Any>) -> f32 {
    if let Ok(value) = data.downcast::<f32>() {
        return *value
    }

    error_panic(&format!("util::extract_f32_value(), expected f32, message: {}", message))
}

pub fn extract_usize_value(message: &str, data: Box<dyn Any>) -> usize {
    if let Ok(value) = data.downcast::<usize>() {
        return *value
    }

    error_panic(&format!("util::extract_usize_value(), expected usize, message: {}", message))
}
