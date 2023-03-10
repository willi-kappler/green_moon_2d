

use std::collections::{HashMap};
use std::sync::Arc;
use std::fmt::Debug;

use log::debug;
use hecs::{World, Entity, Component, EntityBuilder};


use crate::sprite::GMSpriteBuilder;
use crate::texture::GMTexture;
use crate::util::{error_panic, GMAlign};
use crate::math::{GMVec2D, GMSize, GMPosition, GMRelPosition};

#[derive(Debug, Clone)]
pub struct GMBitmapFont {
    texture: Arc<GMTexture>,
    mapping: HashMap<char, u32>,
}

impl GMBitmapFont {
    pub fn new(texture: Arc<GMTexture>, char_mapping: &str) -> Self {
        debug!("GMBitmapFont::new(), char_mapping: '{}'", char_mapping);

        let mut mapping = HashMap::new();

        for (i, c) in char_mapping.chars().enumerate() {
            mapping.insert(c, i as u32);
        }

        Self {
            texture: texture,
            mapping,
        }
    }

    pub fn get_char_dimensions(&self) -> (f32, f32) {
        self.texture.get_unit_dimension()
    }

    pub fn get_index(&self, c: char) -> u32 {
        match self.mapping.get(&c) {
            Some(index) => {
                *index
            }
            None => {
                error_panic(&format!("GMBitmapFont::draw_opt(), Character '{}' not in map.", c));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMBitmapText { // TODO: Remove struct and use components instead ?
    pub font: Arc<GMBitmapFont>,
    pub text: String,
    pub spacing: GMVec2D,
    pub horizontal: bool,
    pub align: GMAlign,
    pub size: GMSize,
    pub chars: Vec<Entity>,
}

impl GMBitmapText {
    pub fn reset_chars(&mut self, world: &mut World, position: &GMPosition) {
        // Remove all the characters and recreate them
        for e in self.chars.iter() {
            world.despawn(*e).unwrap();
        }

        self.chars.clear();

        for c in self.text.chars() {
            let index = self.font.get_index(c);
            let sprite = GMSpriteBuilder::new(self.font.texture.clone(), (0.0, 0.0))
                .texture_index(index)
                .add_component(GMRelPosition(GMVec2D::new(0.0, 0.0)))
                .build(world);
            self.chars.push(sprite);
        }

        self.reset_chars_pos(world, position);
    }

    pub fn reset_chars_pos(&mut self, world: &mut World, text_position: &GMPosition) {
        let (dx, dy) = self.font.get_char_dimensions();
        let num_of_chars = self.chars.len() as f32;
        let mut x: f32;
        let mut y: f32;
        let mut dx2 = dx + self.spacing.x;
        let mut dy2 = dy + self.spacing.y;

        if self.horizontal {
            self.size.width = (dx * num_of_chars) + (self.spacing.x * (num_of_chars - 1.0));
            self.size.height = dy;
            dy2 = 0.0;
        } else {
            self.size.width = dx;
            self.size.height = (dy * num_of_chars) + (self.spacing.y * (num_of_chars - 1.0));
            dx2 = 0.0;
        }

        let text_position = text_position.0;

        match self.align {
            GMAlign::TopLeft => {
                x = text_position.x;
                y = text_position.y;
            }
            GMAlign::TopCenter => {
                x = text_position.x - (self.size.width / 2.0);
                y = text_position.y;
            }
            GMAlign::TopRight => {
                x = text_position.x - self.size.width;
                y = text_position.y;
            }
            GMAlign::MiddleLeft => {
                x = text_position.x;
                y = text_position.y - (self.size.height / 2.0);
            }
            GMAlign::MiddleCenter => {
                x = text_position.x - (self.size.width / 2.0);
                y = text_position.y - (self.size.height / 2.0);
            }
            GMAlign::MiddleRight => {
                x = text_position.x - self.size.width;
                y = text_position.y - (self.size.height / 2.0);
            }
            GMAlign::BottomLeft => {
                x = text_position.x;
                y = text_position.y - self.size.height;
            }
            GMAlign::BottomCenter => {
                x = text_position.x - (self.size.width / 2.0);
                y = text_position.y - self.size.height;
            }
            GMAlign::BottomRight => {
                x = text_position.x - self.size.width;
                y = text_position.y - self.size.height;
            }
        }

        for e in self.chars.iter() {
            let (char_position, char_rel_position) = world.query_one_mut::<(&mut GMPosition, &mut GMRelPosition)>(*e).unwrap();

            char_position.0.x = x;
            char_position.0.y = y;

            char_rel_position.0.x = x - text_position.x;
            char_rel_position.0.y = x - text_position.y;

            x += dx2;
            y += dy2;
        }
    }
}

pub struct GMBitmapTextBuilder {
    font: Arc<GMBitmapFont>,
    text: String,
    position: GMVec2D,
    spacing: GMVec2D,
    horizontal: bool,
    align: GMAlign,
    entity_builder: EntityBuilder,
}

impl GMBitmapTextBuilder {
    pub fn new<S: Into<String>, T: Into<GMVec2D>>(font: Arc<GMBitmapFont>, text: S, position: T) -> Self {
        Self {
            font,
            text: text.into(),
            position: position.into(),
            spacing: GMVec2D::new(0.0, 0.0),
            horizontal: true,
            align: GMAlign::TopLeft,
            entity_builder: EntityBuilder::new(),
        }
    }

    pub fn spacing<T: Into<GMVec2D>>(mut self, spacing: T) -> Self {
        self.spacing = spacing.into();
        self
    }

    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    pub fn align(mut self, align: GMAlign) -> Self {
        self.align = align;
        self
    }

    pub fn add_component<T: Component>(mut self, component: T) -> Self {
        self.entity_builder.add(component);
        self
    }

    pub fn build(mut self, world: &mut World) -> Entity {
        let size = GMSize::new(0.0, 0.0);
        let chars = Vec::new();

        let mut bitmap_text = GMBitmapText {
            font: self.font,
            text: self.text,
            spacing: self.spacing,
            horizontal: self.horizontal,
            align: self.align,
            size,
            chars,
        };

        let position = GMPosition(self.position);

        bitmap_text.reset_chars(world, &position);

        self.entity_builder.add(bitmap_text);
        self.entity_builder.add(position);
        let built_entity = self.entity_builder.build();
        world.spawn(built_entity)
    }
}

// ECS systems:

pub fn update_character_positions(world: &mut World) {
    for (_, (bitmap_text, text_position)) in
        world.query::<(&GMBitmapText, &GMPosition)>().iter() {
        for char in bitmap_text.chars.iter() {
            let mut query = world.query_one::<(&mut GMPosition, &GMRelPosition)>(*char).unwrap();
            let (char_position, char_rel_position) = query.get().unwrap();
            
            char_position.0 = text_position.0 + char_rel_position.0;
        }
    }
}
