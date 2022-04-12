

use std::fmt::{self, Debug, Formatter};
use std::any::Any;
use std::time::Instant;

use crate::GMError;
use crate::GMContext;


#[derive(Clone, Debug)]
pub struct GMMovementInner {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub width: f32,
    pub height: f32,
}

impl GMMovementInner {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            width,
            height,
        }
    }
}

#[derive(Debug)]
pub enum GMMovementMessage {
    SetActive(bool),

    SetAx(f32),
    SetAy(f32),
    SetAxAy(f32, f32),

    SetCircleX(f32, f32),
    SetCircleY(f32, f32),
    SetCircleXY(f32, f32),
    SetCircleRadius(f32),
    SetCircleAngle(f32),
    SetCircleVAngle(f32),

    SetBounds(f32, f32, f32, f32),

    SetFx(f32),
    SetFy(f32),
    SetFxFy(f32, f32),
    SetStrength(f32),
    SetDuration(f32),

    SetCustomProperty(String, Box<dyn Any>),

    GetActive,

    GetAx,
    GetAy,
    GetAxAy,

    GetCircleX,
    GetCircleY,
    GetCircleXY,
    GetCircleRadius,
    GetCircleAngle,
    GetCircleVAngle,

    GetBounds,

    GetFx,
    GetFy,
    GetFxFy,
    GetStrength,
    GetDuration,

    GetCustomProperty(String),
}

#[derive(Debug)]
pub enum GMMovementAnswer {
    None,

    Active(bool),

    Ax(f32),
    Ay(f32),
    AxAy(f32, f32),

    CircleX(f32, f32),
    Circley(f32, f32),
    CircleXY(f32, f32),
    CircleRadius(f32),
    CircleAngle(f32),
    CircleVAngle(f32),

    Bounds(f32, f32, f32, f32),

    Fx(f32),
    Fy(f32),
    FxFy(f32, f32),
    Strength(f32),
    Duration(f32),

    CustomProperty(String, Box<dyn Any>),
}

pub trait GMMovementT {
    fn update(&mut self, movement_inner: &mut GMMovementInner, context: &mut GMContext);

    fn box_clone(&self) -> Box<dyn GMMovementT>;

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError>;
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

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetActive(active) => {
                self.active = active;
                Ok(GMMovementAnswer::None)
            }
            _ => {
                Err(GMError::UnexpectedMovementMessage(message))
            }
        }
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

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetActive(active) => {
                self.active = active;
            }
            GMMovementMessage::GetActive => {
                return Ok(GMMovementAnswer::Active(self.active))
            }
            _ => {
            }
        }

        Ok(GMMovementAnswer::None)
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

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetActive(active) => {
                self.active = active;
            }
            GMMovementMessage::SetAx(x) => {
                self.ax = x;
            }
            GMMovementMessage::SetAy(y) => {
                self.ay = y;
            }
            GMMovementMessage::SetAxAy(x, y) => {
                self.ax = x;
                self.ay = y;
            }
            GMMovementMessage::GetActive => {
                return Ok(GMMovementAnswer::Active(self.active))
            }
            GMMovementMessage::GetAx => {
                return Ok(GMMovementAnswer::Ax(self.ax))
            }
            GMMovementMessage::GetAy => {
                return Ok(GMMovementAnswer::Ay(self.ay))
            }
            GMMovementMessage::GetAxAy => {
                return Ok(GMMovementAnswer::AxAy(self.ax, self.ay))
            }
            _ => {
                return Err(GMError::UnexpectedMovementMessage(message))
            }
        }

        Ok(GMMovementAnswer::None)
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

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();
        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetActive(active) => {
                self.active = active;
            }
            GMMovementMessage::SetBounds(min_x, min_y, max_x, max_y) => {
                self.min_x = min_x;
                self.min_y = min_y;
                self.max_x = max_x;
                self.max_y = max_y;
            }
            GMMovementMessage::GetActive => {
                return Ok(GMMovementAnswer::Active(self.active))
            }
            GMMovementMessage::GetBounds => {
                return Ok(GMMovementAnswer::Bounds(self.min_x, self.min_y, self.max_x, self.max_y))
            }
            _ => {
                return Err(GMError::UnexpectedMovementMessage(message))
            }
        }

        Ok(GMMovementAnswer::None)
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

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetActive(active) => {
                self.active = active;
            }
            GMMovementMessage::SetBounds(min_x, min_y, max_x, max_y) => {
                self.min_x = min_x;
                self.min_y = min_y;
                self.max_x = max_x;
                self.max_y = max_y;
            }
            GMMovementMessage::GetActive => {
                return Ok(GMMovementAnswer::Active(self.active))
            }
            GMMovementMessage::GetBounds => {
                return Ok(GMMovementAnswer::Bounds(self.min_x, self.min_y, self.max_x, self.max_y))
            }
            _ => {
                return Err(GMError::UnexpectedMovementMessage(message))
            }
        }

        Ok(GMMovementAnswer::None)
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

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetActive(active) => {
                self.active = active;
            }
            GMMovementMessage::SetBounds(min_x, min_y, max_x, max_y) => {
                self.min_x = min_x;
                self.min_y = min_y;
                self.max_x = max_x;
                self.max_y = max_y;
            }
            GMMovementMessage::GetActive => {
                return Ok(GMMovementAnswer::Active(self.active))
            }
            GMMovementMessage::GetBounds => {
                return Ok(GMMovementAnswer::Bounds(self.min_x, self.min_y, self.max_x, self.max_y))
            }
            _ => {
                return Err(GMError::UnexpectedMovementMessage(message))
            }
        }

        Ok(GMMovementAnswer::None)
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

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();

        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetActive(active) => {
                self.active = active;
            }
            GMMovementMessage::SetCircleXY(cx, cy) => {
                self.cx = cx;
                self.cy = cy;
            }
            GMMovementMessage::SetCircleRadius(radius) => {
                self.radius = radius;
            }
            GMMovementMessage::SetCircleAngle(angle) => {
                self.angle = angle;
            }
            GMMovementMessage::SetCircleVAngle(v_angle) => {
                self.v_angle = v_angle;
            }
            GMMovementMessage::GetActive => {
                return Ok(GMMovementAnswer::Active(self.active))
            }
            GMMovementMessage::GetCircleXY => {
                return Ok(GMMovementAnswer::CircleXY(self.cx, self.cy))
            }
            GMMovementMessage::GetCircleRadius => {
                return Ok(GMMovementAnswer::CircleRadius(self.radius))
            }
            GMMovementMessage::GetCircleAngle => {
                return Ok(GMMovementAnswer::CircleAngle(self.angle))
            }
            GMMovementMessage::GetCircleVAngle => {
                return Ok(GMMovementAnswer::CircleVAngle(self.v_angle))
            }
            _ => {
                return Err(GMError::UnexpectedMovementMessage(message))
            }
        }

        Ok(GMMovementAnswer::None)
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

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();
        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetActive(active) => {
                self.active = active;

                if self.duration > 0.0 {
                    self.instant = Instant::now();
                }
            }
            GMMovementMessage::SetFx(fx) => {
                self.fx = fx;
            }
            GMMovementMessage::SetFy(fy) => {
                self.fy = fy;
            }
            GMMovementMessage::SetFxFy(fx,fy) => {
                self.fx = fx;
                self.fy = fy;
            }
            GMMovementMessage::SetStrength(strength) => {
                self.strength = strength;
            }
            GMMovementMessage::SetDuration(duration) => {
                self.duration = duration;
            }
            GMMovementMessage::GetActive => {
                return Ok(GMMovementAnswer::Active(self.active))
            }
            GMMovementMessage::GetFx => {
                return Ok(GMMovementAnswer::Fx(self.fx))
            }
            GMMovementMessage::GetFy => {
                return Ok(GMMovementAnswer::Fy(self.fy))
            }
            GMMovementMessage::GetFxFy => {
                return Ok(GMMovementAnswer::FxFy(self.fx, self.fy))
            }
            GMMovementMessage::GetStrength => {
                return Ok(GMMovementAnswer::Strength(self.strength))
            }
            GMMovementMessage::GetDuration => {
                return Ok(GMMovementAnswer::Duration(self.duration))
            }
            _ => {
                return Err(GMError::UnexpectedMovementMessage(message))
            }
        }

        Ok(GMMovementAnswer::None)
    }
}
