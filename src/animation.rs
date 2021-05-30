
use macroquad::math::Rect;

pub enum GMAnimationDirection {
    ForewardOnce,
    ForewardLoop,
    Backwardonce,
    BackwardLoop,
    PingPong,
}

pub struct GMAnimation {
    name: String,
    frames: Vec<(Rect, u32)>,
    direction: GMAnimationDirection,
}

impl GMAnimation {
    pub fn new(name: &str, frames: &[(Rect, u32)]) -> GMAnimation {
        GMAnimation {
            name: name.to_string(),
            frames: frames.to_vec(),
            direction: GMAnimationDirection::ForewardOnce,
        }
    }
}

