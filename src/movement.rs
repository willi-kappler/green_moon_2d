

use crate::math::{GMVec2D, GMCircle};
use crate::interpolation::{GMInterpolateVec2D, GMInterpolateF32, GMCuLinear, GMCurveT};
use crate::util::GMRepetition;

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

    pub fn set_position_of<T: GMPositionT>(&self, movable: &mut T) {
        let new_pos = self.get_position();
        movable.set_position(new_pos);
    }

    pub fn get_position(&self) -> GMVec2D {
        self.interpolation.get_current_value()
    }

    pub fn update(&mut self) {
        self.interpolation.update();
    }

    pub fn set_and_update<T: GMPositionT>(&mut self, movable: &mut T) {
        self.set_position_of(movable);
        self.update();
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

    pub fn set_angle_of<T: GMRotationT>(&self, rotatable: &mut T) {
        let new_angle = self.get_angle();
        rotatable.set_angle(new_angle);
    }

    pub fn get_angle(&self) -> f32 {
        self.interpolation.get_current_value()
    }

    pub fn update(&mut self) {
        self.interpolation.update();
    }

    pub fn set_and_update<T: GMRotationT>(&mut self, rotatable: &mut T) {
        self.set_angle_of(rotatable);
        self.update();
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

    pub fn set_radius(&mut self, radius: f32) {
        self.circle.radius = radius;
    }

    pub fn get_radius(&self) -> f32 {
        self.circle.radius
    }

    pub fn set_position_of<T: GMPositionT>(&self, movable: &mut T) {
        let new_position = self.get_position();
        movable.set_position(new_position);
    }

    pub fn get_position(&self) -> GMVec2D {
        let new_angle = self.interpolation.get_current_value();
        self.circle.position_from_deg(new_angle)

    }

    pub fn update(&mut self) {
        self.interpolation.update();
    }

    pub fn set_and_update<T: GMPositionT>(&mut self, movable: &mut T) {
        self.set_position_of(movable);
        self.update();
    }

    gen_get_interpolation_methods!(GMInterpolateF32);
}

impl GMPositionT for GMMVCircle {
    fn get_position(&self) -> GMVec2D {
        self.circle.center
    }

    fn get_position_mut(&mut self) -> &mut GMVec2D {
        &mut self.circle.center
    }
}

#[derive(Debug, Clone)]
pub struct GMMVCircleMultiple {
    interpolation: GMInterpolateF32,
    angle_step: f32,
    circle: GMCircle,
}

impl GMMVCircleMultiple {
    pub fn new<T: Into<GMVec2D>>(start: f32, end: f32, angle_step: f32, speed: f32, center: T, radius: f32) -> Self {
        Self {
            interpolation: GMInterpolateF32::new(start, end, speed, 0.0),
            angle_step,
            circle: GMCircle::new(center, radius),
        }
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.circle.radius = radius;
    }

    pub fn get_radius(&self) -> f32 {
        self.circle.radius
    }

    pub fn set_position_of<T: GMPositionT>(&self, movable: &mut T, index: u32) {
        let new_position = self.get_position(index);
        movable.set_position(new_position);
    }

    pub fn get_position(&self, index: u32) -> GMVec2D {
        let f_index = index as f32;
        let new_angle = self.interpolation.get_current_value();
        self.circle.position_from_deg(new_angle + (f_index * self.angle_step))
    }

    pub fn update(&mut self) {
        self.interpolation.update();
    }

    gen_get_interpolation_methods!(GMInterpolateF32);
}

impl GMPositionT for GMMVCircleMultiple {
    fn get_position(&self) -> GMVec2D {
        self.circle.center
    }

    fn get_position_mut(&mut self) -> &mut GMVec2D {
        &mut self.circle.center
    }
}


#[derive(Debug, Clone)]
pub struct GMMVPolygon {
    positions: Vec<GMVec2D>,
    speeds: Vec<f32>,
    curves: Vec<Box<dyn GMCurveT>>,
    current_index: usize,
    current_interpolation: GMInterpolateVec2D,
    repetition: GMRepetition,
}

impl GMMVPolygon {
    pub fn new(positions: &[GMVec2D]) -> Self {
        let positions = positions.to_vec();
        let num_of_elems = positions.len();
        assert!(num_of_elems > 2, "GMMVPolygon: must have at least three points (coordinates)");

        let speeds = vec![0.1; num_of_elems];
        let curves: Vec<Box<dyn GMCurveT>> = vec![Box::new(GMCuLinear{}); num_of_elems];
        let start = positions[0];
        let end = positions[1];
        let speed = 0.1;
        let current_interpolation = GMInterpolateVec2D::new(start, end, speed, 0.0);

        Self {
            positions,
            speeds,
            curves,
            current_index: 0,
            current_interpolation,
            repetition: GMRepetition::OnceForward,
        }
    }

    pub fn set_speed_for_all(&mut self, speed: f32) {
        for s in self.speeds.iter_mut() {
            *s = speed;
        }
    }

    pub fn set_curve_for_all<T: GMCurveT>(&mut self, curve: T) {
        for c in self.curves.iter_mut() {
            *c = curve.clone_box();
        }
    }

    pub fn set_curve_for_all2(&mut self, curve: Box<dyn GMCurveT>) {
        for c in self.curves.iter_mut() {
            *c = curve.clone();
        }
    }

    pub fn set_speeds(&mut self, speeds: Vec<f32>) {
        self.speeds = speeds;
    }

    pub fn set_curves<T: GMCurveT>(&mut self, curves: &[T]) {
        self.curves.clear();

        for c in curves.iter() {
            self.curves.push(c.clone_box());
        }
    }

    pub fn set_curves2(&mut self, curves: &[Box<dyn GMCurveT>]) {
        self.curves = curves.to_vec();
    }

    pub fn set_positions(&mut self, positions: &[GMVec2D]) {
        self.positions = positions.to_vec();
    }

    pub fn set_speed_at(&mut self, speed: f32, index: usize) {
        self.speeds[index] = speed;
    }

    pub fn set_curve_at<T: GMCurveT>(&mut self, curve: T, index: usize) {
        self.curves[index] = curve.clone_box();
    }

    pub fn set_curve_at2(&mut self, curve: Box<dyn GMCurveT>, index: usize) {
        self.curves[index] = curve.clone();
    }

    pub fn set_position_at<T: Into<GMVec2D>>(&mut self, position: T, index: usize) {
        self.positions[index] = position.into();
    }

    pub fn set_index(&mut self, index: usize) {
        self.current_index = index;
    }

    pub fn set_repetition(&mut self, repetition: GMRepetition) {
        self.repetition = repetition;
    }

    pub fn set_position_of<T: GMPositionT>(&self, movable: &mut T) {
        let new_position = self.get_position();
        movable.set_position(new_position);
    }

    pub fn get_position(&self) -> GMVec2D {
        self.current_interpolation.get_current_value()
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
        let start = self.positions[0];
        let end = self.positions[1];
        self.current_interpolation.set_start(start);
        self.current_interpolation.set_end(end);
        self.current_interpolation.set_current_step(0.0);
        self.current_interpolation.set_speed(self.speeds[0]);
        self.current_interpolation.set_curve2(self.curves[0].clone());
    }

    pub fn update(&mut self) {
        self.current_interpolation.update();

        if self.current_interpolation.is_finished() {
            self.current_index += 1;
            // TODO: add more code
        }
    }

    pub fn set_and_update<T: GMPositionT>(&mut self, movable: &mut T) {
        self.set_position_of(movable);
        self.update();
    }

}
