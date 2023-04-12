

use crate::math::{GMVec2D, GMCircle};
use crate::interpolation::{GMInterpolateVec2D, GMInterpolateF32, GMCuLinear, GMCurveT};
use crate::util::{GMRepetition, GMUpdateT};
use crate::timer::GMTimer;


pub trait GMPositionT {
    fn set_position_x(&mut self, x: f32);

    fn set_position_y(&mut self, y: f32);

    fn get_position_x(&self) -> f32;

    fn get_position_y(&self) -> f32;

    fn set_position_xy(&mut self, x: f32, y: f32) {
        self.set_position_x(x);
        self.set_position_y(y);
    }

    fn set_position_vec2d(&mut self, position: GMVec2D) {
        self.set_position_x(position.x);
        self.set_position_y(position.y);
    }

    fn set_position_vec2d_b(&mut self, position: &GMVec2D) {
        self.set_position_x(position.x);
        self.set_position_y(position.y);
    }

    fn set_position_tuple(&mut self, position: (f32, f32)) {
        self.set_position_x(position.0);
        self.set_position_y(position.1);
    }

    fn set_position_slice(&mut self, position: &[f32]) {
        self.set_position_x(position[0]);
        self.set_position_y(position[1]);
    }

    fn set_position_array(&mut self, position: [f32; 2]) {
        self.set_position_x(position[0]);
        self.set_position_y(position[1]);
    }

    fn get_position_vec2d(&self) -> GMVec2D {
        GMVec2D::new(self.get_position_x(), self.get_position_y())
    }

    fn get_position_tuple(&self) -> (f32, f32) {
        (self.get_position_x(), self.get_position_y())
    }

    fn add_position_x(&mut self, x: f32) {
        self.set_position_x(self.get_position_x() + x);
    }

    fn add_position_y(&mut self, y: f32) {
        self.set_position_y(self.get_position_y() + y);
    }

    fn add_position_xy(&mut self, x: f32, y: f32) {
        self.add_position_x(x);
        self.add_position_y(y);
    }

    fn add_position_vec2d(&mut self, position: GMVec2D) {
        self.add_position_x(position.x);
        self.add_position_y(position.y);
    }

    fn add_position_vec2d_b(&mut self, position: &GMVec2D) {
        self.add_position_x(position.x);
        self.add_position_y(position.y);
    }

    fn add_position_tuple(&mut self, position: (f32, f32)) {
        self.add_position_x(position.0);
        self.add_position_y(position.1);
    }

    fn add_position_slice(&mut self, position: &[f32]) {
        self.add_position_x(position[0]);
        self.add_position_y(position[1]);
    }

    fn add_position_array(&mut self, position: [f32; 2]) {
        self.add_position_x(position[0]);
        self.add_position_y(position[1]);
    }
}

// If multiple positions are available:

pub trait GMPositionMultipleT {
    fn set_position_x_n(&mut self, x: f32, index: usize);

    fn set_position_y_n(&mut self, y: f32, index: usize);

    fn get_position_x_n(&self, index: usize) -> f32;

    fn get_position_y_n(&self, index: usize) -> f32;

    fn set_position_xy_n(&mut self, x: f32, y: f32, index: usize) {
        self.set_position_x_n(x, index);
        self.set_position_y_n(y, index);
    }

    fn set_position_vec2d_n(&mut self, position: GMVec2D, index: usize) {
        self.set_position_x_n(position.x, index);
        self.set_position_y_n(position.y, index);
    }

    fn set_position_vec2d_b_n(&mut self, position: &GMVec2D, index: usize) {
        self.set_position_x_n(position.x, index);
        self.set_position_y_n(position.y, index);
    }

    fn set_position_tuple_n(&mut self, position: (f32, f32), index: usize) {
        self.set_position_x_n(position.0, index);
        self.set_position_y_n(position.1, index);
    }

    fn set_position_slice_n(&mut self, position: &[f32], index: usize) {
        self.set_position_x_n(position[0], index);
        self.set_position_y_n(position[1], index);
    }

    fn set_position_array_n(&mut self, position: [f32; 2], index: usize) {
        self.set_position_x_n(position[0], index);
        self.set_position_y_n(position[1], index);
    }

    fn get_position_vec2d_n(&self, index: usize) -> GMVec2D {
        GMVec2D::new(self.get_position_x_n(index), self.get_position_y_n(index))
    }

    fn get_position_tuple_n(&self, index: usize) -> (f32, f32) {
        (self.get_position_x_n(index), self.get_position_y_n(index))
    }

    fn add_position_x_n(&mut self, x: f32, index: usize) {
        self.set_position_x_n(self.get_position_x_n(index) + x, index);
    }

    fn add_position_y_n(&mut self, y: f32, index: usize) {
        self.set_position_y_n(self.get_position_y_n(index) + y, index);
    }

    fn add_position_xy_n(&mut self, x: f32, y: f32, index: usize) {
        self.add_position_x_n(x, index);
        self.add_position_y_n(y, index);
    }

    fn add_position_vec2d_n(&mut self, position: GMVec2D, index: usize) {
        self.add_position_x_n(position.x, index);
        self.add_position_y_n(position.y, index);
    }

    fn add_position_vec2d_b_n(&mut self, position: &GMVec2D, index: usize) {
        self.add_position_x_n(position.x, index);
        self.add_position_y_n(position.y, index);
    }

    fn add_position_tuple_n(&mut self, position: (f32, f32), index: usize) {
        self.add_position_x_n(position.0, index);
        self.add_position_y_n(position.1, index);
    }

    fn add_position_slice_n(&mut self, position: &[f32], index: usize) {
        self.add_position_x_n(position[0], index);
        self.add_position_y_n(position[1], index);
    }

    fn add_position_array_n(&mut self, position: [f32; 2], index: usize) {
        self.add_position_x_n(position[0], index);
        self.add_position_y_n(position[1], index);
    }
}

// TODO: Impl GMPositionMultipleT for GMPositionT ?


#[macro_export]
macro_rules! gen_impl_position {
    ($type:ty) => {
        impl GMPositionT for $type {
            fn set_position_x(&mut self, x: f32) {
                self.position.x = x;
            }

            fn set_position_y(&mut self, y: f32) {
                self.position.y = y;
            }

            fn get_position_x(&self) -> f32 {
                self.position.x
            }

            fn get_position_y(&self) -> f32 {
                self.position.y
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

// TODO: Add trait for multiple angles


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

// TODO: Add trait for multiple scales


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
pub struct GMMVVelocity {
    velocity: GMVec2D,
}

impl GMMVVelocity {
    pub fn new<T: Into<GMVec2D>>(velocity: T) -> Self {
        Self {
            velocity: velocity.into(),
        }
    }

    pub fn set_velocity<T: Into<GMVec2D>>(&mut self, velocity: T) {
        self.velocity = velocity.into();
    }

    pub fn get_velocity(&self) -> GMVec2D {
        self.velocity
    }

    pub fn get_velocity_mut(&mut self) -> &mut GMVec2D {
        &mut self.velocity
    }

    pub fn add_velocity<T: Into<GMVec2D>>(&mut self, velocity: T) {
        let velocity = velocity.into();
        self.velocity.add2(velocity)
    }

    pub fn add_velocity2<T: Into<GMVec2D>>(&self, velocity: T) -> GMMVVelocity {
        let velocity = velocity.into();
        let result = self.velocity + velocity;
        GMMVVelocity::new(result)
    }

    pub fn set_position_of<T: GMPositionT>(&self, position: &mut T) {
        position.add_position_vec2d(self.velocity);
    }

    pub fn set_position_n_of<T: GMPositionMultipleT>(&self, position: &mut T, index: usize) {
        position.add_position_vec2d_n(self.velocity, index);
    }
}

#[derive(Debug, Clone)]
pub struct GMMVAcceleration {
    acceleration: GMVec2D,
}

impl GMMVAcceleration {
    pub fn new<T: Into<GMVec2D>>(acceleration: T) -> Self {
        Self {
            acceleration: acceleration.into(),
        }
    }

    pub fn set_acceleration<T: Into<GMVec2D>>(&mut self, acceleration: T) {
        self.acceleration = acceleration.into();
    }

    pub fn get_acceleration(&self) -> GMVec2D {
        self.acceleration
    }

    pub fn get_acceleration_mut(&mut self) -> &mut GMVec2D {
        &mut self.acceleration
    }

    pub fn set_velocity_of(&self, velocity: &mut GMMVVelocity) {
        velocity.add_velocity(self.acceleration);
    }

    pub fn calc_velocity(&self, velocity: &GMMVVelocity) -> GMMVVelocity {
        velocity.add_velocity2(self.acceleration)
    }

    pub fn set_position_and_velocity_of<T: GMPositionT>(&self, position: &mut T, velocity: &mut GMMVVelocity) {
        velocity.add_velocity(self.acceleration);
        velocity.set_position_of(position);
    }

    pub fn set_position_and_velocity_n_of<T: GMPositionMultipleT>(&self, position: &mut T, velocity: &mut GMMVVelocity, index: usize) {
        velocity.add_velocity(self.acceleration);
        velocity.set_position_n_of(position, index);
    }
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
        let new_pos = self.calc_position();
        movable.set_position_vec2d(new_pos);
    }

    pub fn set_position_n_of<T: GMPositionMultipleT>(&self, movable: &mut T, index: usize) {
        let new_pos = self.calc_position();
        movable.set_position_vec2d_n(new_pos, index);
    }

    pub fn calc_position(&self) -> GMVec2D {
        self.interpolation.get_current_value()
    }

    pub fn set_and_update<T: GMPositionT>(&mut self, movable: &mut T) {
        self.set_position_of(movable);
        self.update();
    }

    pub fn set_and_update_n<T: GMPositionMultipleT>(&mut self, movable: &mut T, index: usize) {
        self.set_position_n_of(movable, index);
        self.update();
    }

    pub fn update_start_end(&mut self) {
        self.interpolation.calculate_diff();
    }

    gen_get_interpolation_methods!(GMInterpolateVec2D);
}

impl GMUpdateT for GMMV2Points {
    fn update(&mut self) {
        self.interpolation.update();
    }
}

impl GMPositionMultipleT for GMMV2Points {
    fn set_position_x_n(&mut self, x: f32, index: usize) {
        if index == 0 {
            self.interpolation.get_start_mut().x = x;
        } else {
            self.interpolation.get_end_mut().x = x;
        }
    }

    fn set_position_y_n(&mut self, y: f32, index: usize) {
        if index == 0 {
            self.interpolation.get_start_mut().y = y;
        } else {
            self.interpolation.get_end_mut().y = y;
        }
    }

    fn get_position_x_n(&self, index: usize) -> f32 {
        if index == 0 {
            self.interpolation.get_start().x
        } else {
            self.interpolation.get_end().x
        }
    }

    fn get_position_y_n(&self, index: usize) -> f32 {
        if index == 0 {
            self.interpolation.get_start().y
        } else {
            self.interpolation.get_end().y
        }
    }
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
        let new_angle = self.calc_angle();
        rotatable.set_angle(new_angle);
    }

    pub fn calc_angle(&self) -> f32 {
        self.interpolation.get_current_value()
    }

    pub fn set_and_update<T: GMRotationT>(&mut self, rotatable: &mut T) {
        self.set_angle_of(rotatable);
        self.update();
    }

    gen_get_interpolation_methods!(GMInterpolateF32);
}

impl GMUpdateT for GMMVRotate {
    fn update(&mut self) {
        self.interpolation.update();
    }
}

#[derive(Debug, Clone)]
pub struct GMMVScale {
    interpolation: GMInterpolateF32,
}

impl GMMVScale {
    pub fn new(start: f32, end: f32, speed: f32) -> Self {
        Self {
            interpolation: GMInterpolateF32::new(start, end, speed, 0.0),
        }
    }

    pub fn set_scale_of<T: GMScaleT>(&self, scalable: &mut T) {
        let new_scale = self.calc_scale();
        scalable.set_scale(new_scale);
    }

    pub fn calc_scale(&self) -> f32 {
        self.interpolation.get_current_value()
    }

    pub fn set_and_update<T: GMScaleT>(&mut self, scalable: &mut T) {
        self.set_scale_of(scalable);
        self.update();
    }

    gen_get_interpolation_methods!(GMInterpolateF32);
}

impl GMUpdateT for GMMVScale {
    fn update(&mut self) {
        self.interpolation.update();
    }
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
        let new_position = self.calc_position();
        movable.set_position_vec2d(new_position);
    }

    pub fn set_position_n_of<T: GMPositionMultipleT>(&self, movable: &mut T, index: usize) {
        let new_position = self.calc_position();
        movable.set_position_vec2d_n(new_position, index);
    }

    pub fn calc_position(&self) -> GMVec2D {
        let new_angle = self.interpolation.get_current_value();
        self.circle.position_from_deg(new_angle)

    }

    pub fn set_and_update<T: GMPositionT>(&mut self, movable: &mut T) {
        self.set_position_of(movable);
        self.update();
    }

    pub fn set_and_update_n<T: GMPositionMultipleT>(&mut self, movable: &mut T, index: usize) {
        self.set_position_n_of(movable, index);
        self.update();
    }

    gen_get_interpolation_methods!(GMInterpolateF32);
}

impl GMUpdateT for GMMVCircle {
    fn update(&mut self) {
        self.interpolation.update();
    }
}

impl GMPositionT for GMMVCircle {
    fn set_position_x(&mut self, x: f32) {
        self.circle.center.x = x;
    }

    fn set_position_y(&mut self, y: f32) {
        self.circle.center.y = y;
    }

    fn get_position_x(&self) -> f32 {
        self.circle.center.x
    }

    fn get_position_y(&self) -> f32 {
        self.circle.center.y
    }
}

impl GMScaleT for GMMVCircle {
    fn get_scale(&self) -> f32 {
        self.circle.radius
    }

    fn get_scale_mut(&mut self) -> &mut f32 {
        &mut self.circle.radius
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
        let new_position = self.calc_position(index);
        movable.set_position_vec2d(new_position);
    }

    pub fn calc_position(&self, index: u32) -> GMVec2D {
        let f_index = index as f32;
        let new_angle = self.interpolation.get_current_value();
        self.circle.position_from_deg(new_angle + (f_index * self.angle_step))
    }

    gen_get_interpolation_methods!(GMInterpolateF32);
}

impl GMUpdateT for GMMVCircleMultiple {
    fn update(&mut self) {
        self.interpolation.update();
    }
}

impl GMPositionT for GMMVCircleMultiple {
    fn set_position_x(&mut self, x: f32) {
        self.circle.center.x = x;
    }

    fn set_position_y(&mut self, y: f32) {
        self.circle.center.y = y;
    }

    fn get_position_x(&self) -> f32 {
        self.circle.center.x
    }

    fn get_position_y(&self) -> f32 {
        self.circle.center.y
    }
}

impl GMScaleT for GMMVCircleMultiple {
    fn get_scale(&self) -> f32 {
        self.circle.radius
    }

    fn get_scale_mut(&mut self) -> &mut f32 {
        &mut self.circle.radius
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

        let speeds = vec![0.01; num_of_elems];
        let curves: Vec<Box<dyn GMCurveT>> = vec![Box::new(GMCuLinear{}); num_of_elems];
        let start = positions[0];
        let end = positions[1];
        let current_interpolation = GMInterpolateVec2D::new(start, end, speeds[0], 0.0);

        Self {
            positions,
            speeds,
            curves,
            current_index: 0,
            current_interpolation,
            repetition: GMRepetition::OnceForward,
        }
    }

    pub fn new2(positions: &[(f32, f32)]) -> Self {
        let positions: Vec<GMVec2D> = positions.iter().map(|(x, y)| GMVec2D::new(*x, *y)).collect();
        GMMVPolygon::new(&positions)
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
        let new_position = self.calc_position();
        movable.set_position_vec2d(new_position);
    }

    pub fn set_position_n_of<T: GMPositionMultipleT>(&self, movable: &mut T, index: usize) {
        let new_position = self.calc_position();
        movable.set_position_vec2d_n(new_position, index);
    }

    pub fn calc_position(&self) -> GMVec2D {
        self.current_interpolation.get_current_value()
    }

    pub fn reset_with_end_index(&mut self, end_index: usize) {
        let start = self.positions[self.current_index];
        let end = self.positions[end_index];
        self.current_interpolation.set_start(start);
        self.current_interpolation.set_end(end);
        self.current_interpolation.calculate_diff();
        self.current_interpolation.set_current_step(0.0);
        self.current_interpolation.set_speed(self.speeds[self.current_index]);
        self.current_interpolation.set_curve2(self.curves[self.current_index].clone());
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
        self.reset_with_end_index(1);
    }

    pub fn set_and_update<T: GMPositionT>(&mut self, movable: &mut T) {
        self.set_position_of(movable);
        self.update();
    }

    pub fn set_and_update_n<T: GMPositionMultipleT>(&mut self, movable: &mut T, index: usize) {
        self.set_position_n_of(movable, index);
        self.update();
    }
}

impl GMUpdateT for GMMVPolygon {
    fn update(&mut self) {
        self.current_interpolation.update();

        if self.current_interpolation.is_finished() {
            let num_of_elements = self.positions.len();

            match self.repetition {
                GMRepetition::OnceForward => {
                    if self.current_index < num_of_elements - 2 {
                        self.current_index += 1;
                        self.reset_with_end_index(self.current_index + 1);
                    }
                }
                GMRepetition::OnceBackward => {
                    if self.current_index > 1 {
                        self.current_index -= 1;
                        self.reset_with_end_index(self.current_index - 1);
                    }
                }
                GMRepetition::LoopForward => {
                    if self.current_index < num_of_elements - 2 {
                        self.current_index += 1;
                        self.reset_with_end_index(self.current_index + 1);
                    } else {
                        self.current_index = 0;
                        self.reset_with_end_index(1);
                    }
                }
                GMRepetition::LoopBackward => {
                    if self.current_index > 1 {
                        self.current_index -= 1;
                        self.reset_with_end_index(self.current_index - 1);
                    } else {
                        self.current_index = num_of_elements - 1;
                        self.reset_with_end_index(self.current_index - 2);
                    }
                }
                GMRepetition::PingPongForward => {
                    if self.current_index < num_of_elements - 2 {
                        self.current_index += 1;
                        self.reset_with_end_index(self.current_index + 1);
                    } else {
                        self.reset_with_end_index(self.current_index - 1);
                        self.repetition = GMRepetition::PingPongBackward;
                    }
                }
                GMRepetition::PingPongBackward => {
                    if self.current_index > 1 {
                        self.current_index -= 1;
                        self.reset_with_end_index(self.current_index - 1);
                    } else {
                        self.reset_with_end_index(1);
                        self.repetition = GMRepetition::PingPongForward;
                    }
                }
            };
        }
    }
}

// TODO: Impl GMPositionMultipleT for GMMVPolygon

#[derive(Debug, Clone)]
pub struct GMMVFollow {
    speed: f32,
    timer: GMTimer,
    current_position: GMVec2D,
    current_direction: GMVec2D,
}

impl GMMVFollow {
    pub fn new<T: Into<GMVec2D>>(speed: f32, delay: f32, position: T) -> Self {
        Self {
            speed,
            timer: GMTimer::new(delay),
            current_position: position.into(),
            current_direction: GMVec2D::new(0.0, 0.0),
        }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn set_delay(&mut self, delay: f32) {
        self.timer.set_duration(delay);
    }

    pub fn get_delay(&self) -> f32 {
        self.timer.get_duration()
    }

    pub fn set_position_of<T: GMPositionT>(&self, movable: &mut T) {
        movable.set_position_vec2d(self.current_position.clone());
    }

    pub fn calc_position(&self) -> GMVec2D {
        self.current_position.clone()
    }

    pub fn set_target(&mut self, target: &GMVec2D) {
        if self.timer.finished() {
            self.current_direction = (*target) - self.current_position;
            self.current_direction.norm();
            self.timer.start();
        }
    }

    pub fn set_target2<T: Into<GMVec2D>>(&mut self, target: T) {
        let target = target.into();
        self.set_target(&target);
    }
}

impl GMUpdateT for GMMVFollow {
    fn update(&mut self) {
        self.current_position = self.current_position + (self.current_direction * self.speed);
    }
}

// TODO: Border: Wrap around, bounce off, stop, ...
// TODO: Force, ...
