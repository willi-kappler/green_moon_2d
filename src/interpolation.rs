

use hecs::World;

use crate::util::{GMRepetition, GMActive};
use crate::math::{GMVec2D, GMAngle, GMPosition, GMCircle};


#[derive(Debug, Clone)]
pub struct GMInterpolateF32 {
    start: f32,
    end: f32,
    speed: f32,
    value: f32,
    pub repetition: GMRepetition,
}

impl GMInterpolateF32 {
    pub fn new(start: f32, end: f32, speed: f32) -> Self {
        let result = Self {
            start,
            end,
            speed,
            value: start,
            repetition: GMRepetition::OnceForward,
        };

        result.check_bounds();
        result
    }

    pub fn update(&mut self) {
        match self.repetition {
            GMRepetition::OnceForward => {
                self.value += self.speed;
                if self.value > self.end {
                    self.value = self.end;
                }
            }
            GMRepetition::OnceBackward => {
                self.value -= self.speed;
                if self.value < self.start {
                    self.value = self.start;
                }
            }
            GMRepetition::LoopForward => {
                self.value += self.speed;
                if self.value > self.end {
                    self.value = self.start;
                }
            }
            GMRepetition::LoopBackward => {
                self.value -= self.speed;
                if self.value < self.start {
                    self.value = self.end;
                }
            }
            GMRepetition::PingPongForward => {
                self.value += self.speed;
                if self.value > self.end {
                    self.value = self.end;
                    self.repetition = GMRepetition::PingPongBackward;
                }

            }
            GMRepetition::PingPongBackward => {
                self.value -= self.speed;
                if self.value < self.start {
                    self.value = self.start;
                    self.repetition = GMRepetition::PingPongForward;
                }
            }
        }
    }

    fn check_bounds(&self) {
        assert!(self.end > self.start, "GMInterpolateF32::check_bounds(), end ('{}') must be greater than start ('{}')", self.end, self.start);
        self.check_speed();
    }

    fn check_speed(&self) {
        let difference = self.end - self.start;
        assert!(self.speed > 0.0 && self.speed < difference, "GMInterpolateF32::check_speed(), speed must be between 0.0 and {}", difference);
    }

    pub fn set_start(&mut self, start: f32) {
        self.start = start;
        self.check_bounds();
    }

    pub fn get_start(&self) -> f32 {
        self.start
    }

    pub fn set_end(&mut self, end: f32) {
        self.end = end;
        self.check_bounds();
    }

    pub fn get_end(&self) -> f32 {
        self.end
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
        self.check_speed();
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn set_value(&mut self, value: f32) {
        assert!(value >= self.start && value <= self.end, "GMInterpolateF32::set_value(), value must be between {} and {}", self.start, self.end);

        self.value = value;
    }

    pub fn set_value_norm(&mut self, value: f32) {
        assert!(value >= 0.0 && value <= 1.0, "GMInterpolateF32::set_value_norm(), value must be between 0.0 and 1.0");

        let difference = self.end - self.start;
        self.value = self.start + (difference * value);
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn is_finished(&self) -> bool {
        match self.repetition {
            GMRepetition::OnceForward => {
                self.value == self.end
            }
            GMRepetition::OnceBackward => {
                self.value == self.start
            }
            _ => {
                false
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMInterpolateVec2D {
    start: GMVec2D,
    end: GMVec2D,
    vector: GMVec2D,
    speed: f32,
    direction: GMVec2D,
    length: f32,
    value: f32,
    pub repetition: GMRepetition,
}

impl GMInterpolateVec2D {
    pub fn new<T: Into<GMVec2D>>(start: T, end: T, speed: f32) -> Self {
        let start = start.into();
        let end = end.into();
        let direction = end - start;
        let length = direction.len();

        let result = Self {
            start,
            end,
            vector: start.clone(),
            speed,
            direction: direction.norm2(),
            length,
            value: 0.0,
            repetition: GMRepetition::OnceForward,
        };

        result.check_length();
        result.check_speed();
        result
    }

    pub fn update(&mut self) {
        match self.repetition {
            GMRepetition::OnceForward => {
                self.value += self.speed;
                if self.value > self.length {
                    self.value = self.length;
                }

                self.vector = self.start + (self.direction * self.value);
            },
            GMRepetition::OnceBackward => {
                self.value -= self.speed;
                if self.value < 0.0 {
                    self.value = 0.0;
                }

                self.vector = self.start + (self.direction * self.value);
            },
            GMRepetition::LoopForward => {
                self.value += self.speed;
                if self.value > self.length {
                    self.value = 0.0;
                }

                self.vector = self.start + (self.direction * self.value);
            },
            GMRepetition::LoopBackward => {
                self.value -= self.speed;
                if self.value < 0.0 {
                    self.value = self.length;
                }

                self.vector = self.start + (self.direction * self.value);
            },
            GMRepetition::PingPongForward => {
                self.value += self.speed;
                if self.value > self.length {
                    self.value = self.length;
                    self.repetition = GMRepetition::PingPongBackward;
                }

                self.vector = self.start + (self.direction * self.value);
            },
            GMRepetition::PingPongBackward => {
                self.value -= self.speed;
                if self.value < 0.0 {
                    self.value = 0.0;
                    self.repetition = GMRepetition::PingPongForward;
                }

                self.vector = self.start + (self.direction * self.value);
            },
        }
    }

    fn check_length(&self) {
        assert!(self.length > 0.0, "GMInterpolateVec2D::check_length(), length must be greater than 0.0, start: '{:?}', end: '{:?}'", self.start, self.end);
    }

    fn check_speed(&self) {
        assert!(self.speed > 0.0 && self.speed < self.length, "GMInterpolateVec2D::check_speed(), speed must be between 0.0 and {}", self.length);
    }

    pub fn reset(&mut self) {
        match self.repetition {
            GMRepetition::OnceForward | GMRepetition::LoopForward | GMRepetition::PingPongForward => {
                self.value = 0.0;
            }
            _ => {
                self.value = self.length;
            }
        }
    }

    pub fn set_start(&mut self, start: GMVec2D) {
        self.start = start;

        let direction = self.end - self.start;
        self.length = direction.len();
        self.direction = direction.norm2();

        self.check_length();
        self.check_speed();
    }

    pub fn get_start(&self) -> GMVec2D {
        self.start
    }

    pub fn set_end(&mut self, end: GMVec2D) {
        self.end = end;

        let direction = self.end - self.start;
        self.length = direction.len();
        self.direction = direction.norm2();

        self.check_length();
        self.check_speed();
    }

    pub fn get_end(&self) -> GMVec2D {
        self.end
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;

        self.check_speed();
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn set_start_end_speed<T: Into<GMVec2D>>(&mut self, start: T, end: T, speed: f32) {
        self.start = start.into();
        self.end = end.into();
        self.speed = speed;

        let direction = self.end - self.start;
        self.length = direction.len();
        self.direction = direction.norm2();

        self.check_length();
        self.check_speed();
    }

    pub fn set_value(&mut self, value: f32) {
        assert!(value >= 0.0 && value <= self.length, "GMInterpolateVec2D::set_value(), value must be between 0.0 and {}", self.length);

        self.value = value;
    }

    pub fn set_value_norm(&mut self, value: f32) {
        assert!(value >= 0.0 && value <= 1.0, "GMInterpolateVec2D::set_value_norm(), value must be between 0.0 and 1.0");

        self.value = self.length * value;
    }

    pub fn get_vector(&self) -> GMVec2D {
        self.vector.clone()
    }

    pub fn is_finished(&self) -> bool {
        match self.repetition {
            GMRepetition::OnceForward => {
                self.value == self.length
            }
            GMRepetition::OnceBackward => {
                self.value == 0.0
            }
            _ => {
                false
            }
        }
    }
}

// ECS components:

#[derive(Clone, Debug)]
pub struct GMInterpolateRotation(pub GMInterpolateF32);

#[derive(Clone, Debug)]
pub struct GMInterpolatePosition(pub GMInterpolateVec2D);

#[derive(Clone, Debug)]
pub struct GMInterpolateCircle {
    pub interpolate: GMInterpolateF32,
    pub circle: GMCircle,
}

// ECS systems:

pub fn interpolate_rotation(world: &mut World) {
    for (_e, (angle, interpolate, active)) in
        world.query_mut::<(&mut GMAngle, &mut GMInterpolateRotation, &GMActive)>() {
        if active.0 {
            let interpolate = &mut interpolate.0;
            interpolate.update();
            angle.0 = interpolate.get_value();
        }
    }
}

pub fn interpolate_position(world: &mut World) {
    for (_e, (position, interpolate, active)) in
        world.query_mut::<(&mut GMPosition, &mut GMInterpolatePosition, &GMActive)>() {
        if active.0 {
            let interpolate = &mut interpolate.0;
            interpolate.update();
            position.0 = interpolate.get_vector();
        }
    }
}

pub fn interpolate_circle(world: &mut World) {
    for (_e, (position, interpolate, active)) in
        world.query_mut::<(&mut GMPosition, &mut GMInterpolateCircle, &GMActive)>() {
        if active.0 {
            let circle = interpolate.circle;

            let interpolate = &mut interpolate.interpolate;
            interpolate.update();

            position.0 = circle.position_from_deg(interpolate.get_value());
        }
    }
}
