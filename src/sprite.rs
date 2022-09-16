
use std::rc::Rc;

use log::debug;

use crate::texture::GMTexture;
use crate::animation::{GMAnimation};
use crate::sprite_effect::{GMSpriteEffectT};
use crate::context::GMContext;
use crate::math::GMVec2D;

#[derive(Debug, Clone)]
pub struct GMSpriteBase {
    position: GMVec2D,
    velocity: GMVec2D,
    acceleration: GMVec2D,

    angle: f32,
    angle_velocity: f32,
    angle_acceleration: f32,
    flip_x: bool,
    flip_y: bool,
    scale: f32,

    texture: Rc<GMTexture>,
    animation: GMAnimation,

    visible: bool,
    active: bool,

    // User defined data:
    id: u32,
    group_id: u32,
    name: String,
    custom_data: String,
}

// TODO: Maybe use https://github.com/jbaublitz/getset

impl GMSpriteBase {
    pub fn new(texture: &Rc<GMTexture>) -> Self {
        debug!("GMSpriteBase::new()");

        Self {
            position: GMVec2D::new(0.0, 0.0),
            velocity: GMVec2D::new(0.0, 0.0),
            acceleration: GMVec2D::new(0.0, 0.0),

            angle: 0.0,
            angle_velocity: 0.0,
            angle_acceleration: 0.0,
            flip_x: false,
            flip_y: false,
            scale: 1.0,

            texture: texture.clone(),
            animation: GMAnimation::new(&[(0, 0.0)]),

            visible: true,
            active: true,

            id: 0,
            group_id: 0,
            name: "".to_string(),
            custom_data: "".to_string()
        }

    }

    pub fn position(&self) -> &GMVec2D {
        &self.position
    }

    pub fn position_mut(&mut self) -> &mut GMVec2D {
        &mut self.position
    }

    pub fn velocity(&self) -> &GMVec2D {
        &self.velocity
    }

    pub fn velocity_mut(&mut self) -> &mut GMVec2D {
        &mut self.velocity
    }

    pub fn acceleration(&self) -> &GMVec2D {
        &self.acceleration
    }

    pub fn acceleration_mut(&mut self) -> &mut GMVec2D {
        &mut self.acceleration
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn angle_mut(&mut self) -> &mut f32 {
        &mut self.angle
    }

    pub fn angle_velocity(&self) -> f32 {
        self.angle_velocity
    }

    pub fn angle_velocity_mut(&mut self) -> &mut f32 {
        &mut self.angle_velocity
    }

    pub fn angle_acceleration(&self) -> f32 {
        self.angle_acceleration
    }

    pub fn angle_acceleration_mut(&mut self) -> &mut f32 {
        &mut self.angle_acceleration
    }

    pub fn flip_x(&self) -> bool {
        self.flip_x
    }

    pub fn flip_x_mut(&mut self) -> &mut bool {
        &mut self.flip_x
    }

    pub fn flip_y(&self) -> bool {
        self.flip_y
    }

    pub fn flip_y_mut(&mut self) -> &mut bool {
        &mut self.flip_y
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn scale_mut(&mut self) -> &mut f32 {
        &mut self.scale
    }

    pub fn texture(&self) -> &Rc<GMTexture> {
        &self.texture
    }

    pub fn texture_mut(&mut self) -> &mut Rc<GMTexture> {
        &mut self.texture
    }

    pub fn set_animation(&mut self, animation: GMAnimation) {
        self.animation = animation;
    }

    pub fn animation(&self) -> &GMAnimation {
        &self.animation
    }

    pub fn animation_mut(&mut self) -> &mut GMAnimation {
        &mut self.animation
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn active_mut(&mut self) -> &mut bool {
        &mut self.active
    }

    pub fn id(&mut self) -> u32 {
        self.id
    }

    pub fn id_mut(&mut self) -> &mut u32 {
        &mut self.id
    }

    pub fn group_id(&self) -> u32 {
        self.group_id
    }

    pub fn group_id_mut(&mut self) -> &mut u32 {
        &mut self.group_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut str {
        &mut self.name
    }

    pub fn custom_data(&self) -> &str {
        &self.custom_data
    }

    pub fn custom_data_mut(&mut self) -> &mut str {
        &mut self.custom_data
    }

    pub fn update(&mut self, _context: &mut GMContext) {
        if self.active {
            self.animation.base_mut().update();
        }
    }

    pub fn move_step(&mut self) {
        if self.active {
            self.position.add2(&self.velocity);
            self.velocity.add2(&self.acceleration);

            self.angle += self.angle_velocity;
            self.angle_velocity += self.angle_acceleration;
        }
    }

    pub fn update_move(&mut self, context: &mut GMContext) {
        self.update(context);
        self.move_step();
    }

    pub fn draw(&self, context: &mut GMContext) {
        if self.visible {
            let index = self.animation.base().texture_index();
            self.texture.draw_opt(self.position.x, self.position.y, index,
                self.angle, self.flip_x, self.flip_y, context);
        }
    }

}

#[derive(Debug, Clone)]
pub struct GMSprite {
    base: GMSpriteBase,
    effects: Vec<Box<dyn GMSpriteEffectT>>,
}

impl GMSprite {
    pub fn new(texture: &Rc<GMTexture>) -> Self {
        debug!("GMSprite::new()");

        Self {
            base: GMSpriteBase::new(texture),
            effects: Vec::new(),
        }
    }

    pub fn update(&mut self, context: &mut GMContext) {
        self.base.update(context);

        if self.base.active() {
            for effect in self.effects.iter_mut() {
                effect.update(&mut self.base, context);
            }
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        self.base.draw(context);

        if self.base.visible() {
            for effect in self.effects.iter() {
                effect.draw(&self.base, context);
            }
        }
    }

    pub fn base(&self) -> &GMSpriteBase {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut GMSpriteBase {
        &mut self.base
    }

    // Sprite effect methods
    pub fn add_effect<T: 'static + GMSpriteEffectT>(&mut self, effect: T) {
        debug!("GMSprite::add_effect()");
        self.add_effect2(Box::new(effect));
    }

    pub fn add_effect2(&mut self, effect: Box<dyn GMSpriteEffectT>) {
        debug!("GMSprite::add_effect2()");
        self.effects.push(effect);
    }

    pub fn set_effects(&mut self, effects: Vec<Box<dyn GMSpriteEffectT>>) {
        debug!("GMSprite::set_effects()");
        self.effects = effects;
    }

    pub fn remove_effect(&mut self, index: usize) {
        debug!("GMSprite::remove_effect(), index: {}", index);
        self.effects.remove(index);
    }

    pub fn swap_effects(&mut self, index1: usize, index2: usize) {
        debug!("GMSprite::swap_effect(), index1: {}, index2: {}", index1, index2);
        self.effects.swap(index1, index2);
    }

    pub fn send_effect_message(&mut self, index: usize, message: &str, context: &mut GMContext) {
        self.effects[index].send_message(message, context)
    }
}

// TODO: Maybe use https://github.com/colin-kiegel/rust-derive-builder

pub struct GMSpriteBuilder {
    sprite: GMSprite,
}

impl GMSpriteBuilder {
    pub fn new(texture: &Rc<GMTexture>) -> GMSpriteBuilder {
        Self {
            sprite: GMSprite::new(texture),
        }
    }

    pub fn with_position<T: Into<GMVec2D>>(mut self, position: T) -> Self {
        let position = position.into();
        debug!("GMSpriteBuilder::with_position(), position: '{:?}'", position);

        self.sprite.base.position = position;
        self
    }

    pub fn with_velocity<T: Into<GMVec2D>>(mut self, velocity: T) -> Self {
        let velocity = velocity.into();
        debug!("GMSpriteBuilder::with_velocity(), velocity: '{:?}'", velocity);

        self.sprite.base.velocity = velocity;
        self
    }

    pub fn with_acceleration<T: Into<GMVec2D>>(mut self, acceleration: T) -> Self {
        let acceleration = acceleration.into();
        debug!("GMSpriteBuilder::with_acceleration(), acceleration: '{:?}'", acceleration);

        self.sprite.base.acceleration = acceleration;
        self
    }

    pub fn with_angle(mut self, angle: f32) -> Self {
        debug!("GMSpriteBuilder::with_angle(), angle: '{}'", angle);

        self.sprite.base.angle = angle;
        self
    }

    pub fn with_flip_x(mut self, flip_x: bool) -> Self {
        debug!("GMSpriteBuilder::with_flip_x(), flip_x: '{}'", flip_x);

        self.sprite.base.flip_x = flip_x;
        self
    }

    pub fn with_flip_y(mut self, flip_y: bool) -> Self {
        debug!("GMSpriteBuilder::with_flip_y(), flip_y: '{}'", flip_y);

        self.sprite.base.flip_y = flip_y;
        self
    }

    pub fn with_animation(mut self, animation: GMAnimation) -> Self {
        debug!("GMSpriteBuilder::with_animation()");

        self.sprite.base.animation = animation;
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        debug!("GMSpriteBuilder::with_visible(), visible: '{}'", visible);

        self.sprite.base.visible = visible;
        self
    }


    pub fn with_active(mut self, active: bool) -> Self {
        debug!("GMSpriteBuilder::with_active(), active: '{}'", active);

        self.sprite.base.active = active;
        self
    }

    pub fn with_id(mut self, id: u32) -> Self {
        debug!("GMSpriteBuilder::with_id(), id: '{}'", id);

        self.sprite.base.id = id;
        self
    }

    pub fn with_group_id(mut self, group_id: u32) -> Self {
        debug!("GMSpriteBuilder::with_group_id(), group_id: '{}'", group_id);

        self.sprite.base.group_id = group_id;
        self
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        let name = name.into();
        debug!("GMSpriteBuilder::with_name(), name: '{}'", name);

        self.sprite.base.name = name;
        self
    }

    pub fn with_custom_data<S: Into<String>>(mut self, custom_data: S) -> Self {
        let custom_data = custom_data.into();
        debug!("GMSpriteBuilder::with_custom_data(), custom_data: '{}'", custom_data);

        self.sprite.base.custom_data = custom_data.to_string();
        self
    }

    pub fn with_effect<T: 'static + GMSpriteEffectT>(mut self, effect: T) -> Self {
        debug!("GMSpriteBuilder::with_effect()");

        self.sprite.effects.push(Box::new(effect));
        self
    }

    pub fn with_effect2(mut self, effect: Box<dyn GMSpriteEffectT>) -> Self {
        debug!("GMSpriteBuilder::with_effect2()");

        self.sprite.effects.push(effect);
        self
    }

    pub fn with_effects(mut self, effects: Vec<Box<dyn GMSpriteEffectT>>) -> Self {
        debug!("GMSpriteBuilder::with_effects()");

        self.sprite.effects.extend(effects);
        self
    }

    pub fn build(self) -> GMSprite {
        self.sprite
    }

}
