
use std::collections::HashSet;

use crate::math::GMVec2D;
use crate::sprite::GMSprite;
use crate::context::GMContext;
use crate::effect::{GMEffectManager, GMEffectT};
use crate::object_manager::{GMObjectBaseT, GMObjectManager};

use log::debug;

#[derive(Debug, Clone)]
pub enum GMLineMode {
    Number(u32),
    Spacing(f32),
}

#[derive(Debug, Clone)]
pub struct GMLineBase {
    pub start: GMVec2D,
    pub end: GMVec2D,
    pub init_sprite: GMSprite,
    pub sprites: Vec<GMSprite>,
    pub line_mode: GMLineMode,
    pub name: String,
    pub groups: HashSet<String>,
}

// TODO: Maybe add effect for lines

impl GMLineBase {
    pub fn new<V: Into<GMVec2D>>(start: V, end: V, sprite: GMSprite, number: u32) -> Self {
        let mut result = Self {
            start: start.into(),
            end: end.into(),
            init_sprite: sprite,
            sprites: Vec::new(),
            line_mode: GMLineMode::Number(number),
            name: "".to_string(),
            groups: HashSet::new(),
        };

        result.end_point_changed();
        result
    }

    pub fn new2<V: Into<GMVec2D>>(start: V, end: V, sprite: GMSprite, spacing: f32) -> Self {
        let mut result = Self {
            start: start.into(),
            end: end.into(),
            init_sprite: sprite,
            sprites: Vec::new(),
            line_mode: GMLineMode::Spacing(spacing),
            name: "".to_string(),
            groups: HashSet::new(),
        };

        result.end_point_changed();
        result
    }

    pub fn set_start<V: Into<GMVec2D>>(&mut self, start: V) {
        self.start = start.into();

        self.end_point_changed();
    }

    pub fn set_end<V: Into<GMVec2D>>(&mut self, end: V) {
        self.end = end.into();

        self.end_point_changed();
    }

    pub fn end_point_changed(&mut self) {
        let direction = self.end - self.start;
        let length = direction.len();

        match self.line_mode {
            GMLineMode::Number(number) => {
                let spacing = length / (number as f32);
                self.set_sprites(number, spacing, direction);
            }
            GMLineMode::Spacing(spacing) => {
                let number = (length / spacing).floor() as u32;
                self.set_sprites(number, spacing, direction);
            }
        }
    }

    pub fn set_number(&mut self, number: u32) {
        self.line_mode = GMLineMode::Number(number);

        let direction = self.end - self.start;
        let length = direction.len();
        let spacing = length / (number as f32);

        self.set_sprites(number, spacing, direction);
    }

    pub fn set_spacing(&mut self, spacing: f32) {
        self.line_mode = GMLineMode::Spacing(spacing);

        let direction = self.end - self.start;
        let length = direction.len();
        let number = (length / spacing).floor() as u32;

        self.set_sprites(number, spacing, direction);
    }

    pub fn set_sprites(&mut self, number: u32, spacing: f32, mut direction: GMVec2D) {
        direction.norm();

        // If more sprites are needed just add them
        let diff = ((number as i32) - (self.sprites.len() as i32)) as i32;

        for _ in 0..diff {
            self.sprites.push(self.init_sprite.clone());
        }

        // Now re-calculate the positions of all sprites, and disable the ones that are not needed.
        for i in 0..self.sprites.len() {
            let sprite = &mut self.sprites[i].base;

            if i <= (number as usize) {
                let new_position = self.start + (direction * (spacing * (i as f32)));
                sprite.position = new_position;
                sprite.active = true;
                sprite.visible = true;
            } else {
                sprite.active = false;
                sprite.visible = false;
            }
        }
    }
}

impl GMObjectBaseT for GMLineBase {
    fn update(&mut self, context: &mut GMContext) {
        for sprite in self.sprites.iter_mut() {
            sprite.update(context);
        }
    }

    fn draw(&self, context: &mut GMContext) {
        for sprite in self.sprites.iter() {
            sprite.draw(context);
        }
    }

    fn send_message(&mut self, message: &str, data: crate::data::GMData, context: &mut GMContext) {
        todo!()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn groups(&self) -> &std::collections::HashSet<String> {
        &self.groups
    }
}

pub type GMLine = GMObjectManager<GMLineBase>;

impl GMLine {
    pub fn new(sprite: GMSprite) -> Self {
        Self {
            base: GMLineBase::new((0.0, 0.0), (0.0, 0.0), sprite, 0),
            effects: GMEffectManager::new(),
        }
    }
}

pub struct GMLineBuilder {
    line: GMLine,
}

impl GMLineBuilder {
    pub fn new(sprite: GMSprite) -> Self {
        Self {
            line: GMLine::new(sprite),
        }
    }

    pub fn with_start<S: Into<GMVec2D>>(mut self, start: S) -> Self {
        self.line.base.start = start.into();
        self
    }

    pub fn with_end<S: Into<GMVec2D>>(mut self, end: S) -> Self {
        self.line.base.end = end.into();
        self
    }

    pub fn with_number(mut self, number: u32) -> Self {
        self.line.base.set_number(number);
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.line.base.set_spacing(spacing);
        self
    }

    pub fn with_effect<T: 'static + GMEffectT<GMLineBase>>(mut self, effect: T) -> Self {
        debug!("GMLineBuilder::with_effect()");

        self.line.effects.add_effect(effect);
        self
    }

    pub fn with_effect2(mut self, effect: Box<dyn GMEffectT<GMLineBase>>) -> Self {
        debug!("GMLineBuilder::with_effect2()");

        self.line.effects.add_effect2(effect);
        self
    }

    pub fn with_effects(mut self, effects: Vec<Box<dyn GMEffectT<GMLineBase>>>) -> Self {
        debug!("GMLineBuilder::with_effects()");

        self.line.effects.set_effects(effects);
        self
    }

    pub fn build(self) -> GMLine {
        self.line
    }
}
