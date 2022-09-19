
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

pub fn parse_string(message: &str) -> (&str, Vec<&str>) {
    let mut fields = message.split_whitespace();

    let name = fields.next().unwrap();

    let values: Vec<&str> = fields.collect();

    (name, values)
}

pub fn parse_f32(message: &str) -> (&str, Vec<f32>) {
    let (name, fields) = parse_string(message);
    let values = fields.iter().map(|s| s.parse::<f32>().unwrap()).collect();

    (name, values)
}

pub fn parse_u32(message: &str) -> (&str, Vec<u32>) {
    let (name, fields) = parse_string(message);
    let values = fields.iter().map(|s| s.parse::<u32>().unwrap()).collect();

    (name, values)
}
