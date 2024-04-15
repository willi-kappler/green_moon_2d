
use std::f32::consts::PI;
use std::fmt::Debug;


pub trait GMCurveT: Debug {
    fn evaluate(&mut self, x: f32) -> f32;
    fn clone_box(&self) -> Box<dyn GMCurveT>;
}

impl Clone for Box<dyn GMCurveT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<U: GMCurveT + 'static> From<U> for Box<dyn GMCurveT> {
    fn from(object: U) -> Self {
        Box::new(object)
    }
}

impl From<&dyn GMCurveT> for Box<dyn GMCurveT> {
    fn from(object: &dyn GMCurveT) -> Self {
        object.clone_box()
    }
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
