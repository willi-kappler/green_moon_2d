use std::fmt::Debug;
use std::f32::consts::TAU;

use log::debug;

use crate::context::GMContext;
use crate::sprite::GMSpriteBase;
use crate::math::GMVec2D;
use crate::util::{GMRepetition, parse_string, error_panic};

pub trait GMSpriteEffectT: Debug {
    fn update(&mut self, _sprite: &mut GMSpriteBase, _context: &mut GMContext) {
    }

    fn draw(&self, _sprite: &GMSpriteBase, _context: &mut GMContext) {
    }

    fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
    }

    fn clone_box(&self) -> Box<dyn GMSpriteEffectT>;
}

impl Clone for Box<dyn GMSpriteEffectT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct GMSpriteEffectLinearMovement {
    start: GMVec2D,
    end: GMVec2D,
    direction: GMVec2D,
    factor: f32,
    speed: f32,
    repetition: GMRepetition,
}

impl GMSpriteEffectLinearMovement {
    pub fn new<T: Into<GMVec2D>>(start: T, end: T, speed: f32, repetition: GMRepetition) -> Self {
        assert!(speed > 0.0 && speed < 1.0, "GMSpriteEffectLinearMovement::new(), speed must be greater than zero and smaller than one");

        let start = start.into();
        let end = end.into();

        debug!("GMSpriteEffectLinearMovement::new(), start: '{:?}', end: '{:?}', speed: '{}'", start, end, speed);

        let direction = end - start;

        Self {
            start,
            end,
            direction,
            factor: 0.0,
            speed,
            repetition,
        }
    }
}

impl GMSpriteEffectT for GMSpriteEffectLinearMovement {
    fn update(&mut self, sprite: &mut GMSpriteBase, _context: &mut GMContext) {
        match self.repetition {
            GMRepetition::OnceForward => {
                if self.factor < 1.0 {
                    *sprite.position_mut() = self.start + (self.direction * self.factor);
                    self.factor += self.speed;
                }
            }
            GMRepetition::OnceBackward => {
                if self.factor > 0.0 {
                    *sprite.position_mut() = self.start + (self.direction * self.factor);
                    self.factor -= self.speed;
                }
            }
            GMRepetition::LoopForward => {
                *sprite.position_mut() = self.start + (self.direction * self.factor);
                self.factor += self.speed;

                if self.factor > 1.0 {
                    self.factor = 0.0;
                }
            }
            GMRepetition::LoopBackward => {
                *sprite.position_mut() = self.start + (self.direction * self.factor);
                self.factor -= self.speed;

                if self.factor < 0.0 {
                    self.factor = 1.0;
                }
            }
            GMRepetition::PingPongForward => {
                *sprite.position_mut() = self.start + (self.direction * self.factor);
                self.factor += self.speed;

                if self.factor > 1.0 {
                    self.factor = 1.0;
                    self.repetition = GMRepetition::PingPongBackward;
                }
            }
            GMRepetition::PingPongBackward => {
                *sprite.position_mut() = self.start + (self.direction * self.factor);
                self.factor -= self.speed;

                if self.factor > 0.0 {
                    self.factor = 0.0;
                    self.repetition = GMRepetition::PingPongForward;
                }
            }
        }
    }

    fn send_message(&mut self, message: &str, _context: &mut GMContext) {
        let (name, values) = parse_string(message);

        match name {
            "set_start" => {
                let x = values[0].parse::<f32>().unwrap();
                let y = values[1].parse::<f32>().unwrap();
                self.start.x = x;
                self.start.y = y;
            }
            "set_end" => {
                let x = values[0].parse::<f32>().unwrap();
                let y = values[1].parse::<f32>().unwrap();
                self.end.x = x;
                self.end.y = y;
            }
            "set_speed" => {
                let speed = values[0].parse::<f32>().unwrap();
                self.speed = speed;

            }
            "set_repetition" => {
                self.repetition = GMRepetition::from(values[0]);

                match self.repetition {
                    GMRepetition::OnceBackward | GMRepetition::LoopBackward => {
                        self.factor = 1.0;
                    }
                    _ => {
                        self.factor = 0.0;
                    }
                }
            }
            _ => {
                error_panic(&format!("GMTextEffectRotateChars::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMSpriteEffectT> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMSpriteEffectCircularMovement {
    center: GMVec2D,
    radius: f32,
    factor: f32,
    speed: f32,
    repetition: GMRepetition,
    // TODO: set min and max for factor to allow half circle and similar movements
}

impl GMSpriteEffectCircularMovement {
    pub fn new<T: Into<GMVec2D>>(center: T, radius: f32, speed: f32, repetition: GMRepetition) -> Self {
        assert!(speed > 0.0 && speed < 1.0, "GMSpriteEffectCircularMovement::new(), speed must be greater than zero and smaller than one");

        let center = center.into();

        debug!("GMSpriteEffectCircularMovement::new(), center: '{:?}', radius: '{}', speed: '{}'", center, radius, speed);

        Self {
            center,
            radius,
            factor: 0.0,
            speed,
            repetition,
        }
    }

    fn set_sprite_pos(&self, sprite: &mut GMSpriteBase) {
        let angle = TAU * self.factor;
        let x = self.center.x + (self.radius * angle.cos());
        let y = self.center.y + (self.radius * angle.sin());

        sprite.position_mut().set1(x, y);
    }
}

impl GMSpriteEffectT for GMSpriteEffectCircularMovement {
    fn update(&mut self, sprite: &mut GMSpriteBase, _context: &mut GMContext) {
        match self.repetition {
            GMRepetition::OnceForward => {
                if self.factor < 1.0 {
                    self.set_sprite_pos(sprite);
                    self.factor += self.speed;
                }
            }
            GMRepetition::OnceBackward => {
                if self.factor > 0.0 {
                    self.set_sprite_pos(sprite);
                    self.factor -= self.speed;
                }
            }
            GMRepetition::LoopForward => {
                self.set_sprite_pos(sprite);
                self.factor += self.speed;

                if self.factor > 1.0 {
                    self.factor = 0.0;
                }
            }
            GMRepetition::LoopBackward => {
                self.set_sprite_pos(sprite);
                self.factor -= self.speed;

                if self.factor < 0.0 {
                    self.factor = 1.0;
                }
            }
            GMRepetition::PingPongForward => {
                self.set_sprite_pos(sprite);
                self.factor += self.speed;

                if self.factor > 1.0 {
                    self.factor = 1.0;
                    self.repetition = GMRepetition::PingPongBackward;
                }
            }
            GMRepetition::PingPongBackward => {
                self.set_sprite_pos(sprite);
                self.factor -= self.speed;

                if self.factor < 0.0 {
                    self.factor = 0.0;
                    self.repetition = GMRepetition::PingPongForward;
                }
            }
        }
    }

    fn send_message(&mut self, message: &str, _context: &mut GMContext) {
        let (name, values) = parse_string(message);

        match name {
            "set_center" => {
                let x = values[0].parse::<f32>().unwrap();
                let y = values[1].parse::<f32>().unwrap();
                self.center.x = x;
                self.center.y = y;
            }
            "set_radius" => {
                let radius = values[0].parse::<f32>().unwrap();
                self.radius = radius;
            }
            "set_speed" => {
                let speed = values[0].parse::<f32>().unwrap();
                self.speed = speed;

            }
            "set_repetition" => {
                self.repetition = GMRepetition::from(values[0]);
            }
            _ => {
                error_panic(&format!("GMTextEffectRotateChars::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMSpriteEffectT> {
        Box::new(self.clone())
    }
}
