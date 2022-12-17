

use hecs::Entity;

use crate::math::GMVec2D;
use crate::texture::GMSharedTexture;
use crate::animation::GMAnimation;

#[derive(Debug, Clone)]
pub enum GMLineMode {
    Number(u32),
    Spacing(f32),
}
#[derive(Debug, Clone)]
pub struct GMLine {
    pub start: GMVec2D,
    pub end: GMVec2D,
    pub texture: GMSharedTexture,
    pub animation: GMAnimation,
    pub elements: Vec<Entity>,
    pub line_mode: GMLineMode,
}

/*

use crate::math::GMVec2D;
use crate::sprite::GMSprite;
use crate::context::GMContext;
use crate::effect::{GMEffectManager, GMEffectT};
use crate::object_manager::{GMObjectBaseT, GMObjectManager};
use crate::util::error_panic;
use crate::data::GMData;

use crate::{return_name_and_groups, create_builder_methods};

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
    pub active: bool,
    pub visible: bool,
}

impl GMLineBase {
    pub fn new(sprite: GMSprite) -> Self {
        debug!("GMLineBase::new()");

        Self {
            start: GMVec2D::new(0.0, 0.0),
            end: GMVec2D::new(0.0, 0.0),
            init_sprite: sprite,
            sprites: Vec::new(),
            line_mode: GMLineMode::Number(0),
            name: "".to_string(),
            groups: HashSet::new(),
            active: true,
            visible: true,
        }
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
        if self.active {
            for sprite in self.sprites.iter_mut() {
                sprite.update(context);
            }
        }
    }

    fn draw(&self, context: &mut GMContext) {
        if self.visible {
            for sprite in self.sprites.iter() {
                sprite.draw(context);
            }
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_start" => {
                let start: GMVec2D = data.into();
                self.set_start(start);
            }
            "set_start2" => {
                self.start = data.into();
            }
            "set_end" => {
                let end: GMVec2D = data.into();
                self.set_end(end);
            }
            "set_end2" => {
                self.end = data.into();
            }
            "set_number" => {
                let number: u32 = data.into();
                self.set_number(number);
            }
            "set_number2" => {
                let number: u32 = data.into();
                self.line_mode = GMLineMode::Number(number);
            }
            "set_spacing" => {
                let spacing: f32 = data.into();
                self.set_spacing(spacing);
            }
            "set_spacing2" => {
                let spacing: f32 = data.into();
                self.line_mode = GMLineMode::Spacing(spacing);
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
                error_panic(&format!("GMLineBase::send_message(), unknown message: '{}'", message))
            }
        }
    }

    return_name_and_groups!();
}

pub type GMLine = GMObjectManager<GMLineBase>;

impl GMLine {
    pub fn new(sprite: GMSprite) -> Self {
        Self {
            base: GMLineBase::new(sprite),
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
        let start = start.into();

        debug!("GMLineBuilder::with_start(), start: '{}'", start);

        self.line.base.start = start;
        self
    }

    pub fn with_end<S: Into<GMVec2D>>(mut self, end: S) -> Self {
        let end = end.into();

        debug!("GMLineBuilder::end(), end: '{}'", end);

        self.line.base.end = end;
        self
    }

    pub fn with_number(mut self, number: u32) -> Self {
        debug!("GMLineBuilder::with_number(), number: '{}'", number);

        self.line.base.set_number(number);
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        debug!("GMLineBuilder::with_spacing(), spacing: '{}'", spacing);

        self.line.base.set_spacing(spacing);
        self
    }

    create_builder_methods!(GMLineBuilder, GMLineBase, line);

    pub fn build(self) -> GMLine {
        self.line
    }
}
*/
