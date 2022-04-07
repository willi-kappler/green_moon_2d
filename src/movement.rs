

use std::fmt::{self, Debug, Formatter};
use std::any::Any;

use crate::GMError;


#[derive(Clone, Debug)]
pub struct GMMovementInner {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl GMMovementInner {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug)]
pub enum GMMovementMessage {
    SetVx(f32),
    SetVy(f32),
    SetVxVy(f32, f32),

    SetAx(f32),
    SetAy(f32),
    SetAxAy(f32, f32),

    GetVx,
    GetVy,
    GetVxVy,

    GetAx,
    GetAy,
    GetAxAy,

    CustomProperty(String, Box<dyn Any>),
}

#[derive(Debug)]
pub enum GMMovementAnswer {
    None,

    Vx(f32),
    Vy(f32),
    VxVy(f32, f32),

    Ax(f32),
    Ay(f32),
    AxAy(f32, f32),

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

pub struct GMConstVelocity {
    pub vx: f32,
    pub vy: f32,
    active: bool,
}

impl GMConstVelocity {
    pub fn new(vx: f32, vy: f32) -> Self {
        Self {
            vx,
            vy,
            active: true,
        }
    }
}

impl GMMovementT for GMConstVelocity {
    fn update(&mut self, movement_inner: &mut GMMovementInner) {
        if self.active {
            movement_inner.x += self.vx;
            movement_inner.y += self.vy;
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = GMConstVelocity {
            vx: self.vx,
            vy: self.vy,
            active: self.active,
        };

        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetVx(x) => {
                self.vx = x;
            }
            GMMovementMessage::SetVy(y) => {
                self.vy = y;
            }
            GMMovementMessage::SetVxVy(x, y) => {
                self.vx = x;
                self.vy = y;
            }
            GMMovementMessage::GetVx => {
                return Ok(GMMovementAnswer::Vx(self.vx))
            }
            GMMovementMessage::GetVy => {
                return Ok(GMMovementAnswer::Vy(self.vy))
            }
            GMMovementMessage::GetVxVy => {
                return Ok(GMMovementAnswer::VxVy(self.vx, self.vy))
            }
            _ => {
                return Err(GMError::UnexpectedMovementMessage(message))
            }
        }

        Ok(GMMovementAnswer::None)
    }

}

pub struct GMConstAcceleration {
    pub vx: f32,
    pub vy: f32,
    pub ax: f32,
    pub ay: f32,
    active: bool,
}

impl GMConstAcceleration {
    pub fn new(vx: f32, vy: f32, ax: f32, ay: f32) -> Self {
        Self {
            vx,
            vy,
            ax,
            ay,
            active: true,
        }
    }
}

impl GMMovementT for GMConstAcceleration {
    fn update(&mut self, movement_inner: &mut GMMovementInner) {
        if self.active {
            self.vx += self.ax;
            self.vy += self.ay;
            movement_inner.x += self.vx;
            movement_inner.y += self.vy;
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = GMConstAcceleration {
            vx: self.vx,
            vy: self.vy,
            ax: self.ax,
            ay: self.ay,
            active: self.active,
        };

        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        match message {
            GMMovementMessage::SetVx(x) => {
                self.vx = x;
            }
            GMMovementMessage::SetVy(y) => {
                self.vy = y;
            }
            GMMovementMessage::SetVxVy(x, y) => {
                self.vx = x;
                self.vy = y;
            }
            GMMovementMessage::GetVx => {
                return Ok(GMMovementAnswer::Vx(self.vx))
            }
            GMMovementMessage::GetVy => {
                return Ok(GMMovementAnswer::Vy(self.vy))
            }
            GMMovementMessage::GetVxVy => {
                return Ok(GMMovementAnswer::VxVy(self.vx, self.vy))
            }
            _ => {
                return Err(GMError::UnexpectedMovementMessage(message))
            }
        }

        Ok(GMMovementAnswer::None)
    }
}

pub struct GMWrapAround {
    pub screen_width: f32,
    pub screen_height: f32,
    active: bool,
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
        let x = movement_inner.x;
        let y = movement_inner.y;

        if x > self.screen_width {
            movement_inner.x = -movement_inner.width;
        } else if x < -movement_inner.width {
            movement_inner.x = self.screen_width;
        }

        if y > self.screen_height {
            movement_inner.y = -movement_inner.height;
        } else if y < -movement_inner.height {
            movement_inner.y = self.screen_height;
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = GMWrapAround {
            screen_width: self.screen_width,
            screen_height: self.screen_height,
            active: self.active,
        };

        Box::new(result)
    }

    fn send_message(&mut self, message: GMMovementMessage) -> Result<GMMovementAnswer, GMError> {
        todo!()
    }
}

pub struct GMMovementConstVeloBounce {
    pub vx: f32,
    pub vy: f32,
    pub screen_width: f32,
    pub screen_height: f32,
    active: bool,
}

pub struct GMMovementConstAccelBounce {
    pub vx: f32,
    pub vy: f32,
    pub ax: f32,
    pub ay: f32,
    pub screen_width: f32,
    pub screen_height: f32,
    active: bool,
}

pub struct GMMovementCircular {
    pub mx: f32,
    pub my: f32,
    pub radius: f32,
    pub angle: f32,
    active: bool,
}
