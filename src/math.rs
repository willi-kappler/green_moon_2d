use std::ops::{Add, AddAssign, Sub, Mul};
use std::fmt::Display;

use std::f32::consts::TAU;

use crate::util::{error_panic, send_message_f32};
use crate::value::GMValue;
use crate::message::GMMessage;

#[derive(Copy, Clone, Debug)]
pub struct GMVec2D {
    pub x: f32,
    pub y: f32,
}

impl GMVec2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn len(&self) -> f32 {
        self.len2().sqrt()
    }

    pub fn len2(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn norm(&mut self) {
        let l = self.len();

        self.x  = self.x / l;
        self.y  = self.y / l;
    }

    pub fn norm2(&self) -> Self {
        let l = self.len();

        Self {
            x: self.x / l,
            y: self.y / l,
        }
    }

    pub fn set<T: Into<GMVec2D>>(&mut self, vec2d: T) {
        let other = vec2d.into();
        self.x = other.x;
        self.y = other.y;
    }

    pub fn add2<T: Into<GMVec2D>>(&mut self, vec2d: T) {
        let other = vec2d.into();
        self.x += other.x;
        self.y += other.y;
    }

    pub fn sub2<T: Into<GMVec2D>>(&mut self, vec2d: T) {
        let other = vec2d.into();
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn mul2(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }

    pub fn dot(&self, other: &GMVec2D) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn angle<T: Into<GMVec2D>>(&self, vec2d: T) -> f32 {
        let other = vec2d.into();
        let dot = self.dot(&other);
        let l1 = self.len();
        let l2 = other.len();

        (dot/(l1*l2)).acos()
    }

    pub fn dist_to<T: Into<GMVec2D>>(&self, vec2d: T) -> f32 {
        let other = vec2d.into();
        let dx = self.x - other.x;
        let dy= self.y - other.y;

        dx.hypot(dy)
    }

    pub fn send_message(&mut self, method: &str, value: GMValue) -> GMValue {
        match method {
            "get" => {
                return self.clone().into();
            }
            "set" => {
                *self = value.into_vec2d();
            }
            "add" => {
                *self += value.into_vec2d();
            }
            "mul" => {
                let factor = value.into_f32();
                self.x *= factor;
                self.y *= factor;
            }
            "get_x" => {
                return self.x.into();
            }
            "set_x" => {
                self.x = value.into_f32();
            }
            "add_x" =>{
                self.x += value.into_f32();
            }
            "mul_x" => {
                self.x *= value.into_f32();
            }
            "get_y" => {
                return self.x.into();
            }
            "set_y" => {
                self.x = value.into_f32();
            }
            "add_y" =>{
                self.x += value.into_f32();
            }
            "mul_y" => {
                self.x *= value.into_f32();
            }
            "get_xy" =>{
                let x: GMValue = self.x.into();
                let y: GMValue = self.y.into();
                return x.chain(y)
            }
            "set_xy" => {
                let mut values = value.to_vec_deque();
                self.x = values.pop_front().unwrap().into_f32();
                self.y = values.pop_front().unwrap().into_f32();
            }
            _ => {
                error_panic(&format!("GMVec2D::send_message, unknown method: '{}'", method));
            }
        }

        GMValue::None
    }
}

impl Default for GMVec2D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}

impl Add for GMVec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        GMVec2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for GMVec2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for GMVec2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        GMVec2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for GMVec2D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        GMVec2D::new(self.x * rhs, self.y * rhs)
    }
}

impl From<(f32, f32)> for GMVec2D {
    fn from((x, y): (f32, f32)) -> Self {
        GMVec2D { x, y }
    }
}

impl From<[f32; 2]> for GMVec2D {
    fn from([x, y]: [f32; 2]) -> Self {
        GMVec2D { x, y }
    }
}

impl From<&[f32]> for GMVec2D {
    fn from(array: &[f32]) -> Self {
        GMVec2D {
            x: array[0],
            y: array[1],
        }
    }
}

// TODO: impl from Vec<(f32, f32)> to Vec<GMVec2D>

impl Display for GMVec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GMSize {
    pub width: f32,
    pub height: f32,
}

impl GMSize {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn send_message(&mut self, mut message: GMMessage) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "get" => {
                        return self.clone().into();
                    }
                    "set" => {
                        *self = value.into_size();
                    }
                    "add" => {
                        let mut vec_deque = value.to_vec_deque();
                        let width = vec_deque.pop_front().unwrap().into_f32();
                        let height = vec_deque.pop_front().unwrap().into_f32();
                        self.width += width;
                        self.height += height;
                    }
                    "mul" => {
                        let factor = value.into_f32();
                        self.width *= factor;
                        self.height *= factor;
                    }
                    "set_wh" => {
                        let mut vec_deque = value.to_vec_deque();
                        let width = vec_deque.pop_front().unwrap().into_f32();
                        let height = vec_deque.pop_front().unwrap().into_f32();
                        self.width = width;
                        self.height = height;
                    }
                    "get_wh" => {
                        let width: GMValue = self.width.into();
                        let height: GMValue = self.height.into();
                        return width.chain(height)
                    }
                    _ => {
                        error_panic(&format!("GMSize::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "width" => {
                return send_message_f32(&mut self.width, method, value);
            }
            "height" => {
                return send_message_f32(&mut self.height, method, value);
            }
            _ => {
                error_panic(&format!("GMSize::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }
}

impl Default for GMSize {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }
}

impl From<(f32, f32)> for GMSize {
    fn from((width, height): (f32, f32)) -> Self {
        GMSize { width, height }
    }
}

impl Display for GMSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(w: {}, h: {})", self.width, self.height)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GMRectangle {
    // TODO: use Vec2D
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

impl GMRectangle {
    pub fn new(x1: f32, x2: f32, y1: f32, y2: f32) -> Self {
        Self {
            x1, x2, y1, y2,
        }
    }

    pub fn new2(point: &GMVec2D, size: &GMSize) -> Self {
        Self {
            x1: point.x,
            x2: point.x + size.width,
            y1: point.y,
            y2: point.y + size.width,
        }
    }

    // Center point
    pub fn new3(point: &GMVec2D, size: &GMSize) -> Self {
        let w2 = size.width / 2.0;
        let h2 = size.height / 2.0;

        Self {
            x1: point.x - w2,
            x2: point.x + w2,
            y1: point.y - h2,
            y2: point.y + h2,
        }
    }

    pub fn point_inside(&self, x: f32, y: f32) -> bool {
        (self.x1 <= x) && (x <= self.x2) && (self.y1 <= y) && (y <= self.y2)
    }

    pub fn point_inside2(&self, point: &GMVec2D) -> bool {
        self.point_inside(point.x, point.y)
    }

    pub fn rect_intersect(&self, other: &GMRectangle) -> bool {
        self.point_inside(other.x1, other.y1) ||
        self.point_inside(other.x1, other.y2) ||
        self.point_inside(other.x2, other.y1) ||
        self.point_inside(other.x2, other.y2)
    }

    // TODO: return intersect points

    pub fn send_message(&mut self, mut message: GMMessage) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "get" => {
                        return self.clone().into()
                    }
                    "set" => {
                        *self = value.into_rectangle();
                    }
                    _ => {
                        error_panic(&format!("GMRectangle::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "top_left" => {

            }
            "bottom_right" => {

            }
            _ => {
                error_panic(&format!("GMRectangle::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }

}

impl From<(f32, f32, f32, f32)> for GMRectangle {
    fn from((x1, x2, y1, y2): (f32, f32, f32, f32)) -> Self {
        GMRectangle { x1, x2, y1, y2 }
    }
}

impl Display for GMRectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x1: {}, y1: {}, x2: {}, y2: {})", self.x1, self.y1, self.x2, self.y2)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GMCircle {
    pub center: GMVec2D,
    pub radius: f32,
}

impl GMCircle {
    pub fn new<T: Into<GMVec2D>>(position: T, radius: f32) -> Self {
        Self {
            center: position.into(),
            radius,
        }
    }

    pub fn point_inside<T: Into<GMVec2D>>(&self, position: T) -> bool {
        let point = position.into();
        let dist = (self.center - point).len();

        dist <= self.radius
    }

    pub fn circ_intersect(&self, other: &GMCircle) -> bool {
        let dist = (self.center - other.center).len();

        dist <= (self.radius + other.radius)
    }

    // Point on circle border
    pub fn circ_point<T: Into<GMVec2D>>(&self, position: T) -> GMVec2D {
        let mut vec = position.into();
        vec.norm();
        vec.mul2(self.radius);
        vec.add2(self.center);
        vec
    }

    pub fn position_from_deg(&self, deg: f32) -> GMVec2D {
        self.position_from_rad(deg * TAU / 360.0)
    }

    pub fn position_from_rad(&self, rad: f32) -> GMVec2D {
        let x = self.center.x + (rad.cos() * self.radius);
        let y = self.center.y + (rad.sin() * self.radius);
        GMVec2D::new(x, y)
    }

    pub fn send_message(&mut self, mut message: GMMessage) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    // TODO: Add more methods
                    _ => {
                        error_panic(&format!("GMCircle::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "radius" => {
                return send_message_f32(&mut self.radius, method, value);
            }
            "position" => {
                return self.center.send_message(method, value);
            }
            _ => {
                error_panic(&format!("GMCircle::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }
}

#[derive(Clone, Debug)]
pub struct GMFlipXY {
    pub x: bool,
    pub y: bool,
}

impl GMFlipXY {
    pub fn new() -> Self {
        Self {
            x: false,
            y: false,
        }
    }

    pub fn send_message(&mut self, method: &str, value: GMValue) -> GMValue {
        // TODO: use full message, with tag
        match method {
            "get" => {
                return self.clone().into();
            }
            "set" => {
                *self = value.into_flipxy();
            }
            "toggle" => {
                self.x = !self.x;
                self.y = !self.y;
            }
            "get_xy" => {
                let x: GMValue = self.x.into();
                let y: GMValue = self.y.into();
                return x.chain(y);
            }
            "set_xy" => {
                let mut vec_deque = value.to_vec_deque();
                self.x = vec_deque.pop_front().unwrap().into_bool();
                self.y = vec_deque.pop_front().unwrap().into_bool();
            }

            "get_x" => {
                return self.x.into();
            }
            "set_x" => {
                self.x = value.into_bool();
            }
            "toggle_x" => {
                self.x = !self.x;
            }
            "get_y" => {
                return self.y.into();
            }
            "set_y" => {
                self.y = value.into_bool();
            }
            "toggle_y" => {
                self.y = !self.y;
            }
            _ => {
                error_panic(&format!("GMFlipXY::send_message, unknown method: '{}'", method));
            }
        }

        GMValue::None
    }
}

