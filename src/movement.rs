

use std::fmt::{self, Debug, Formatter};
use std::any::Any;
use std::time::Instant;

use crate::GMError;


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
    SetAx(f32),
    SetAy(f32),
    SetAxAy(f32, f32),

    GetAx,
    GetAy,
    GetAxAy,

    SetScreenSize(f32, f32),

    SetCircleCenter(f32, f32),
    SetCircleRadius(f32),
    SetCircleAngle(f32),
    SetCircleVAngle(f32),

    GetCircleCenter,
    GetCircleRadius,
    GetCircleAngle,
    GetCircleVAngle,

    CustomProperty(String, Box<dyn Any>),
}

#[derive(Debug)]
pub enum GMMovementAnswer {
    None,

    Ax(f32),
    Ay(f32),
    AxAy(f32, f32),

    CircleCenter(f32, f32),
    CircleRadius(f32),
    CircleAngle(f32),
    CircleVAngle(f32),

    CustomProperty(String, Box<dyn Any>),
}

pub trait GMMovementT {
    fn update(&mut self, movement_inner: &mut GMMovementInner);

    fn set_active(&mut self, active: bool);

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
    fn update(&mut self, movement_inner: &mut GMMovementInner) {
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

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            _ => {
                Ok(GMMovementAnswer::None)
            }
        }
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
    fn update(&mut self, movement_inner: &mut GMMovementInner) {
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

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
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


// TODO: block screen

#[derive(Clone, Debug)]
pub struct GMWrapAround {
    pub screen_width: f32,
    pub screen_height: f32,
    pub active: bool,
}

impl GMWrapAround {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            screen_width,
            screen_height,
            active: true,
        }
    }
}

impl GMMovementT for GMWrapAround {
    fn update(&mut self, movement_inner: &mut GMMovementInner) {
        if self.active {
            let x = movement_inner.x;
            let y = movement_inner.y;
            let width = movement_inner.width;
            let height = movement_inner.height;

            if x > self.screen_width {
                movement_inner.x = -width;
            } else if x < -width {
                movement_inner.x = self.screen_width;
            }

            if y > self.screen_height {
                movement_inner.y = -height;
            } else if y < -height {
                movement_inner.y = self.screen_height;
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

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetScreenSize(screen_width, screen_height) => {
                self.screen_width = screen_width;
                self.screen_height = screen_height;
                Ok(GMMovementAnswer::None)
            }
            _ => {
                Err(GMError::UnexpectedMovementMessage(message))
            }
        }

    }
}

#[derive(Clone, Debug)]
pub struct GMMovementBounce {
    pub screen_width: f32,
    pub screen_height: f32,
    pub active: bool,
}

impl GMMovementBounce {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            screen_width,
            screen_height,
            active: true,
        }
    }
}

impl GMMovementT for GMMovementBounce {
    fn update(&mut self, movement_inner: &mut GMMovementInner) {
        if self.active {
            let x = movement_inner.x;
            let y = movement_inner.y;
            let width = movement_inner.width;
            let height = movement_inner.height;

            if x + width >= self.screen_width {
                movement_inner.vx = -movement_inner.vx;
            } else if x <= 0.0 {
                movement_inner.vx = -movement_inner.vx;
            }

            if y + height >= self.screen_height {
                movement_inner.vy = -movement_inner.vy;
            } else if y <= 0.0 {
                movement_inner.vy = -movement_inner.vy;
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

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetScreenSize(screen_width, screen_height) => {
                self.screen_width = screen_width;
                self.screen_height = screen_height;
                Ok(GMMovementAnswer::None)
            }
            _ => {
                Err(GMError::UnexpectedMovementMessage(message))
            }
        }
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
    fn update(&mut self, movement_inner: &mut GMMovementInner) {
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

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetCircleCenter(cx, cy) => {
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
            GMMovementMessage::GetCircleCenter => {
                return Ok(GMMovementAnswer::CircleCenter(self.cx, self.cy))
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
    fn update(&mut self, movement_inner: &mut GMMovementInner) {
        if self.active {
            if self.duration <= 0.0 || (self.instant.elapsed().as_secs_f32() < self.duration) {

            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = self.clone();
        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        todo!()
    }
}
