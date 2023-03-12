
use std::sync::Arc;


use crate::texture::{GMTexture, GMTextureT};
use crate::math::{GMVec2D};
use crate::util::{error_panic, GMAlign, GMDrawT, GMUpdateT, GMVisibleT, GMActiveT, GMFlipXYT};
use crate::context::GMContext;
use crate::movement::{GMPositionT, GMRotationT, GMScaleT};

use crate::{gen_effect_trait, gen_effect_impl_for_type, gen_type_effect_methods,
    gen_impl_position, gen_impl_rotation, gen_impl_scale, gen_impl_flipxy,
    gen_impl_visible, gen_impl_active, gen_impl_texture};


#[derive(Debug, Clone)]
pub struct GMSpriteBase {
    texture: Arc<GMTexture>,
    position: GMVec2D,
    rotation: f32,
    scale: f32,
    flip_x: bool,
    flip_y: bool,
    visible: bool,
    active: bool,
}

impl GMSpriteBase {
    pub fn new<T: Into<GMVec2D>>(texture: Arc<GMTexture>, position: T) -> Self {
        Self {
            texture,
            position: position.into(),
            rotation: 0.0,
            scale: 1.0,
            flip_x: false,
            flip_y: false,
            visible: true,
            active: true,
        }
    }
}

gen_impl_position!(GMSpriteBase);

gen_impl_rotation!(GMSpriteBase);

gen_impl_scale!(GMSpriteBase);

gen_impl_flipxy!(GMSpriteBase);

gen_impl_visible!(GMSpriteBase);

gen_impl_active!(GMSpriteBase);

gen_impl_texture!(GMSpriteBase);
