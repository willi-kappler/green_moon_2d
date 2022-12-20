// use std::fmt::Debug;
// use std::f32::consts::{TAU, PI};

// use log::debug;

/*

use crate::context::GMContext;
use crate::sprite::GMSpriteBase;
use crate::math::GMVec2D;
use crate::util::{GMRepetition, error_panic, random_range_f32};
use crate::interpolation::{GMInterpolateVec2D, GMInterpolateF32};
use crate::timer::GMTimer;
use crate::data::GMData;
use crate::effect::GMEffectT;

pub type GMBoxSpriteEffect = Box<dyn GMEffectT<GMSpriteBase>>;

#[derive(Debug, Clone)]
pub struct GMSEVelocity {
    pub velocity: GMVec2D,
    pub active: bool,
}

impl GMSEVelocity {
    pub fn new<T: Into<GMVec2D>>(velocity: T) -> Self {
        let velocity = velocity.into();

        debug!("GMSEVelocity::new(), velocity: '{:?}'", velocity);

        Self {
            velocity,
            active: true
        }
    }
}

impl GMEffectT<GMSpriteBase> for GMSEVelocity {
    fn update(&mut self, sprite: &mut GMSpriteBase, _context: &mut GMContext) {
        if self.active {
            sprite.position.add2(&self.velocity);
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_velocity" => {
                let velocity: (f32, f32) = data.into();
                self.velocity = velocity.into();
            }
            "set_random_direction" => {
                let (min, max): (f32, f32) = data.into();
                let angle = random_range_f32(min, max);
                let rad = angle * PI / 180.0;
                let x = rad.cos();
                let y = rad.sin();

                self.velocity = GMVec2D::new(x, y);
            }
            "set_random_speed" => {
                let (min, max): (f32, f32) = data.into();
                let speed = random_range_f32(min, max);
                self.velocity.mul2(speed);
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSEVelocity::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "velocity" => {
                self.velocity.into()
            }
            "active" => {
                self.active.into()
            }
            _ => {
                error_panic(&format!("GMSEVelocity::get_property(), unknown property: '{}'", name))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMSEMaxSpeed {
    pub max_speed: f32,
    active: bool,
}

impl GMSEMaxSpeed {
    pub fn new(max_speed: f32) -> Self {
        debug!("GMSEMaxSpeed::new(), max_speed: {}", max_speed);

        Self {
            max_speed,
            active: true,
        }
    }
}

impl GMEffectT<GMSpriteBase> for GMSEMaxSpeed {
    fn update(&mut self, _sprite: &mut GMSpriteBase, _context: &mut GMContext) {
        if self.active {
            // TODO: implement max speed
            todo!();
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_max_speed" => {
                self.max_speed = data.into();
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSEMaxSpeed::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "max_speed" => {
                self.max_speed.into()
            }
            "active" => {
                self.active.into()
            }
            _ => {
                error_panic(&format!("GMSEMaxSpeed::get_property(), unknown property: '{}'", name))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMSEAcceleration {
    pub velocity: GMVec2D,
    pub acceleration: GMVec2D,
    pub active: bool,
}

impl GMSEAcceleration {
    pub fn new<T: Into<GMVec2D>, U: Into<GMVec2D>>(velocity: T, acceleration: U) -> Self {
        let velocity = velocity.into();
        let acceleration = acceleration.into();

        debug!("GMSEAcceleration::new(), velocity: '{:?}', acceleration: '{:?}'", velocity, acceleration);

        Self {
            velocity,
            acceleration,
            active: true
        }
    }
}

impl GMEffectT<GMSpriteBase> for GMSEAcceleration {
    fn update(&mut self, sprite: &mut GMSpriteBase, _context: &mut GMContext) {
        if self.active {
            sprite.position.add2(&self.velocity);
            self.velocity.add2(&self.acceleration);
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_velocity" => {
                let velocity: (f32, f32) = data.into();
                self.velocity = velocity.into();
            }
            "reset_velocity" => {
                self.velocity.x = 0.0;
                self.velocity.y = 0.0;
            }
            "set_random_velocity_direction" => {
                let (min, max): (f32, f32) = data.into();
                let angle = random_range_f32(min, max);
                let rad = angle * PI / 180.0;
                let x = rad.cos();
                let y = rad.sin();

                self.velocity = GMVec2D::new(x, y);
            }
            "set_random_speed" => {
                let (min, max): (f32, f32) = data.into();
                let speed = random_range_f32(min, max);
                self.velocity.mul2(speed);
            }
            "set_acceleration" => {
                let acceleration: (f32, f32) = data.into();
                self.acceleration = acceleration.into();
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSEAcceleration::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "velocity" => {
                self.velocity.into()
            }
            "acceleration" => {
                self.acceleration.into()
            }
            "active" => {
                self.active.into()
            }
            _ => {
                error_panic(&format!("GMSEAcceleration::get_property(), unknown property: '{}'", name))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMSERotation1 {
    pub speed: f32,
    pub active: bool,
}

impl GMSERotation1 {
    pub fn new(speed: f32) -> Self {
        debug!("GMSERotation1::new(), speed: '{:?}'", speed);

        Self {
            speed,
            active: true
        }
    }
}

impl GMEffectT<GMSpriteBase> for GMSERotation1 {
    fn update(&mut self, sprite: &mut GMSpriteBase, _context: &mut GMContext) {
        if self.active {
            sprite.angle += self.speed;

            if sprite.angle > 360.0 {
                sprite.angle -= 360.0;
            } else if sprite.angle < 0.0 {
                sprite.angle += 360.0;
            }
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_speed" => {
                self.speed = data.into()
            }
            "set_random_speed" => {
                let (min, max): (f32, f32) = data.into();
                self.speed = random_range_f32(min, max);
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSERotation1::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "speed" => {
                self.speed.into()
            }
            "active" => {
                self.active.into()
            }
            _ => {
                error_panic(&format!("GMSERotation1::get_property(), unknown property: '{}'", name))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMSERotation2 {
    pub interpolate: GMInterpolateF32,
    pub active: bool,
}

impl GMSERotation2 {
    pub fn new(angle: f32, min_angle: f32, max_angle: f32, speed: f32) -> Self {
        debug!("GMSERotation2::new(), angle: '{}', min_angle: '{}', max_angle: '{}', speed: '{}'", angle, min_angle, max_angle, speed);

        let mut interpolate = GMInterpolateF32::new(min_angle, max_angle, speed);
        interpolate.repetition = GMRepetition::PingPongForward;
        interpolate.set_value(angle);

        Self {
            interpolate,
            active: true,
        }
    }
}

impl GMEffectT<GMSpriteBase> for GMSERotation2 {
    fn update(&mut self, base: &mut GMSpriteBase, _context: &mut GMContext) {
        if self.active {
            self.interpolate.update();
            base.angle = self.interpolate.get_value();
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_angle" => {
                let angle = data.into();
                self.interpolate.set_value(angle);
            }
            "set_min_angle" => {
                let min_angle = data.into();
                self.interpolate.set_start(min_angle);
            }
            "set_max_angle" => {
                let max_angle = data.into();
                self.interpolate.set_end(max_angle);
            }
            "set_speed" => {
                let speed = data.into();
                self.interpolate.set_speed(speed);
            }
            "set_random_speed" => {
                let (min, max): (f32, f32) = data.into();
                let speed = random_range_f32(min, max);
                self.interpolate.set_speed(speed);
            }
            "set_repetition" => {
                self.interpolate.repetition = data.into();
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSERotation2::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "angle" => {
                self.interpolate.get_value().into()
            }
            "min_angle" => {
                self.interpolate.get_start().into()
            }
            "max_angle" => {
                self.interpolate.get_end().into()
            }
            "speed" => {
                self.interpolate.get_speed().into()
            }
            "active" => {
                self.active.into()
            }
            _ => {
                error_panic(&format!("GMSERotation2::get_property(), unknown property: '{}'", name))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMSELinearMovement {
    pub interpolation: GMInterpolateVec2D,
    pub active: bool,
}

impl GMSELinearMovement {
    pub fn new<T: Into<GMVec2D>>(start: T, end: T, speed: f32, repetition: GMRepetition) -> Self {
        let start = start.into();
        let end = end.into();

        debug!("GMSELinearMovement::new(), start: '{:?}', end: '{:?}', speed: '{}'", start, end, speed);

        let mut interpolation = GMInterpolateVec2D::new(start, end, speed);
        interpolation.repetition = repetition;
        interpolation.reset();

        Self {
            interpolation,
            active: true,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.interpolation.is_finished()
    }
}

impl GMEffectT<GMSpriteBase> for GMSELinearMovement {
    fn update(&mut self, sprite: &mut GMSpriteBase, _context: &mut GMContext) {
        if self.active {
            self.interpolation.update();
            sprite.position = self.interpolation.get_position();
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_start" => {
                let start = data.into();
                self.interpolation.set_start(start);
            }
            "set_end" => {
                let start = data.into();
                self.interpolation.set_end(start);
            }
            "set_speed" => {
                let start = data.into();
                self.interpolation.set_speed(start);
            }
            "set_repetition" => {
                let repetition = data.into();
                self.interpolation.repetition = repetition;
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSELinearMovement::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "start" => {
                self.interpolation.get_start().into()
            }
            "end" => {
                self.interpolation.get_end().into()
            }
            "speed" => {
                self.interpolation.get_speed().into()
            }
            "active" => {
                self.active.into()
            }
            _ => {
                error_panic(&format!("GMSELinearMovement::get_property(), unknown property: '{}'", name))
            }
        }
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
        assert!(positions.len() >= 3, "GMSEPolygonMovement::new(), at least three positions expected: {:?}", positions);
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

    pub fn reset_movement(&mut self, repetition: GMRepetition) {
        let start = self.positions[self.current_index];
        let end = self.positions[self.current_index + 1];
        let speed = self.speeds[self.current_index];
        self.linear_movement.interpolation.set_start_end_speed(start, end, speed);
        self.linear_movement.interpolation.repetition = repetition;
        self.linear_movement.interpolation.reset();
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
                            self.reset_movement(GMRepetition::OnceForward);
                        }
                    }
                }
                GMRepetition::OnceBackward => {
                    if self.linear_movement.is_finished() {
                        if self.current_index > 0 {
                            self.current_index -= 1;
                            self.reset_movement(GMRepetition::OnceBackward);
                        }
                    }
                }
                GMRepetition::LoopForward => {
                    if self.linear_movement.is_finished() {
                        self.current_index += 1;
                        if self.current_index >= self.speeds.len() {
                            self.current_index = 0;
                        }
                        self.reset_movement(GMRepetition::OnceForward);
                    }
                }
                GMRepetition::LoopBackward => {
                    if self.linear_movement.is_finished() {
                        self.current_index -= 1;
                        if self.current_index >= self.speeds.len() {
                            self.current_index = self.speeds.len() - 1;
                        }
                        self.reset_movement(GMRepetition::OnceBackward);
                    }
                }
                GMRepetition::PingPongForward => {
                    if self.linear_movement.is_finished() {
                        self.current_index += 1;
                        if self.current_index >= self.speeds.len() {
                            self.current_index = self.speeds.len() - 1;
                            self.reset_movement(GMRepetition::OnceBackward);
                            self.repetition = GMRepetition::PingPongBackward;
                        } else {
                            self.reset_movement(GMRepetition::OnceForward);
                        }
                    }
                }
                GMRepetition::PingPongBackward => {
                    if self.linear_movement.is_finished() {
                        self.current_index -= 1;
                        if self.current_index >= self.speeds.len() {
                            self.current_index = 0;
                            self.reset_movement(GMRepetition::OnceForward);
                            self.repetition = GMRepetition::PingPongForward;
                        } else {
                            self.reset_movement(GMRepetition::OnceBackward);
                        }
                    }
                }
            }
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_active" => {
                self.active = data.into();
            }
            // TODO: Add more messages, set_repetition, add_position, remove_position, ...
            _ => {
                error_panic(&format!("GMSEPolygonMovement::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "active" => {
                self.active.into()
            }
            "current_index" => {
                self.current_index.into()
            }
            _ => {
                error_panic(&format!("GMSEPolygonMovement::get_property(), unknown property: '{}'", name))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMSECircularMovement {
    pub radius: f32,
    pub factor: f32,
    pub speed: f32,
    pub repetition: GMRepetition,
    pub active: bool,
    // TODO: set min and max for factor to allow half circle and similar movements
}

impl GMSECircularMovement {
    pub fn new(radius: f32, speed: f32, repetition: GMRepetition) -> Self {
        assert!(speed > 0.0 && speed < 1.0, "GMSECircularMovement::new(), speed must be greater than zero and smaller than one");

        debug!("GMSECircularMovement::new(), radius: '{}', speed: '{}'", radius, speed);

        Self {
            radius,
            factor: 0.0,
            speed,
            repetition,
            active: true,
        }
    }

    fn set_sprite_pos(&self, sprite: &mut GMSpriteBase) {
        let angle = TAU * self.factor;
        let x = self.radius * angle.cos();
        let y = self.radius * angle.sin();

        sprite.offset.set1(x, y);
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

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
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
                error_panic(&format!("GMSECircularMovement::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "radius" => {
                self.radius.into()
            }
            "speed" => {
                self.speed.into()
            }
            "repetition" => {
                self.repetition.into()
            }
            "active" => {
                self.active.into()
            }
            _ => {
                error_panic(&format!("GMSECircularMovement::get_property(), unknown property: '{}'", name))
            }
        }
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

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_duration" => {
                self.timer.set_duration(data.into());
                self.timer.start();
            }
            "set_active" => {
                self.active = data.into();
            }
            "set_name" => {
                self.name = data.into();
            }
            _ => {
                error_panic(&format!("GMSETarget::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "duration" => {
                self.timer.get_duration().into()
            }
            "name" => {
                self.name.clone().into()
            }
            "active" => {
                self.active.into()
            }
            _ => {
                error_panic(&format!("GMSETarget::get_property(), unknown property: '{}'", name))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMSEFollow {
    pub timer: GMTimer,
    pub target_name: String,
    pub speed: f32,
    pub direction: GMVec2D,
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
            direction: GMVec2D::new(0.0, 0.0),
            active: true,
        }
    }
}

impl GMEffectT<GMSpriteBase> for GMSEFollow {
    fn update(&mut self, sprite: &mut GMSpriteBase, context: &mut GMContext) {
        if self.active {
            sprite.position.add2(&self.direction);

            if self.timer.finished() {
                let data = context.get_tag(&self.target_name).unwrap().clone();
                let position: GMVec2D = data.into();

                self.direction = position - sprite.position;
                self.direction.norm();
                self.direction.mul2(self.speed);

                self.timer.start();
            }
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_duration" => {
                self.timer.set_duration(data.into());
                self.timer.start();
            }
            "set_active" => {
                self.active = data.into();
            }
            "set_target_name" => {
                self.target_name = data.into();
            }
            "set_speed" => {
                self.speed = data.into();
            }
            _ => {
                error_panic(&format!("GMSEFollow::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "duration" => {
                self.timer.get_duration().into()
            }
            "target_name" => {
                self.target_name.clone().into()
            }
            "speed" => {
                self.speed.into()
            }
            "active" => {
                self.active.into()
            }
            _ => {
                error_panic(&format!("GMSEFollow::get_property(), unknown property: '{}'", name))
            }
        }
    }
}

#[derive(Clone)]
pub struct GMSETimed {
    pub timer: GMTimer,
    pub active: bool,
    pub repeat: bool,
    pub closure: fn(&mut GMSpriteBase, &mut GMContext) -> (),
}

impl Debug for GMSETimed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GMSETimed")
            .field("timer", &self.timer)
            .field("active", &self.active)
            .field("repeat", &self.repeat)
            .finish()
    }
}

impl GMSETimed {
    pub fn new(duration: f32, repeat: bool, closure: fn(&mut GMSpriteBase, &mut GMContext) -> ()) -> Self {
        debug!("GMSETarget::new(), duration: '{}', repeat: {}", duration, repeat);

        Self {
            timer: GMTimer::new(duration),
            active: true,
            repeat,
            closure,
        }
    }
}

impl GMEffectT<GMSpriteBase> for GMSETimed {
    fn update(&mut self, sprite: &mut GMSpriteBase, context: &mut GMContext) {
        if self.active {
            if self.timer.finished() {

                (self.closure)(sprite, context);

                if self.repeat {
                    self.timer.start();
                }
            }
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_duration" => {
                self.timer.set_duration(data.into());
                self.timer.start();
            }
            "set_active" => {
                self.active = data.into();
            }
            "set_repeat" => {
                self.repeat = data.into();
            }
            _ => {
                error_panic(&format!("GMSETimed::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "duration" => {
                self.timer.get_duration().into()
            }
            "repeat" => {
                self.repeat.into()
            }
            "active" => {
                self.active.into()
            }
            _ => {
                error_panic(&format!("GMSETimed::get_property(), unknown property: '{}'", name))
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct GMSEScaling {
    interpolate: GMInterpolateF32,
    active: bool,
}

impl GMSEScaling {
    pub fn new(size: f32, min_size: f32, max_size: f32, speed: f32) -> Self {
        debug!("GMSEScaling::new(), size: '{}', min_size: '{}', max_size: '{}', speed: '{}'", size, min_size, max_size, speed);

        let mut interpolate = GMInterpolateF32::new(min_size, max_size, speed);
        interpolate.repetition = GMRepetition::PingPongForward;
        interpolate.set_value(size);

        Self {
            interpolate,
            active: true,
        }
    }
}

impl GMEffectT<GMSpriteBase> for GMSEScaling {
    fn update(&mut self, base: &mut GMSpriteBase, _context: &mut GMContext) {
        self.interpolate.update();
        base.scale = self.interpolate.get_value();
}

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_size" => {
                let size = data.into();
                self.interpolate.set_value(size);
            }
            "set_min_size" => {
                let min_size = data.into();
                self.interpolate.set_start(min_size);
            }
            "set_max_size" => {
                let max_size = data.into();
                self.interpolate.set_end(max_size);
            }
            "set_speed" => {
                let speed = data.into();
                self.interpolate.set_speed(speed);
            }
            "set_random_speed" => {
                let (min, max): (f32, f32) = data.into();
                let speed = random_range_f32(min, max);
                self.interpolate.set_speed(speed);
            }
            "set_repetition" => {
                self.interpolate.repetition = data.into();
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSEScaling::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> GMBoxSpriteEffect {
        Box::new(self.clone())
    }


    fn get_property(&self, name: &str) -> GMData {
        match name {
            "size" => {
                self.interpolate.get_value().into()
            }
            "min_size" => {
                self.interpolate.get_start().into()
            }
            "max_size" => {
                self.interpolate.get_end().into()
            }
            "speed" => {
                self.interpolate.get_speed().into()
            }
            "repetition" => {
                self.interpolate.repetition.into()
            }
            "active" => {
                self.active.into()
            }
            _ => {
                error_panic(&format!("GMSEScaling::get_property(), unknown property: '{}'", name))
            }
        }
    }
}
*/
