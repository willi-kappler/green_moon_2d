
use std::ops::{Add, Sub, Mul};
use std::f32::consts::PI;
use std::fmt::Debug;

use crate::util::GMRepetition;
use crate::math::GMVec2D;

pub trait GMCurveT : Debug {
    fn evaluate(&mut self, x: f32) -> f32;
    fn clone_box(&self) -> Box<dyn GMCurveT>;
}

#[derive(Debug, Clone)]
pub struct GMCuLinear;

impl GMCurveT for GMCuLinear {
    fn evaluate(&mut self, x: f32) -> f32 {
        x
    }
    fn clone_box(&self) -> Box<dyn GMCurveT> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMCuX2Up;

impl GMCurveT for GMCuX2Up {
    fn evaluate(&mut self, x: f32) -> f32 {
        x*x
    }
    fn clone_box(&self) -> Box<dyn GMCurveT> {
        Box::new(self.clone())
    }
}


#[derive(Debug, Clone)]
pub struct GMCuX2Down;

impl GMCurveT for GMCuX2Down {
    fn evaluate(&mut self, x: f32) -> f32 {
        1.0 - (x*x)
    }
    fn clone_box(&self) -> Box<dyn GMCurveT> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMCuSlopeInOut;

impl GMCurveT for GMCuSlopeInOut {
    fn evaluate(&mut self, x: f32) -> f32 {
        if x < 0.5 {
            (x.powf(4.0)) * 8.0
        } else {
            ((-(x - 1.0).powf(4.0)) * 8.0) + 1.0
        }
    }
    fn clone_box(&self) -> Box<dyn GMCurveT> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMCuSinSlope;

impl GMCurveT for GMCuSinSlope {
    fn evaluate(&mut self, x: f32) -> f32 {
        (((x*PI) - PI/2.0).sin() + 1.0) / 2.0
    }
    fn clone_box(&self) -> Box<dyn GMCurveT> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn GMCurveT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct GMInterpolate<T> {
    start: T,
    end: T,
    diff: T,
    speed: f32,
    current_step: f32,
    current_value: T,
    repetition: GMRepetition,
    curve: Box<dyn GMCurveT>,
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

    pub fn set_start(&mut self, start: T) {
        self.start = start;
    }

    pub fn get_start(&self) -> T {
        self.start
    }

    pub fn set_end(&mut self, end: T) {
        self.end = end;
    }

    pub fn get_end(&self) -> T {
        self.end
    }

    pub fn calculate_diff(&mut self) {
        self.diff = self.end - self.start;
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn set_current_step(&mut self, current_step: f32) {
        self.current_step = current_step;
    }

    pub fn get_current_step(&self) -> f32 {
        self.current_step
    }

    pub fn get_current_value(&self) -> T {
        self.current_value
    }

    pub fn set_repetition(&mut self, repetition: GMRepetition) {
        self.repetition = repetition;
    }

    pub fn get_repetition(&self) -> GMRepetition {
        self.repetition
    }

    pub fn set_curve<U: GMCurveT + 'static>(&mut self, curve: U) {
        self.curve = Box::new(curve);
    }

    pub fn set_curve2(&mut self, curve: Box<dyn GMCurveT>) {
        self.curve = curve;
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
