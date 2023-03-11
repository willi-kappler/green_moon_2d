

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

pub trait GMFlipXYT {
    fn set_flip_x(&mut self, flip_x: bool);
    fn get_flip_x(&self) -> bool;
    fn set_flip_y(&mut self, flip_y: bool);
    fn get_flip_y(&self) -> bool;
}

#[macro_export]
macro_rules! gen_impl_flipxy {
    ($type:ty) => {
        impl GMFlipXYT for $type {
            fn set_flip_x(&mut self, flip_x: bool) {
                self.flip_x = flip_x;
            }

            fn get_flip_x(&self) -> bool {
                self.flip_x
            }

            fn set_flip_y(&mut self, flip_y: bool) {
                self.flip_y = flip_y;
            }

            fn get_flip_y(&self) -> bool {
                self.flip_y
            }
        }
    };
}

pub trait GMActiveT {
    fn set_active(&mut self, active: bool);
    fn get_active(&self) -> bool;
}

#[macro_export]
macro_rules! gen_impl_active {
    ($type:ty) => {
        impl GMActiveT  for $type {
            fn set_active(&mut self, active: bool) {
                self.active = active;
            }

            fn get_active(&self) -> bool {
                self.active
            }
        }
    };
}

pub trait GMVisibleT {
    fn set_visible(&mut self, visible: bool);
    fn get_visible(&self) -> bool;
}

#[macro_export]
macro_rules! gen_impl_visible {
    ($type:ty) => {
        impl GMVisibleT  for $type {
            fn set_visible(&mut self, visible: bool) {
                self.visible = visible;
            }

            fn get_visible(&self) -> bool {
                self.visible
            }
        }
    };
}
