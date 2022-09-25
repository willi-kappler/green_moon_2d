
use std::rc::Rc;

use log::debug;

use crate::data::GMData;
use crate::texture::GMTexture;
use crate::animation::{GMAnimation};
use crate::context::{GMContext, GMObjectMessage};
use crate::math::GMVec2D;
use crate::effect::{GMEffectManager, GMEffectT};

#[derive(Debug, Clone)]
pub struct GMSpriteBase {
    pub position: GMVec2D,
    pub offset: GMVec2D,
    pub velocity: GMVec2D,
    pub acceleration: GMVec2D,

    pub angle: f32,
    pub angle_velocity: f32,
    pub angle_acceleration: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub scale: f32,

    pub texture: Rc<GMTexture>,
    pub animation: GMAnimation,

    pub visible: bool,
    pub active: bool,

    // User defined data:
    pub id: u32,
    pub group_id: u32,
    pub name: String,
    pub custom_data: String,
}

// TODO: Maybe use https://github.com/jbaublitz/getset

impl GMSpriteBase {
    pub fn new(texture: &Rc<GMTexture>) -> Self {
        debug!("GMSpriteBase::new()");

        Self {
            position: GMVec2D::new(0.0, 0.0),
            offset: GMVec2D::new(0.0, 0.0),
            velocity: GMVec2D::new(0.0, 0.0),
            acceleration: GMVec2D::new(0.0, 0.0),

            angle: 0.0,
            angle_velocity: 0.0,
            angle_acceleration: 0.0,
            flip_x: false,
            flip_y: false,
            scale: 1.0,

            texture: texture.clone(),
            animation: GMAnimation::new("", &[(0, 0.0)]),

            visible: true,
            active: true,

            id: 0,
            group_id: 0,
            name: "".to_string(),
            custom_data: "".to_string()
        }

    }

    pub fn update(&mut self, context: &mut GMContext) {
        if self.active {
            self.animation.update(context);
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
            let index = self.animation.base.texture_index();
            let x = self.position.x + self.offset.x;
            let y = self.position.y + self.offset.y;

            self.texture.draw_opt(x, y, index,
                self.angle, self.flip_x, self.flip_y, context);
        }
    }

    pub fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
        todo!();
    }

    pub fn send_message_data(&mut self, _message: &str, _data: GMData, _context: &mut GMContext) {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct GMSprite {
    pub base: GMSpriteBase,
    pub effects: GMEffectManager<GMSpriteBase>,
}

impl GMSprite {
    pub fn new(texture: &Rc<GMTexture>) -> Self {
        debug!("GMSprite::new()");

        Self {
            base: GMSpriteBase::new(texture),
            effects: GMEffectManager::new(),
        }
    }

    pub fn update(&mut self, context: &mut GMContext) {
        self.base.update(context);

        if self.base.active {
            self.effects.update(&mut self.base, context);
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        self.base.draw(context);

        if self.base.visible {
            self.effects.draw(&self.base, context);
        }
    }

    pub fn check_messages(&mut self, context: &mut GMContext) {
        let mut messages = context.get_object_messages(&self.base.name);

        while let Some(message) = messages.pop_front() {
            match message {
                GMObjectMessage::Simple(message) => {
                    self.base.send_message(&message, context);
                }
                GMObjectMessage::Data(message, data) => {
                    self.base.send_message_data(&message, data, context);
                }
                GMObjectMessage::SimpleEffect(index, message) => {
                    self.effects.send_effect_message(index, &message, context);
                }
                GMObjectMessage::DataEffect(index, message, data) => {
                    self.effects.send_effect_message_data(index, &message, data, context);
                }
            }
        }
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

    pub fn with_effect<T: 'static + GMEffectT<GMSpriteBase>>(mut self, effect: T) -> Self {
        debug!("GMSpriteBuilder::with_effect()");

        self.sprite.effects.add_effect(effect);
        self
    }

    pub fn with_effect2(mut self, effect: Box<dyn GMEffectT<GMSpriteBase>>) -> Self {
        debug!("GMSpriteBuilder::with_effect2()");

        self.sprite.effects.add_effect2(effect);
        self
    }

    pub fn with_effects(mut self, effects: Vec<Box<dyn GMEffectT<GMSpriteBase>>>) -> Self {
        debug!("GMSpriteBuilder::with_effects()");

        self.sprite.effects.set_effects(effects);
        self
    }

    pub fn build(self) -> GMSprite {
        self.sprite
    }

}
