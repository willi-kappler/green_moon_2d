use std::fmt::Debug;
use std::any::Any;

use log::debug;

use crate::context::GMContext;
use crate::sprite::GMSpriteBase;
use crate::math::GMVec2D;
use crate::util::GMRepetition;

pub trait GMSpriteEffectT: Debug {
    fn update(&mut self, _sprite: &mut GMSpriteBase, _context: &mut GMContext) {
    }

    fn draw(&self, _sprite: &GMSpriteBase, _context: &mut GMContext) {
    }

    fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
    }
}

#[derive(Debug)]
pub struct GMSpriteEffectEmpty {
}

impl GMSpriteEffectEmpty {
    pub fn new() -> Self {
        debug!("GMSpriteEffectEmpty::new()");

        Self { }
    }
}

impl GMSpriteEffectT for GMSpriteEffectEmpty {
}

#[derive(Debug)]
pub struct GMSpriteEffectLinearMovement {
    start: GMVec2D,
    end: GMVec2D,
    direction: GMVec2D,
    factor: f32,
    speed: f32,
    repetition: GMRepetition,
}

impl GMSpriteEffectLinearMovement {
    pub fn new(start: GMVec2D, end: GMVec2D, speed: f32) -> Self {
        debug!("GMSpriteEffectLinearMovement::new()");

        let direction = end - start;

        Self {
            start,
            end,
            direction,
            factor: 0.0,
            speed,
            repetition: GMRepetition::OnceForward,
        }
    }
}

impl GMSpriteEffectT for GMSpriteEffectLinearMovement {
    fn update(&mut self, sprite: &mut GMSpriteBase, _context: &mut GMContext) {
        match self.repetition {
            GMRepetition::OnceForward => {
                if self.factor < 1.0 {
                    *sprite.get_position_mut() = self.start + (self.direction * self.factor);
                    self.factor += self.speed;
                }
            }
            GMRepetition::OnceBackward => {
                if self.factor > 0.0 {
                    *sprite.get_position_mut() = self.start + (self.direction * self.factor);
                    self.factor -= self.speed;
                }
            }
            GMRepetition::LoopForward => {
                *sprite.get_position_mut() = self.start + (self.direction * self.factor);
                self.factor += self.speed;

                if self.factor > 1.0 {
                    self.factor = 0.0;
                }
            }
            GMRepetition::LoopBackward => {
                *sprite.get_position_mut() = self.start + (self.direction * self.factor);
                self.factor -= self.speed;

                if self.factor < 0.0 {
                    self.factor = 1.0;
                }
            }
            GMRepetition::PingPongForward => {
                *sprite.get_position_mut() = self.start + (self.direction * self.factor);
                self.factor += self.speed;

                if self.factor > 1.0 {
                    self.factor = 0.0;
                    self.repetition = GMRepetition::PingPongBackward;
                }
            }
            GMRepetition::PingPongBackward => {
                *sprite.get_position_mut() = self.end - (self.direction * self.factor);
                self.factor += self.speed;

                if self.factor > 1.0 {
                    self.factor = 0.0;
                    self.repetition = GMRepetition::PingPongForward;
                }
            }
        }
    }

    fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
        // TODO: implement
        todo!();
    }
}

// TODO: GMSpriteEffectCircularMovement
