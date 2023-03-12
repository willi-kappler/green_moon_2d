
use std::sync::Arc;
use std::fmt::Debug;
use log::debug;


use crate::texture::{GMTexture, GMTextureT};
use crate::math::{GMVec2D};
use crate::util::{GMDrawT, GMUpdateT, GMVisibleT, GMActiveT, GMFlipXYT};
use crate::context::GMContext;
use crate::movement::{GMPositionT, GMRotationT, GMScaleT};
use crate::animation::GMAnimation;

use crate::{gen_effect_trait, gen_effect_impl_for_type,
    gen_impl_position, gen_impl_rotation, gen_impl_scale, gen_impl_flipxy,
    gen_impl_visible, gen_impl_active, gen_impl_texture};


#[derive(Debug, Clone)]
pub struct GMSpriteBase {
    texture: Arc<GMTexture>,
    position: GMVec2D,
    animation: GMAnimation,
    rotation: f32,
    scale: f32,
    flip_x: bool,
    flip_y: bool,
    draw_first: bool,
    update_first: bool,
    visible: bool,
    active: bool,
}

impl GMSpriteBase {
    pub fn new<T: Into<GMVec2D>>(texture: Arc<GMTexture>, position: T, animation: GMAnimation) -> Self {
        Self {
            texture,
            position: position.into(),
            animation,
            rotation: 0.0,
            scale: 1.0,
            flip_x: false,
            flip_y: false,
            draw_first: true,
            update_first: true,
            visible: true,
            active: true,
        }
    }

    gen_draw_first_methods!();

    gen_update_first_methods!();
}

impl GMDrawT for GMSpriteBase {
    fn draw(&self, context: &mut GMContext) {
        if self.visible {
            let index = self.animation.texture_index();
            let dx = self.position.x;
            let dy = self.position.y;

            self.texture.draw_opt(dx, dy, index, self.rotation, self.scale, self.flip_x, self.flip_y, context);
        }
    }
}

impl GMUpdateT for GMSpriteBase {
    fn update(&mut self, _context: &mut GMContext) {
        if self.active {
            self.animation.update();
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

gen_container_type!(GMSprite, GMSpriteBase, GMSpriteEffectT);

impl GMSprite {
    pub fn new<T: Into<GMVec2D>>(texture: Arc<GMTexture>, position: T, animation: GMAnimation) -> Self {
        let base = GMSpriteBase::new(texture, position, animation);

        Self {
            base,
            effects: Vec::new(),
            active: true,
            visible: true,
        }
    }

    gen_type_effect_methods!(GMSpriteBase, GMSpriteEffectT);
}

gen_effect_impl_for_type!(GMSprite);

gen_effect_trait!(GMSpriteEffectT, GMSpriteBase);
