
use std::sync::Arc;
use std::fmt::Debug;

use log::debug;

use crate::texture::{GMTexture, GMTextureT};
use crate::math::{GMVec2D, GMSize};
use crate::util::{GMDrawT, GMUpdateT, GMVisibleT, GMActiveT, GMFlipXYT, GMSizeT};
use crate::context::GMContext;
use crate::movement::{GMPositionT, GMRotationT, GMScaleT};
use crate::animation::GMAnimation;
use crate::line::GMLineT;

use crate::{gen_impl_position, gen_impl_rotation, gen_impl_scale, gen_impl_flipxy,
    gen_impl_visible, gen_impl_active, gen_impl_texture, gen_impl_size};


#[derive(Debug, Clone)]
pub struct GMSprite {
    texture: Arc<GMTexture>,
    position: GMVec2D,
    animation: GMAnimation,
    angle: f32,
    scale: f32,
    flip_x: bool,
    flip_y: bool,
    size: GMSize,
    visible: bool,
    active: bool,
    // TODO: Add sprite children ?
}

impl GMSprite {
    pub fn new<T: Into<GMVec2D>>(texture: &Arc<GMTexture>, position: T, animation: GMAnimation) -> Self {
        let position = position.into();

        debug!("GMSprite::new(), position: '{}'", position);

        let (width, height) = texture.get_unit_dimension();

        Self {
            texture: texture.clone(),
            position: position.into(),
            animation,
            angle: 0.0,
            scale: 1.0,
            size: GMSize::new(width, height),
            flip_x: false,
            flip_y: false,
            visible: true,
            active: true,
        }
    }
}

impl GMDrawT for GMSprite {
    fn draw(&self, context: &mut GMContext) {
        if self.visible {
            let index = self.animation.texture_index();
            let dx = self.position.x;
            let dy = self.position.y;

            self.texture.draw_opt(dx, dy, index, self.angle, self.scale, self.flip_x, self.flip_y, context);
        }
    }
}

impl GMUpdateT for GMSprite {
    fn update(&mut self) {
        if self.active {
            self.animation.update();
        }
    }
}

gen_impl_position!(GMSprite);

gen_impl_rotation!(GMSprite);

gen_impl_scale!(GMSprite);

gen_impl_flipxy!(GMSprite);

gen_impl_visible!(GMSprite);

gen_impl_active!(GMSprite);

gen_impl_texture!(GMSprite);

gen_impl_size!(GMSprite);

impl GMLineT for GMSprite {
    fn clone_box(&self) -> Box<dyn GMLineT> {
        Box::new(self.clone())
    }
}
