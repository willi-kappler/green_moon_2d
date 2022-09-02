use std::fmt::Debug;
use std::any::Any;

use log::debug;

use crate::context::GMContext;
use crate::sprite::GMSprite;
use crate::math::GMVec2D;
use crate::util::GMRepetition;

pub trait GMSpriteEffectT: Debug {
    fn update(&mut self, _sprite: &mut GMSprite, _context: &mut GMContext) {
    }

    fn draw(&self, _sprite: &GMSprite, _context: &mut GMContext) {
    }

    fn send_message(&mut self, _message: &str, _data: Option<Box<dyn Any>>, _context: &mut GMContext) {
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
pub struct GMSpriteEffectLinear {
    start: GMVec2D,
    end: GMVec2D,
    direction: GMVec2D,
    factor: f32,
    speed: f32,
    repetition: GMRepetition,
}

impl GMSpriteEffectLinear {
    pub fn new(start: GMVec2D, end: GMVec2D, speed: f32) -> Self {
        let direction = end - start;

        Self {
            start,
            end,
            direction,
            factor: 0.0,
            speed,
            repetition: GMRepetition::Once,
        }
    }
}

impl GMSpriteEffectT for GMSpriteEffectLinear {
    fn update(&mut self, sprite: &mut GMSprite, _context: &mut GMContext) {
        match self.repetition {
            GMRepetition::Once => {
                if self.factor < 1.0 {
                    sprite.set_position(self.start + (self.direction * self.factor));
                    self.factor += self.speed;
                }
            }
            GMRepetition::Loop => {
                sprite.set_position(self.start + (self.direction * self.factor));
                self.factor += self.speed;

                if self.factor > 1.0 {
                    self.factor = 0.0;
                }
            }
            GMRepetition::PingPongForward => {
                sprite.set_position(self.start + (self.direction * self.factor));
                self.factor += self.speed;

                if self.factor > 1.0 {
                    self.factor = 0.0;
                    self.repetition = GMRepetition::PingPongBackward;
                }
            }
            GMRepetition::PingPongBackward => {
                sprite.set_position(self.end - (self.direction * self.factor));
                self.factor += self.speed;

                if self.factor > 1.0 {
                    self.factor = 0.0;
                    self.repetition = GMRepetition::PingPongForward;
                }
            }
        }
    }

    fn send_message(&mut self, _message: &str, _data: Option<Box<dyn Any>>, _context: &mut GMContext) {
    }
}
