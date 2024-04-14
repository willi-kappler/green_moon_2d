use std::f32::consts;
use std::any::Any;
pub struct GMKeyValue<'a> {
    pub key: &'a str,
    pub value: Box<dyn Any>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GMOffscreenMode {
    Keep,
    Destroy,
    WrapAround,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GMValue {
    None,
    Bool(bool),
    F32(f32),
    USize(usize),
    String(String),
}

pub enum GMDirection4 {
    Up,
    Right,
    Down,
    Left,
}

pub fn between(a: f32, b: f32, c: f32) -> bool {
    a <= b && b <= c
}

pub fn in_rect(x1: f32, x2: f32, y1: f32, y2: f32, xp: f32, yp: f32) -> bool {
    between(x1, xp, x2) && between(y1, yp, y2)
}

pub fn dist_point(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    dx.hypot(dy)
}

pub fn angle_point(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;

    let mut angle = (dy / dx).atan();

    if dx < 0.0 && dy >= 0.0 {
        angle += consts::PI;
    } else if dx < 0.0 && dy < 0.0 {
        angle += consts::PI;
    } else if dx >= 0.0 && dy < 0.0 {
        angle += consts::TAU;
    }

    angle
}
