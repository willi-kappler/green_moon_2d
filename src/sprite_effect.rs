use std::fmt::Debug;
use std::f32::consts::TAU;

use log::debug;

use crate::context::GMContext;
use crate::sprite::GMSpriteBase;
use crate::math::GMVec2D;
use crate::util::{GMRepetition, error_panic};
use crate::timer::GMTimer;
use crate::data::GMData;

pub trait GMSpriteEffectT: Debug {
    fn update(&mut self, _sprite: &mut GMSpriteBase, _context: &mut GMContext) {
    }

    fn draw(&self, _sprite: &GMSpriteBase, _context: &mut GMContext) {
    }

    fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
    }

    fn send_message_data(&mut self, _message: &str, _data: GMData, _context: &mut GMContext) {
    }

    fn set_active(&mut self, active: bool);

    fn clone_box(&self) -> Box<dyn GMSpriteEffectT>;
}

impl Clone for Box<dyn GMSpriteEffectT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct GMSELinearMovement {
    start: GMVec2D,
    end: GMVec2D,
    direction: GMVec2D,
    factor: f32,
    speed: f32,
    repetition: GMRepetition,
    active: bool,
}

impl GMSELinearMovement {
    pub fn new<T: Into<GMVec2D>>(start: T, end: T, speed: f32, repetition: GMRepetition) -> Self {
        assert!(speed > 0.0 && speed < 1.0, "GMSELinearMovement::new(), speed must be greater than zero and smaller than one");

        let start = start.into();
        let end = end.into();

        debug!("GMSELinearMovement::new(), start: '{:?}', end: '{:?}', speed: '{}'", start, end, speed);

        let direction = end - start;

        Self {
            start,
            end,
            direction,
            factor: 0.0,
            speed,
            repetition,
            active: true,
        }
    }
}

impl GMSpriteEffectT for GMSELinearMovement {
    fn update(&mut self, sprite: &mut GMSpriteBase, _context: &mut GMContext) {
        if self.active {
            match self.repetition {
                GMRepetition::OnceForward => {
                    if self.factor < 1.0 {
                        sprite.position = self.start + (self.direction * self.factor);
                        self.factor += self.speed;
                    }
                }
                GMRepetition::OnceBackward => {
                    if self.factor > 0.0 {
                        sprite.position = self.start + (self.direction * self.factor);
                        self.factor -= self.speed;
                    }
                }
                GMRepetition::LoopForward => {
                    sprite.position = self.start + (self.direction * self.factor);
                    self.factor += self.speed;

                    if self.factor > 1.0 {
                        self.factor = 0.0;
                    }
                }
                GMRepetition::LoopBackward => {
                    sprite.position = self.start + (self.direction * self.factor);
                    self.factor -= self.speed;

                    if self.factor < 0.0 {
                        self.factor = 1.0;
                    }
                }
                GMRepetition::PingPongForward => {
                    sprite.position = self.start + (self.direction * self.factor);
                    self.factor += self.speed;

                    if self.factor > 1.0 {
                        self.factor = 1.0;
                        self.repetition = GMRepetition::PingPongBackward;
                    }
                }
                GMRepetition::PingPongBackward => {
                    sprite.position = self.start + (self.direction * self.factor);
                    self.factor -= self.speed;

                    if self.factor > 0.0 {
                        self.factor = 0.0;
                        self.repetition = GMRepetition::PingPongForward;
                    }
                }
            }
        }
    }

    fn send_message_data(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_start" => {
                self.start = data.into();

                self.direction = self.end - self.start;
            }
            "set_end" => {
                self.end = data.into();

                self.direction = self.end - self.start;
            }
            "set_speed" => {
                self.speed = data.into();

            }
            "set_repetition" => {
                self.repetition = data.into();

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
                error_panic(&format!("GMSELinearMovement::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMSpriteEffectT> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMSECircularMovement {
    center: GMVec2D,
    radius: f32,
    factor: f32,
    speed: f32,
    repetition: GMRepetition,
    active: bool,
    // TODO: set min and max for factor to allow half circle and similar movements
}

impl GMSECircularMovement {
    pub fn new<T: Into<GMVec2D>>(center: T, radius: f32, speed: f32, repetition: GMRepetition) -> Self {
        assert!(speed > 0.0 && speed < 1.0, "GMSECircularMovement::new(), speed must be greater than zero and smaller than one");

        let center = center.into();

        debug!("GMSECircularMovement::new(), center: '{:?}', radius: '{}', speed: '{}'", center, radius, speed);

        Self {
            center,
            radius,
            factor: 0.0,
            speed,
            repetition,
            active: true,
        }
    }

    fn set_sprite_pos(&self, sprite: &mut GMSpriteBase) {
        let angle = TAU * self.factor;
        let x = self.center.x + (self.radius * angle.cos());
        let y = self.center.y + (self.radius * angle.sin());

        sprite.position.set1(x, y);
    }
}

impl GMSpriteEffectT for GMSECircularMovement {
    fn update(&mut self, sprite: &mut GMSpriteBase, _context: &mut GMContext) {
        if self.active {
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
    }

    fn send_message_data(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_center" => {
                self.center = data.into();
            }
            "set_radius" => {
                self.radius = data.into();
            }
            "set_speed" => {
                self.speed = data.into();

            }
            "set_repetition" => {
                self.repetition = data.into();
            }
            _ => {
                error_panic(&format!("GMSECircularMovement::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMSpriteEffectT> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMSETarget {
    timer: GMTimer,
    name: String,
    active: bool,
}

impl GMSETarget {
    pub fn new<T: Into<String>>(duration: f32, name: T) -> Self {
        Self {
            timer: GMTimer::new(duration),
            name: name.into(),
            active: true,
        }
    }
}

impl GMSpriteEffectT for GMSETarget {
    fn update(&mut self, sprite: &mut GMSpriteBase, context: &mut GMContext) {
        if self.active {
            if self.timer.finished() {
                let position = sprite.position;
                context.set_tag(&self.name, position.into());

                self.timer.start();
            }
        }
    }

    fn send_message_data(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_duration" => {
                self.timer.set_duration(data.into());
                self.timer.start();
            }
            _ => {
                error_panic(&format!("GMSETarget::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMSpriteEffectT> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMSEFollow {
    timer: GMTimer,
    target_name: String,
    speed: f32,
    active: bool,
}

impl GMSEFollow {
    pub fn new<T: Into<String>>(duration: f32, name: T, speed: f32) -> Self {
        Self {
            timer: GMTimer::new(duration),
            target_name: name.into(),
            speed,
            active: true,
        }
    }
}

impl GMSpriteEffectT for GMSEFollow {
    fn update(&mut self, sprite: &mut GMSpriteBase, context: &mut GMContext) {
        if self.active {
            if self.timer.finished() {
                let data = context.get_tag(&self.target_name).unwrap().clone();
                let position: GMVec2D = data.into();

                let mut direction = position - sprite.position;
                direction.norm();
                direction.mul2(self.speed);
                sprite.velocity.set3(direction);

                self.timer.start();
            }
        }
    }

    fn send_message_data(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_duration" => {
                self.timer.set_duration(data.into());
                self.timer.start();
            }
            _ => {
                error_panic(&format!("GMSEFollow::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMSpriteEffectT> {
        Box::new(self.clone())
    }
}
