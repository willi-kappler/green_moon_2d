
use std::rc::Rc;
use std::collections::HashSet;

use log::debug;

use crate::data::GMData;
use crate::texture::GMTexture;
use crate::animation::{GMAnimation};
use crate::context::{GMContext, GMObjectMessage};
use crate::math::GMVec2D;
use crate::effect::{GMEffectManager, GMEffectT};
use crate::util::error_panic;

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
    pub name: String,
    pub groups: HashSet<String>,
    pub custom_data: GMData,
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

            name: "".to_string(),
            groups: HashSet::new(),
            custom_data: GMData::None,
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

    pub fn send_message(&mut self, message: &str, _context: &mut GMContext) {
        // error_panic("GMSpriteBase::send_message(), no message defined yet");
        match message {
            "clear_groups" => {
                self.groups.clear();
            }
            _ => {
                error_panic(&format!("GMSpriteBase::send_message(), unknown message: '{}'", message))
            }
        }
    }

    pub fn send_message_data(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_position" => {
                self.position = data.into();
            }
            "set_offset" => {
                self.offset = data.into();
            }
            "set_velocity" => {
                self.velocity = data.into();
            }
            "set_acceleration" => {
                self.acceleration = data.into();
            }
            "set_angle" => {
                self.angle = data.into();
            }
            "set_angle_velocity" => {
                self.angle_velocity = data.into();
            }
            "set_angle_acceleration" => {
                self.angle_acceleration = data.into();
            }
            "set_flip_x" => {
                self.flip_x = data.into();
            }
            "set_flip_y" => {
                self.flip_y = data.into();
            }
            "set_texture" => {
                self.texture = data.into();
            }
            "set_animation" => {
                self.animation = data.into();
            }
            "set_visible" => {
                self.visible = data.into();
            }
            "set_active" => {
                self.active = data.into();
            }
            "set_name" => {
                self.name = data.into();
            }
            "add_group" => {
                self.groups.insert(data.into());
            }
            "remove_group" => {
                let group: String = data.into();
                self.groups.remove(&group);
            }
            _ => {
                error_panic(&format!("GMSpriteBase::send_message_data(), unknown message: '{}'", message))
            }
        }
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

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        let name = name.into();
        debug!("GMSpriteBuilder::with_name(), name: '{}'", name);

        self.sprite.base.name = name;
        self
    }

    pub fn with_group<S: Into<String>>(mut self, group: S) -> Self {
        let group = group.into();
        debug!("GMSpriteBuilder::with_group(), group: '{}'", group);

        self.sprite.base.groups.insert(group);
        self
    }

    pub fn with_groups(mut self, groups: HashSet<String>) -> Self {
        debug!("GMSpriteBuilder::with_groups(), groups: '{:?}'", groups);

        self.sprite.base.groups = groups;
        self
    }

    pub fn with_custom_data(mut self, custom_data: GMData) -> Self {
        debug!("GMSpriteBuilder::with_custom_data(), custom_data: '{:?}'", custom_data);

        self.sprite.base.custom_data = custom_data;
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
