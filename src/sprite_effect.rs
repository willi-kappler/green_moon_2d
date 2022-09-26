use std::fmt::Debug;
use std::f32::consts::TAU;

use log::debug;

use crate::context::GMContext;
use crate::sprite::GMSpriteBase;
use crate::math::GMVec2D;
use crate::util::{GMRepetition, error_panic};
use crate::timer::GMTimer;
use crate::data::GMData;
use crate::effect::GMEffectT;


#[derive(Debug, Clone)]
pub struct GMSELinearMovement {
    pub start: GMVec2D,
    pub end: GMVec2D,
    pub direction: GMVec2D,
    pub factor: f32,
    pub speed: f32,
    pub repetition: GMRepetition,
    pub active: bool,
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

    pub fn is_finished(&self) -> bool {
        match self.repetition {
            GMRepetition::OnceForward => {
                self.factor == 1.0
            }
            GMRepetition::OnceBackward => {
                self.factor == 0.0
            }
            _ => {
                false
            }
        }
    }
}

impl GMEffectT<GMSpriteBase> for GMSELinearMovement {
    fn update(&mut self, sprite: &mut GMSpriteBase, _context: &mut GMContext) {
        if self.active {
            match self.repetition {
                GMRepetition::OnceForward => {
                    if self.factor < 1.0 {
                        sprite.position = self.start + (self.direction * self.factor);
                        self.factor += self.speed;
                        if self.factor > 1.0 {
                            self.factor = 1.0;
                        }
                    }
                }
                GMRepetition::OnceBackward => {
                    if self.factor > 0.0 {
                        sprite.position = self.start + (self.direction * self.factor);
                        self.factor -= self.speed;
                        if self.factor < 0.0 {
                            self.factor = 0.0;
                        }
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
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSELinearMovement::send_message_data(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMSpriteBase>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMSEPolygonMovement {
    pub positions: Vec<GMVec2D>,
    pub speeds: Vec<f32>,
    pub repetition: GMRepetition,
    pub active: bool,
    pub current_index: usize,
    pub linear_movement: GMSELinearMovement,
}

impl GMSEPolygonMovement {
    pub fn new(positions: Vec<GMVec2D>, speeds: Vec<f32>, repetition: GMRepetition) -> Self {
        assert!(positions.len() > 2, "GMSEPolygonMovement::new(), at least two positions expected: {:?}", positions);
        assert!(positions.len() == speeds.len() + 1, "GMSEPolygonMovement::new(), number of speeds must be one less than number of positions: {:?}", speeds);

        debug!("GMSEPolygonMovement::new(), positions: {:?}, speeds: {:?}, repetition: {:?}", positions, speeds, repetition);

        let linear_movement = GMSELinearMovement::new(
        positions[0].clone(),
          positions[1].clone(),
        speeds[0],
   GMRepetition::OnceForward);

        Self {
            positions,
            speeds,
            repetition,
            active: true,
            current_index: 0,
            linear_movement,
        }
    }

    pub fn reset_movement(&mut self) {
        self.linear_movement.start = self.positions[self.current_index];
        self.linear_movement.end = self.positions[self.current_index + 1];
        self.linear_movement.direction = self.linear_movement.end - self.linear_movement.start;
        self.linear_movement.speed = self.speeds[self.current_index];
    }
}

impl GMEffectT<GMSpriteBase> for GMSEPolygonMovement {
    fn update(&mut self, sprite: &mut GMSpriteBase, context: &mut GMContext) {
        if self.active {
            self.linear_movement.update(sprite, context);

            match self.repetition {
                GMRepetition::OnceForward => {
                    if self.linear_movement.is_finished() {
                        if self.current_index < self.speeds.len() - 1 {
                            self.current_index += 1;
                            self.reset_movement();
                            self.linear_movement.factor = 0.0;
                        }
                    }
                }
                GMRepetition::OnceBackward => {
                    todo!();
                }
                GMRepetition::LoopForward => {
                    if self.linear_movement.is_finished() {
                        self.current_index += 1;
                        if self.current_index >= self.speeds.len() {
                            self.current_index = 0;
                            self.reset_movement();
                            self.linear_movement.factor = 0.0;
                        }
                    }
                }
                GMRepetition::LoopBackward => {
                    todo!();
                }
                GMRepetition::PingPongForward => {
                    todo!();
                }
                GMRepetition::PingPongBackward => {
                    todo!();
                }
            }
        }
    }

    fn send_message_data(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSEPolygonMovement::send_message_data(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMSpriteBase>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMSECircularMovement {
    pub center: GMVec2D,
    pub radius: f32,
    pub factor: f32,
    pub speed: f32,
    pub repetition: GMRepetition,
    pub active: bool,
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

impl GMEffectT<GMSpriteBase> for GMSECircularMovement {
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
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSECircularMovement::send_message_data(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMSpriteBase>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMSETarget {
    pub timer: GMTimer,
    pub name: String,
    pub active: bool,
}

impl GMSETarget {
    pub fn new<T: Into<String>>(duration: f32, name: T) -> Self {
        let name = name.into();
        debug!("GMSETarget::new(), duration: '{}', name: '{}'", duration, name);

        Self {
            timer: GMTimer::new(duration),
            name,
            active: true,
        }
    }
}

impl GMEffectT<GMSpriteBase> for GMSETarget {
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
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSETarget::send_message_data(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMSpriteBase>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMSEFollow {
    pub timer: GMTimer,
    pub target_name: String,
    pub speed: f32,
    pub active: bool,
}

impl GMSEFollow {
    pub fn new<T: Into<String>>(duration: f32, name: T, speed: f32) -> Self {
        let name = name.into();
        debug!("GMSEFollow::new(), duration: '{}', name: '{}', speed: '{}'", duration, name, speed);

        Self {
            timer: GMTimer::new(duration),
            target_name: name.into(),
            speed,
            active: true,
        }
    }
}

impl GMEffectT<GMSpriteBase> for GMSEFollow {
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
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSEFollow::send_message_data(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMSpriteBase>> {
        Box::new(self.clone())
    }
}
