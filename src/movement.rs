

use std::fmt::{self, Debug, Formatter};
use std::any::Any;
use std::time::Instant;

use crate::GMContext;


pub fn point_inside(x_min: f32, y_min: f32, x_max: f32, y_max: f32, px: f32, py: f32) -> bool {
    (x_min <= px) && (px <= x_max) && (y_min <= py) && (py <= y_max)
}

#[derive(Clone, Debug)]
pub struct GMMovementInner {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for GMMovementInner {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0, width: 0.0, height: 0.0 }
    }
}

impl GMMovementInner {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height, ..Default::default() }
    }

    pub fn collides_rect(&self, other: &GMMovementInner) -> bool {
        let vlen1 = self.vx.hypot(self.vy);
        let vlen2 = other.vx.hypot(other.vy);
        let vlen_max = vlen1.max(vlen2).max(1.0);
        let count = vlen_max.ceil() as u32;

        let vx1 = self.vx / vlen_max;
        let vy1 = self.vy / vlen_max;
        let vx2 = other.vx / vlen_max;
        let vy2 = other.vx / vlen_max;

        let mut x_min1 = self.x;
        let mut y_min1 = self.y;
        let mut x_max1 = self.x + self.width;
        let mut y_max1 = self.y + self.height;

        let mut x_min2 = other.x;
        let mut y_min2 = other.y;
        let mut x_max2 = other.x + other.width;
        let mut y_max2 = other.y + other.height;

        for _ in 0..count {
            if point_inside(x_min1, y_min1, x_max1, y_max1, x_min2, y_min2) { return true };
            if point_inside(x_min1, y_min1, x_max1, y_max1, x_min2, y_max2) { return true };
            if point_inside(x_min1, y_min1, x_max1, y_max1, x_max2, y_min2) { return true };
            if point_inside(x_min1, y_min1, x_max1, y_max1, x_max2, y_max2) { return true };

            if point_inside(x_min2, y_min2, x_max2, y_max2, x_min1, y_min1) { return true };
            if point_inside(x_min2, y_min2, x_max2, y_max2, x_min1, y_max1) { return true };
            if point_inside(x_min2, y_min2, x_max2, y_max2, x_max1, y_min1) { return true };
            if point_inside(x_min2, y_min2, x_max2, y_max2, x_max1, y_max1) { return true };

            x_min1 += vx1;
            y_min1 += vy1;
            x_max1 += vx1;
            y_max1 += vy1;

            x_min2 += vx2;
            y_min2 += vy2;
            x_max2 += vx2;
            y_max2 += vy2;
        }

        false
    }

    pub fn bounce_x(&mut self) {
        self.vx = -self.vx
    }

    pub fn bounce_y(&mut self) {
        self.vy = -self.vy
    }
}

#[derive(Debug)]
pub enum GMMovementRefType<'a> {
    ResetVelocity(&'a GMResetVelocity),
    ConstVelocity(&'a GMConstVelocity),
    ConstAcceleration(&'a GMConstAcceleration),
    StopAtBounds(&'a GMStopAtBounds),
    WrapAroundBounds(&'a GMWrapAroundBounds),
    BounceBounds(&'a GMMovementBounceBounds),
    Circular(&'a GMMovementCircular),
    Force(&'a GMMovementForce),

    Custom(&'a dyn Any)
}

#[derive(Debug)]
pub enum GMMovementMutRefType<'a> {
    ResetVelocity(&'a mut GMResetVelocity),
    ConstVelocity(&'a mut GMConstVelocity),
    ConstAcceleration(&'a mut GMConstAcceleration),
    StopAtBounds(&'a mut GMStopAtBounds),
    WrapAroundBounds(&'a mut GMWrapAroundBounds),
    BounceBounds(&'a mut GMMovementBounceBounds),
    Circular(&'a mut GMMovementCircular),
    Force(&'a mut GMMovementForce),

    Custom(&'a mut dyn Any)
}

pub trait GMMovementT {
    fn update(&mut self, _movement_inner: &mut GMMovementInner, _context: &mut GMContext) {}

    fn set_active(&mut self, _active: bool) {}

    fn box_clone(&self) -> Box<dyn GMMovementT>;

    fn cast_ref(&self) -> GMMovementRefType;

    fn cast_mut_ref(&mut self) -> GMMovementMutRefType;
}

impl Clone for Box<dyn GMMovementT> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Debug for Box<dyn GMMovementT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GMMovementT")
    }
}

#[derive(Clone, Debug)]
pub struct GMResetVelocity {
    active: bool,
}

impl GMResetVelocity {
    pub fn new() -> Self {
        Self {
            active: true,
        }
    }
}

impl GMMovementT for GMResetVelocity {
    fn update(&mut self, movement_inner: &mut GMMovementInner, _context: &mut GMContext) {
        if self.active {
            movement_inner.vx = 0.0;
            movement_inner.vy = 0.0;
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn cast_ref(&self) -> GMMovementRefType {
        GMMovementRefType::ResetVelocity(self)
    }

    fn cast_mut_ref(&mut self) -> GMMovementMutRefType {
        GMMovementMutRefType::ResetVelocity(self)
    }
}



#[derive(Clone, Debug)]
pub struct GMConstVelocity {
    active: bool,
}

impl GMConstVelocity {
    pub fn new() -> Self {
        Self {
            active: true,
        }
    }
}

impl GMMovementT for GMConstVelocity {
    fn update(&mut self, movement_inner: &mut GMMovementInner, _context: &mut GMContext) {
        if self.active {
            movement_inner.x += movement_inner.vx;
            movement_inner.y += movement_inner.vy;
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn cast_ref(&self) -> GMMovementRefType {
        GMMovementRefType::ConstVelocity(self)
    }

    fn cast_mut_ref(&mut self) -> GMMovementMutRefType {
        GMMovementMutRefType::ConstVelocity(self)
    }
}

#[derive(Clone, Debug)]
pub struct GMConstAcceleration {
    pub ax: f32,
    pub ay: f32,
    pub active: bool,
}

impl GMConstAcceleration {
    pub fn new(ax: f32, ay: f32) -> Self {
        Self {
            ax,
            ay,
            active: true,
        }
    }
}

impl GMMovementT for GMConstAcceleration {
    fn update(&mut self, movement_inner: &mut GMMovementInner, _context: &mut GMContext) {
        if self.active {
            movement_inner.vx += self.ax;
            movement_inner.vy += self.ay;
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn cast_ref(&self) -> GMMovementRefType {
        GMMovementRefType::ConstAcceleration(self)
    }

    fn cast_mut_ref(&mut self) -> GMMovementMutRefType {
        GMMovementMutRefType::ConstAcceleration(self)
    }
}

#[derive(Clone, Debug)]
pub struct GMStopAtBounds {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub active: bool,
}

impl GMStopAtBounds {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
            active: true,
        }
    }
}

impl GMMovementT for GMStopAtBounds {
    fn update(&mut self, movement_inner: &mut GMMovementInner, _context: &mut GMContext) {
        if movement_inner.x <= self.min_x {
            movement_inner.x = self.min_x;
            movement_inner.vx = 0.0;
        } else if movement_inner.x >= self.max_x - movement_inner.width {
            movement_inner.x = self.max_x - movement_inner.width;
            movement_inner.vx = 0.0;
        }

        if movement_inner.y <= self.min_y {
            movement_inner.y = self.min_y;
            movement_inner.vy = 0.0;
        } else if movement_inner.y >= self.max_y - movement_inner.height {
            movement_inner.y = self.max_y - movement_inner.height;
            movement_inner.vy = 0.0;
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();
        Box::new(result)
    }

    fn cast_ref(&self) -> GMMovementRefType {
        GMMovementRefType::StopAtBounds(self)
    }

    fn cast_mut_ref(&mut self) -> GMMovementMutRefType {
        GMMovementMutRefType::StopAtBounds(self)
    }
}

#[derive(Clone, Debug)]
pub struct GMWrapAroundBounds {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub active: bool,
}

impl GMWrapAroundBounds {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
            active: true,
        }
    }
}

impl GMMovementT for GMWrapAroundBounds {
    fn update(&mut self, movement_inner: &mut GMMovementInner, _context: &mut GMContext) {
        if self.active {

            if movement_inner.x > self.max_x {
                movement_inner.x -= self.max_x - self.min_x;
            } else if movement_inner.x < self.min_x - movement_inner.width {
                movement_inner.x += self.max_x - self.min_x;
            }

            if movement_inner.y > self.max_y {
                movement_inner.y -= self.max_y - self.min_y;
            } else if movement_inner.y <  self.min_y - movement_inner.height {
                movement_inner.y += self.max_y - self.min_y;
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn cast_ref(&self) -> GMMovementRefType {
        GMMovementRefType::WrapAroundBounds(self)
    }

    fn cast_mut_ref(&mut self) -> GMMovementMutRefType {
        GMMovementMutRefType::WrapAroundBounds(self)
    }
}

#[derive(Clone, Debug)]
pub struct GMMovementBounceBounds {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub active: bool,
}

impl GMMovementBounceBounds {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
            active: true,
        }
    }
}

impl GMMovementT for GMMovementBounceBounds {
    fn update(&mut self, movement_inner: &mut GMMovementInner, _context: &mut GMContext) {
        if self.active {

            if movement_inner.x > self.max_x - movement_inner.width {
                movement_inner.x = self.max_x - movement_inner.width;
                let vx = movement_inner.vx.abs();
                movement_inner.vx = -vx;
            } else if movement_inner.x < self.min_x {
                movement_inner.x = self.min_x;
                let vx = movement_inner.vx.abs();
                movement_inner.vx = vx;
            }

            if movement_inner.y > self.max_y - movement_inner.height {
                movement_inner.y = self.max_y - movement_inner.height;
                let vy = movement_inner.vy.abs();
                movement_inner.vy = -vy;
            } else if movement_inner.y < self.min_y {
                movement_inner.y = self.min_y;
                let vy = movement_inner.vy.abs();
                movement_inner.vy = vy;
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn cast_ref(&self) -> GMMovementRefType {
        GMMovementRefType::BounceBounds(self)
    }

    fn cast_mut_ref(&mut self) -> GMMovementMutRefType {
        GMMovementMutRefType::BounceBounds(self)
    }
}

#[derive(Clone, Debug)]
pub struct GMMovementCircular {
    pub cx: f32,
    pub cy: f32,
    pub radius: f32,
    pub angle: f32,
    pub v_angle: f32,
    pub active: bool,
}

impl GMMovementCircular {
    pub fn new(cx: f32, cy: f32, radius: f32, angle: f32, v_angle: f32) -> Self {
        Self {
            cx,
            cy,
            radius,
            angle,
            v_angle,
            active: true,
        }
    }
}

impl GMMovementT for GMMovementCircular {
    fn update(&mut self, movement_inner: &mut GMMovementInner, _context: &mut GMContext) {
        if self.active {
            self.angle += self.v_angle;
            let new_x = self.cx + (self.angle.cos() * self.radius);
            let new_y = self.cy + (self.angle.sin() * self.radius);

            movement_inner.vx = new_x - movement_inner.x;
            movement_inner.vy = new_y - movement_inner.y;
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn cast_ref(&self) -> GMMovementRefType {
        GMMovementRefType::Circular(self)
    }

    fn cast_mut_ref(&mut self) -> GMMovementMutRefType {
        GMMovementMutRefType::Circular(self)
    }
}

#[derive(Clone, Debug)]
pub struct GMMovementForce {
    pub fx: f32,
    pub fy: f32,
    pub strength: f32,
    pub duration: f32,
    pub instant: Instant,
    pub active: bool,
}

impl GMMovementForce {
    pub fn new(fx: f32, fy: f32, strength: f32, duration: f32) -> Self {
        Self {
            fx,
            fy,
            strength,
            duration,
            instant: Instant::now(),
            active: true,
        }
    }
}

impl GMMovementT for GMMovementForce {
    fn update(&mut self, movement_inner: &mut GMMovementInner, _context: &mut GMContext) {
        if self.active {
            let dist_x = movement_inner.x - self.fx;
            let dist_y = movement_inner.y - self.fy;

            let dist2 = dist_x.powi(2) + dist_y.powi(2);

            if dist2 > 1.0 { // More than one pixel distance
                let dist3 = dist2.sqrt() * dist2;

                let ax = self.strength * dist_x / dist3;
                let ay = self.strength * dist_y / dist3;

                movement_inner.vx += ax;
                movement_inner.vx += ay;
            } else {
                // Less than one pixel distance doesn't make sense
                // So we set dist2 = 1.0
                let ax = self.strength * dist_x;
                let ay = self.strength * dist_y;

                movement_inner.vx += ax;
                movement_inner.vx += ay;
            }

            if self.duration > 0.0  && self.instant.elapsed().as_secs_f32() > self.duration {
                self.active = false;
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;

        if active && self.duration > 0.0 {
            self.instant = Instant::now();
        }
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();
        Box::new(result)
    }

    fn cast_ref(&self) -> GMMovementRefType {
        GMMovementRefType::Force(self)
    }

    fn cast_mut_ref(&mut self) -> GMMovementMutRefType {
        GMMovementMutRefType::Force(self)
    }
}
