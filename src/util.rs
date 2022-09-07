
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
