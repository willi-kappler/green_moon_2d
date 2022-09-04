
use std::any::Any;
use std::rc::Rc;

use log::debug;

use crate::texture::GMTexture;
use crate::animation::{GMAnimationT, GMAnimationStatic};
use crate::sprite_effect::{GMSpriteEffectT};
use crate::context::GMContext;
use crate::math::GMVec2D;

#[derive(Debug, Clone)]
pub struct GMSpriteBase {
    position: GMVec2D,
    velocity: GMVec2D,
    acceleration: GMVec2D,

    angle: f32,
    flip_x: bool,
    flip_y: bool,

    texture: Rc<GMTexture>,
    animation: Box<dyn GMAnimationT>,

    visible: bool,
    active: bool,
}

impl GMSpriteBase {
    pub fn new(texture: Rc<GMTexture>) -> Self {
        debug!("GMSpriteBase::new()");

        Self {
            position: GMVec2D::new(0.0, 0.0),
            velocity: GMVec2D::new(0.0, 0.0),
            acceleration: GMVec2D::new(0.0, 0.0),
            angle: 0.0,
            flip_x: false,
            flip_y: false,
            texture,
            animation: Box::new(GMAnimationStatic::new(0)),
            visible: true,
            active: true,
        }

    }

    pub fn set_position(&mut self, position: GMVec2D) {
        self.position = position;
    }

    pub fn get_position(&self) -> &GMVec2D {
        &self.position
    }

    pub fn set_velocity(&mut self, velocity: GMVec2D) {
        self.velocity = velocity;
    }

    pub fn get_velocity(&self) -> &GMVec2D {
        &self.velocity
    }

    pub fn set_acceleration(&mut self, acceleration: GMVec2D) {
        self.acceleration = acceleration;
    }

    pub fn get_acceleration(&self) -> &GMVec2D {
        &self.acceleration
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    pub fn set_flip_x(&mut self, flip_x: bool) {
        self.flip_x = flip_x;
    }

    pub fn get_flip_x(&self) -> bool {
        self.flip_x
    }

    pub fn set_flip_y(&mut self, flip_y: bool) {
        self.flip_y = flip_y;
    }

    pub fn get_flip_y(&self) -> bool {
        self.flip_y
    }

    pub fn set_texture(&mut self, texture: &Rc<GMTexture>) {
        self.texture = texture.clone();
    }

    pub fn get_texture(&self) -> &Rc<GMTexture> {
        &self.texture
    }

    pub fn set_animation<T: 'static + GMAnimationT>(&mut self, animation: T) {
        self.animation = Box::new(animation);
    }

    pub fn get_animation(&self) -> &Box<dyn GMAnimationT> {
        &self.animation
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn get_visible(&self) -> bool {
        self.visible
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    pub fn update(&mut self, _context: &mut GMContext) {
        if self.active {
            self.animation.update();

            self.position.add2(&self.velocity);
            self.velocity.add2(&self.acceleration);
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        if self.visible {
            let index = self.animation.texture_index();
            self.texture.draw_opt(self.position.x, self.position.y, index,
                self.angle, self.flip_x, self.flip_y, context);
        }
    }

}

pub struct GMSprite {
    base: GMSpriteBase,
    effects: Vec<Box<dyn GMSpriteEffectT>>,
}

impl GMSprite {
    pub fn new(texture: Rc<GMTexture>) -> Self {
        debug!("GMSprite::new()");

        Self {
            base: GMSpriteBase::new(texture),
            effects: Vec::new(),
        }
    }

    // Builder pattern
    pub fn with_position<T: Into<GMVec2D>>(mut self, position: T) -> Self {
        self.base.position = position.into();
        self
    }

    pub fn with_velocity<T: Into<GMVec2D>>(mut self, velocity: T) -> Self {
        self.base.velocity = velocity.into();
        self
    }

    pub fn with_acceleration<T: Into<GMVec2D>>(mut self, acceleration: T) -> Self {
        self.base.acceleration = acceleration.into();
        self
    }

    pub fn with_angle(mut self, angle: f32) -> Self {
        self.base.angle = angle;
        self
    }

    pub fn with_flip_x(mut self, flip_x: bool) -> Self {
        self.base.flip_x = flip_x;
        self
    }

    pub fn with_flip_y(mut self, flip_y: bool) -> Self {
        self.base.flip_y = flip_y;
        self
    }

    pub fn with_animation<T: 'static + GMAnimationT>(mut self, animation: T) -> Self {
        self.base.animation = Box::new(animation);
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.base.visible = visible;
        self
    }


    pub fn with_active(mut self, active: bool) -> Self {
        self.base.active = active;
        self
    }

    // Sprite methods
    pub fn update(&mut self, context: &mut GMContext) {
        self.base.update(context);

        if self.base.get_active() {
            for effect in self.effects.iter_mut() {
                effect.update(&mut self.base, context);
            }
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        self.base.draw(context);

        if self.base.get_visible() {
            for effect in self.effects.iter() {
                effect.draw(&self.base, context);
            }
        }
    }

    pub fn get_base(&self) -> &GMSpriteBase {
        &self.base
    }

    pub fn get_mut_base(&mut self) -> &mut GMSpriteBase {
        &mut self.base
    }

    // Sprite effect methods
    pub fn add_effect<T: 'static + GMSpriteEffectT>(&mut self, effect: T) {
        self.effects.push(Box::new(effect));
    }

    pub fn remove_effect(&mut self, index: usize) {
        self.effects.remove(index);
    }

    pub fn swap_effects(&mut self, index1: usize, index2: usize) {
        self.effects.swap(index1, index2);
    }

    pub fn send_effect_message(&mut self, index: usize, message: &str, context: &mut GMContext) {
        self.effects[index].send_message(message, context)
    }

    pub fn send_effect_message_data(&mut self, index: usize, message: &str, data: Box<dyn Any>, context: &mut GMContext) {
        self.effects[index].send_message_data(message, data, context)
    }
}
