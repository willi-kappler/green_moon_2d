
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
    debug!("error_panic() called from: '{}'", std::panic::Location::caller());
    error!("{}", message);
    panic!("{}", message);
}

pub fn parse_f32(message: &str) -> (&str, Vec<f32>) {
    let mut fields = message.split_whitespace();

    let name = fields.next().unwrap();

    let values = fields.map(|s| s.parse::<f32>().unwrap()).collect();

    (name, values)
}

pub fn parse_u32(message: &str) -> (&str, Vec<u32>) {
    let mut fields = message.split_whitespace();

    let name = fields.next().unwrap();

    let values = fields.map(|s| s.parse::<u32>().unwrap()).collect();

    (name, values)
}
