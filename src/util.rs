


use nanorand::{WyRand, Rng};
use log::{error, debug};

use crate::context::{GMContext};
use crate::math::{GMSize, GMFlipXY};

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

#[derive(Debug, Clone)]
pub enum GMProperty {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
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
    fn update(&mut self) {
    }

    fn update2(&mut self, _context: &mut GMContext) {
        self.update()
    }
}

pub trait GMFlipXYT {
    fn set_flip_x(&mut self, flip_x: bool) {
        self.get_flip_xy_mut().flip_x = flip_x;
    }

    fn get_flip_x(&self) -> bool {
        self.get_flip_xy().flip_x
    }

    fn toggle_x(&mut self) {
        let flip_x = self.get_flip_x();
        self.get_flip_xy_mut().flip_x = !flip_x;
    }

    fn set_flip_y(&mut self, flip_y: bool) {
        self.get_flip_xy_mut().flip_y = flip_y;
    }

    fn get_flip_y(&self) -> bool {
        self.get_flip_xy().flip_y
    }

    fn set_flip_xy(&mut self, flip_x: bool, flip_y: bool) {
        let flip_xy = self.get_flip_xy_mut();
        flip_xy.flip_x = flip_x;
        flip_xy.flip_y = flip_y;
    }

    fn toggle_y(&mut self) {
        let flip_y = self.get_flip_y();
        self.get_flip_xy_mut().flip_y = !flip_y;
    }

    fn get_flip_xy(&self) -> &GMFlipXY;

    fn get_flip_xy_mut(&mut self) -> &mut GMFlipXY;
}

#[macro_export]
macro_rules! gen_impl_flipxy {
    ($type:ty) => {
        impl GMFlipXYT for $type {
            fn get_flip_xy(&self) -> &GMFlipXY {
                &self.flip_xy
            }

            fn get_flip_xy_mut(&mut self) -> &mut GMFlipXY {
                &mut self.flip_xy
            }
        }
    };
}

pub trait GMActiveT {
    fn set_active(&mut self, active: bool);

    fn get_active(&self) -> bool;

    fn toggle(&mut self) {
        let active = self.get_active();
        self.set_active(!active);
    }
}

#[macro_export]
macro_rules! gen_impl_active {
    ($type:ty) => {
        impl GMActiveT for $type {
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

    fn toggle(&mut self) {
        let visible = self.get_visible();
        self.set_visible(!visible);
    }
}

#[macro_export]
macro_rules! gen_impl_visible {
    ($type:ty) => {
        impl GMVisibleT for $type {
            fn set_visible(&mut self, visible: bool) {
                self.visible = visible;
            }

            fn get_visible(&self) -> bool {
                self.visible
            }
        }
    };
}

pub trait GMSizeT {
    fn set_width(&mut self, width: f32) {
        self.get_size_mut().width = width;
    }

    fn get_width(&self) -> f32 {
        self.get_size().width
    }

    fn set_height(&mut self, height: f32) {
        self.get_size_mut().height = height;
    }

    fn get_height(&self) -> f32 {
        self.get_size().height
    }

    fn set_size(&mut self, width: f32, height: f32) {
        let size = self.get_size_mut();
        size.width = width;
        size.height = height;
    }

    fn get_size(&self) -> &GMSize;

    fn get_size_mut(&mut self) -> &mut GMSize;
}

#[macro_export]
macro_rules! gen_impl_size {
    ($type:ty) => {
        impl GMSizeT for $type {
            fn get_size(&self) -> &GMSize {
                &self.size
            }

            fn get_size_mut(&mut self) -> &mut GMSize {
                &mut self.size
            }
        }
    };
}
