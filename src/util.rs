
use log::{error, debug};

use crate::math::GMVec2D;

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

pub struct GMInterpolateF32 {
    start: f32,
    end: f32,
    speed: f32,
    value: f32,
    pub repetition: GMRepetition,
}

impl GMInterpolateF32 {
    pub fn new(start: f32, end: f32, speed: f32) -> Self {
        let result = Self {
            start,
            end,
            speed,
            value: start,
            repetition: GMRepetition::OnceForward,
        };

        result.check_bounds();
        result
    }

    pub fn update(&mut self) {
        match self.repetition {
            GMRepetition::OnceForward => {
                self.value += self.speed;
                if self.value > self.end {
                    self.value = self.end;
                }
            }
            GMRepetition::OnceBackward => {
                self.value -= self.speed;
                if self.value < self.start {
                    self.value = self.start;
                }
            }
            GMRepetition::LoopForward => {
                self.value += self.speed;
                if self.value > self.end {
                    self.value = self.start;
                }
            }
            GMRepetition::LoopBackward => {
                self.value -= self.speed;
                if self.value < self.start {
                    self.value = self.end;
                }
            }
            GMRepetition::PingPongForward => {
                self.value += self.speed;
                if self.value > self.end {
                    self.value = self.end;
                    self.repetition = GMRepetition::PingPongBackward;
                }

            }
            GMRepetition::PingPongBackward => {
                self.value -= self.speed;
                if self.value < self.start {
                    self.value = self.start;
                    self.repetition = GMRepetition::PingPongForward;
                }
            }
        }
    }

    fn check_bounds(&self) {
        assert!(self.end > self.start, "GMInterpolateF32::check_bounds(), end ('{}') must be greater than start ('{}')", self.end, self.start);
        self.check_speed();
    }

    fn check_speed(&self) {
        let difference = self.end - self.start;
        assert!(self.speed > 0.0 && self.speed < difference, "GMInterpolateF32::check_speed(), speed must be between 0.0 and {}", difference);
    }

    pub fn set_start(&mut self, start: f32) {
        self.start = start;
        self.check_bounds();
    }

    pub fn set_end(&mut self, end: f32) {
        self.end = end;
        self.check_bounds();
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
        self.check_speed();
    }

    pub fn set_value(&mut self, value: f32) {
        let difference = self.end - self.start;
        assert!(value >= 0.0 && value <= difference, "GMInterpolateF32::set_value(), value must be between 0.0 and {}", difference);

        self.value = value;
    }

    pub fn set_value_norm(&mut self, value: f32) {
        assert!(value >= 0.0 && value <= 1.0, "GMInterpolateF32::set_value_norm(), value must be between 0.0 and 1.0");

        let difference = self.end - self.start;
        self.value = difference * value;
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }
}

pub struct GMInterpolateVec2D {
    start: GMVec2D,
    end: GMVec2D,
    position: GMVec2D,
    speed: f32,
    direction: GMVec2D,
    length: f32,
    value: f32,
    pub repetition: GMRepetition,
}

impl GMInterpolateVec2D {
    pub fn new(start: GMVec2D, end: GMVec2D, speed: f32) -> Self {
        let direction = end - start;
        let length = direction.len();

        let result = Self {
            start,
            end,
            position: start.clone(),
            speed,
            direction: direction.norm2(),
            length,
            value: 0.0,
            repetition: GMRepetition::OnceForward,
        };

        result.check_length();
        result.check_speed();
        result
    }

    pub fn update(&mut self) {
        match self.repetition {
            GMRepetition::OnceForward => {
                self.value += self.speed;
                if self.value > self.length {
                    self.value = self.length;
                }

                self.position = self.start + (self.direction * self.value);
            },
            GMRepetition::OnceBackward => {
                self.value -= self.speed;
                if self.value < 0.0 {
                    self.value = 0.0;
                }

                self.position = self.start + (self.direction * self.value);
            },
            GMRepetition::LoopForward => {
                self.value += self.speed;
                if self.value > self.length {
                    self.value = 0.0;
                }

                self.position = self.start + (self.direction * self.value);
            },
            GMRepetition::LoopBackward => {
                self.value -= self.speed;
                if self.value < 0.0 {
                    self.value = self.length;
                }

                self.position = self.start + (self.direction * self.value);
            },
            GMRepetition::PingPongForward => {
                self.value += self.speed;
                if self.value > self.length {
                    self.value = self.length;
                    self.repetition = GMRepetition::PingPongBackward;
                }

                self.position = self.start + (self.direction * self.value);
            },
            GMRepetition::PingPongBackward => {
                self.value -= self.speed;
                if self.value < 0.0 {
                    self.value = 0.0;
                    self.repetition = GMRepetition::PingPongForward;
                }

                self.position = self.start + (self.direction * self.value);
            },
        }
    }

    fn check_length(&self) {
        assert!(self.length > 0.0, "GMInterpolateVec2D::check_length(), length must be greater than 0.0, start: '{:?}', end: '{:?}'", self.start, self.end);
    }

    fn check_speed(&self) {
        assert!(self.speed > 0.0 && self.speed < self.length, "GMInterpolateVec2D::check_speed(), speed must be between 0.0 and {}", self.length);
    }

    pub fn set_start(&mut self, start: GMVec2D) {
        self.start = start;

        let direction = self.end - self.start;
        self.length = direction.len();
        self.direction = direction.norm2();

        self.check_length();
        self.check_speed();
    }

    pub fn set_end(&mut self, end: GMVec2D) {
        self.end = end;

        let direction = self.end - self.start;
        self.length = direction.len();
        self.direction = direction.norm2();

        self.check_length();
        self.check_speed();
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;

        self.check_speed();
    }

    pub fn set_value(&mut self, value: f32) {
        assert!(value >= 0.0 && value <= self.length, "GMInterpolateVec2D::set_value(), value must be between 0.0 and {}", self.length);

        self.value = value;
    }

    pub fn set_value_norm(&mut self, value: f32) {
        assert!(value >= 0.0 && value <= 1.0, "GMInterpolateVec2D::set_value_norm(), value must be between 0.0 and 1.0");

        self.value = self.length * value;
    }

    pub fn get_position(&self) -> GMVec2D {
        self.position.clone()
    }
}
