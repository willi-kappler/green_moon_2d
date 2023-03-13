
use crate::math::{GMVec2D, GMCircle};
use crate::interpolation::{GMInterpolateVec2D, GMInterpolateF32};

pub trait GMPositionT {
    fn set_position<T: Into<GMVec2D>>(&mut self, position: T) {
        *self.get_position_mut() = position.into();
    }

    fn set_position_x(&mut self, x: f32) {
        self.get_position_mut().x = x;
    }

    fn set_position_y(&mut self, y: f32) {
        self.get_position_mut().y = y;
    }

    fn add_position<T: Into<GMVec2D>>(&mut self, position: T) {
        self.get_position_mut().add2(position);
    }

    fn add_position_x(&mut self, x: f32) {
        self.get_position_mut().x += x;
    }

    fn add_position_y(&mut self, y: f32) {
        self.get_position_mut().y += y;
    }

    fn get_position(&self) -> GMVec2D;

    fn get_position_mut(&mut self) -> &mut GMVec2D;
}

#[macro_export]
macro_rules! gen_impl_position {
    ($type:ty) => {
        impl GMPositionT for $type {
            fn get_position(&self) -> GMVec2D {
                self.position
            }

            fn get_position_mut(&mut self) -> &mut GMVec2D {
                &mut self.position
            }
        }
    };
}

pub trait GMRotationT {
    fn set_angle(&mut self, rotation: f32) {
        *self.get_angle_mut() = rotation;
    }

    fn add_angle(&mut self, rotation: f32) {
        *self.get_angle_mut() += rotation;
    }

    fn get_angle(&self) -> f32;

    fn get_angle_mut(&mut self) -> &mut f32;
}

#[macro_export]
macro_rules! gen_impl_rotation {
    ($type:ty) => {
        impl GMRotationT for $type {
            fn get_angle(&self) -> f32 {
                self.angle
            }

            fn get_angle_mut(&mut self) -> &mut f32 {
                &mut self.angle
            }
        }
    };
}

pub trait GMScaleT {
    fn set_scale(&mut self, scale: f32) {
        *self.get_scale_mut() = scale;
    }

    fn add_scale(&mut self, scale: f32) {
        *self.get_scale_mut() += scale;
    }

    fn get_scale(&self) -> f32;

    fn get_scale_mut(&mut self) -> &mut f32;
}

#[macro_export]
macro_rules! gen_impl_scale {
    ($type:ty) => {
        impl GMScaleT for $type {
            fn get_scale(&self) -> f32 {
                self.scale
            }

            fn get_scale_mut(&mut self) -> &mut f32 {
                &mut self.scale
            }
        }
    };
}

#[macro_export]
macro_rules! gen_get_interpolation_methods {
    ($type:ty) => {
        pub fn get_interpolation(&self) -> &$type {
            &self.interpolation
        }

        pub fn get_interpolation_mut(&mut self) -> &mut $type {
            &mut self.interpolation
        }
    };
}

#[derive(Debug, Clone)]
pub struct GMMV2Points {
    interpolation: GMInterpolateVec2D,
}

impl GMMV2Points {
    pub fn new<S: Into<GMVec2D>, E: Into<GMVec2D>>(start: S, end: E, speed: f32) -> Self {
        Self {
            interpolation: GMInterpolateVec2D::new(start.into(), end.into(), speed, 0.0),
        }
    }

    pub fn update<T: GMPositionT>(&mut self, movable: &mut T) {
        let new_pos = self.interpolation.get_current_value();
        movable.set_position(new_pos);
        self.interpolation.update();
    }

    gen_get_interpolation_methods!(GMInterpolateVec2D);
}

#[derive(Debug, Clone)]
pub struct GMMVRotate {
    interpolation: GMInterpolateF32,
}

impl GMMVRotate {
    pub fn new(start: f32, end: f32, speed: f32) -> Self {
        Self {
            interpolation: GMInterpolateF32::new(start, end, speed, 0.0),
        }
    }

    pub fn update<T: GMRotationT>(&mut self, rotatable: &mut T) {
        let new_angle = self.interpolation.get_current_value();
        rotatable.set_angle(new_angle);
        self.interpolation.update();
    }

    gen_get_interpolation_methods!(GMInterpolateF32);
}

#[derive(Debug, Clone)]
pub struct GMMVCircle {
    interpolation: GMInterpolateF32,
    circle: GMCircle,
}

impl GMMVCircle {
    pub fn new<T: Into<GMVec2D>>(start: f32, end: f32, speed: f32, center: T, radius: f32) -> Self {
        Self {
            interpolation: GMInterpolateF32::new(start, end, speed, 0.0),
            circle: GMCircle::new(center, radius),
        }
    }

    pub fn update<T: GMPositionT>(&mut self, movable: &mut T) {
        let new_angle = self.interpolation.get_current_value();
        let new_position = self.circle.position_from_deg(new_angle);
        movable.set_position(new_position);
        self.interpolation.update();
    }

    gen_get_interpolation_methods!(GMInterpolateF32);
}
