
use crate::math::GMVec2D;

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

    fn add_position(&mut self, position: GMVec2D) {
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
    fn set_rotation(&mut self, rotation: f32) {
        *self.get_rotation_mut() = rotation;
    }

    fn add_rotation(&mut self, rotation: f32) {
        *self.get_rotation_mut() += rotation;
    }

    fn get_rotation(&self) -> f32;

    fn get_rotation_mut(&mut self) -> &mut f32;
}

#[macro_export]
macro_rules! gen_impl_rotation {
    ($type:ty) => {
        impl GMRotationT for $type {
            fn get_rotation(&self) -> f32 {
                self.rotation
            }

            fn get_rotation_mut(&mut self) -> &mut f32 {
                &mut self.rotation
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
