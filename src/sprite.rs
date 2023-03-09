
use std::sync::Arc;

use hecs::{World, Entity, Component, EntityBuilder};

use crate::texture::{GMTexture, GMTextureIndex, GMSharedTexture};
use crate::math::{GMVec2D, GMPosition, GMScale, GMAngle, GMFlipXY};
use crate::util::{GMActive, GMVisible};


pub struct GMSpriteBuilder {
    entity_builder: EntityBuilder,
}

impl GMSpriteBuilder {
    pub fn new<P: Into<GMVec2D>>(texture: Arc<GMTexture>, position: P) -> Self {
        let mut entity_builder = EntityBuilder::new();
        entity_builder.add(GMSharedTexture(texture))
            .add(GMPosition(position.into()))
            .add(GMTextureIndex(0))
            .add(GMScale(1.0))
            .add(GMAngle(0.0))
            .add(GMFlipXY(false, false))
            .add(GMActive(true))
            .add(GMVisible(true));

        Self {
            entity_builder,
        }
    }

    pub fn texture_index(mut self, texture_index: u32) -> Self {
        self.entity_builder.add(GMTextureIndex(texture_index));
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.entity_builder.add(GMScale(scale));
        self
    }

    pub fn angle(mut self, angle: f32) -> Self {
        self.entity_builder.add(GMAngle(angle));
        self
    }

    pub fn flip_xy(mut self, flip_x: bool, flip_y: bool) -> Self {
        self.entity_builder.add(GMFlipXY(flip_x, flip_y));
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.entity_builder.add(GMActive(active));
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.entity_builder.add(GMVisible(visible));
        self
    }

    pub fn add_component<T: Component>(mut self, component: T) -> Self {
        self.entity_builder.add(component);
        self
    }

    pub fn build(mut self, world: &mut World) -> Entity {
        let built_entity = self.entity_builder.build();
        world.spawn(built_entity)
    }
}
