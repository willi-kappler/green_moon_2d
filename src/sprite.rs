

use std::sync::Arc;

use hecs::World;

use crate::math::GMPosition;
use crate::context::GMContext;
use crate::texture::{GMTexture, GMTextureIndex};


#[derive(Debug, Clone)]
pub struct GMSprite {
    pub texture: Arc<GMTexture>,
}

pub fn draw_sprites(world: &mut World, context: &mut GMContext) {
    for (e, (sprite, position, index)) in world.query::<(&GMSprite, &GMPosition, &GMTextureIndex)>().iter() {
        let v = position.0;
        let x = v.x;
        let y = v.y;
        sprite.texture.draw(x, y, index.0, context);
    }
}

/*
use crate::data::GMData;
use crate::texture::GMTexture;
use crate::animation::{GMAnimation};
use crate::context::GMContext;
use crate::math::GMVec2D;
use crate::effect::{GMEffectManager, GMEffectT};
use crate::util::error_panic;
use crate::object_manager::{GMObjectBaseT, GMObjectManager};

use crate::{return_name_and_groups, create_builder_methods};

#[derive(Debug, Clone)]
pub struct GMSpriteBase {
    pub position: GMVec2D,
    pub offset: GMVec2D,

    pub angle: f32,
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

impl GMSpriteBase {
    pub fn new(texture: &Rc<GMTexture>) -> Self {
        debug!("GMSpriteBase::new()");

        Self {
            position: GMVec2D::new(0.0, 0.0),
            offset: GMVec2D::new(0.0, 0.0),

            angle: 0.0,
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
}

impl GMObjectBaseT for GMSpriteBase {
    fn update(&mut self, context: &mut GMContext) {
        if self.active {
            self.animation.update(context);
        }
    }

    fn draw(&self, context: &mut GMContext) {
        if self.visible {
            let index = self.animation.base.texture_index();
            let x = self.position.x + self.offset.x;
            let y = self.position.y + self.offset.y;

            self.texture.draw_opt(x, y, index, self.angle, self.scale,
                self.flip_x, self.flip_y, context);
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_position" => {
                self.position = data.into();
            }
            "set_offset" => {
                self.offset = data.into();
            }
            "set_angle" => {
                self.angle = data.into();
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
            "clear_group" => {
                self.groups.clear();
            }
            _ => {
                error_panic(&format!("GMSpriteBase::send_message(), unknown message: '{}'", message))
            }
        }
    }

    return_name_and_groups!();
}

pub type GMSprite = GMObjectManager<GMSpriteBase>;

impl GMSprite {
    pub fn new(texture: &Rc<GMTexture>) -> Self {
        Self {
            base: GMSpriteBase::new(texture),
            effects: GMEffectManager::new(),
        }
    }
}

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

    pub fn with_custom_data(mut self, custom_data: GMData) -> Self {
        debug!("GMSpriteBuilder::with_custom_data(), custom_data: '{:?}'", custom_data);

        self.sprite.base.custom_data = custom_data;
        self
    }

    create_builder_methods!(GMSpriteBuilder, GMSpriteBase, sprite);

    pub fn build(self) -> GMSprite {
        self.sprite
    }
}
*/
