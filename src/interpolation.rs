
use std::ops::{Add, Sub, Mul};
use std::fmt::Debug;

use crate::util::GMRepetition;
use crate::math::GMVec2D;
use crate::curve::{GMCurveT, GMCuLinear};


#[derive(Debug, Clone)]
pub struct GMInterpolate<T> {
    pub start: T,
    pub end: T,
    diff: T,
    pub speed: f32,
    pub current_step: f32,
    current_value: T,
    pub repetition: GMRepetition,
    pub curve: Box<dyn GMCurveT>,
}

impl<T: Sub<T, Output = T> + Add<T, Output = T> + Mul<f32, Output = T> + Copy> GMInterpolate<T> {
    pub fn new(start: T, end: T, speed: f32, current_step: f32) -> Self {
        let diff = end - start;
        let current_value = start + (diff * current_step);

        Self {
            start,
            end,
            diff,
            speed,
            current_step,
            current_value,
            repetition: GMRepetition::OnceForward,
            curve: Box::new(GMCuLinear{}),
        }
    }

    pub fn calculate_diff(&mut self) {
        self.diff = self.end - self.start;
    }

    pub fn get_current_value(&self) -> T {
        self.current_value
    }

    pub fn calculate_value(&mut self) {
        let curve_value = self.curve.evaluate(self.current_step).clamp(0.0, 1.0);
        self.current_value = self.start + (self.diff * curve_value);
    }

    pub fn reset(&mut self) {
        self.current_step = 0.0;
        self.current_value = self.start;
    }

    pub fn update(&mut self) {
        match self.repetition {
            GMRepetition::OnceForward => {
                if self.current_step < 1.0 {
                    self.current_step += self.speed;
                    if self.current_step > 1.0 {
                        self.current_step = 1.0;
                    }
                }
            }
            GMRepetition::OnceBackward => {
                if self.current_step > 0.0 {
                    self.current_step -= self.speed;
                    if self.current_step < 0.0 {
                        self.current_step = 0.0;
                    }
                }
            }
            GMRepetition::LoopForward => {
                self.current_step += self.speed;

                if self.current_step > 1.0 {
                    self.current_step = 0.0;
                }
        }
            GMRepetition::LoopBackward => {
                self.current_step -= self.speed;

                if self.current_step < 0.0 {
                    self.current_step = 1.0;
                }
        }
            GMRepetition::PingPongForward => {
                if self.current_step < 1.0 {
                    self.current_step += self.speed;
                    if self.current_step > 1.0 {
                        self.current_step = 1.0;
                        self.repetition.reverse();
                    }
                }
            }
            GMRepetition::PingPongBackward => {
                if self.current_step > 0.0 {
                    self.current_step -= self.speed;
                    if self.current_step < 0.0 {
                        self.current_step = 0.0;
                        self.repetition.reverse();
                    }
                }
            }
        }

        self.calculate_value();
    }

    pub fn is_finished(&self) -> bool {
        match self.repetition {
            GMRepetition::OnceForward => {
                self.current_step >= 1.0
            }
            GMRepetition::OnceBackward => {
                self.current_step <= 0.0
            }
            _ => {
                false
            }
        }
    }
}

pub type GMInterpolateF32 = GMInterpolate<f32>;

pub type GMInterpolateVec2D = GMInterpolate<GMVec2D>;
