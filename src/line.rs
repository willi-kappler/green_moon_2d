

use crate::math::GMVec2D;
use crate::sprite::GMSprite;
use crate::util::{GMActiveT, GMVisibleT, GMDrawT, GMUpdateT};
use crate::context::GMContext;
use crate::movement::GMPositionT;

use crate::{gen_impl_active, gen_impl_visible};

#[derive(Debug, Clone)]
pub enum GMLineMode {
    Number(u32),
    Spacing(f32),
}

#[derive(Debug, Clone)]
pub struct GMLine {
    pub start: GMVec2D,
    pub end: GMVec2D,
    pub init_sprite: GMSprite,
    pub sprites: Vec<GMSprite>,
    pub line_mode: GMLineMode,
    pub active: bool,
    pub visible: bool,
}

impl GMLine {
    pub fn new(start: GMVec2D, end: GMVec2D, init_sprite: GMSprite, line_mode: GMLineMode) -> Self {
        let mut line = Self {
            start,
            end,
            init_sprite,
            sprites: Vec::new(),
            line_mode,
            active: true,
            visible: true,
        };

        line.end_point_changed();

        line
    }

    pub fn set_start<V: Into<GMVec2D>>(&mut self, start: V) {
        self.start = start.into();
    }

    pub fn set_end<V: Into<GMVec2D>>(&mut self, end: V) {
        self.end = end.into();
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
            let sprite = &mut self.sprites[i];

            if i <= (number as usize) {
                let new_position = self.start + (direction * (spacing * (i as f32)));
                sprite.set_position(new_position);
                sprite.set_active(true);
                sprite.set_visible(true);
            } else {
                sprite.set_active(false);
                sprite.set_visible(false);
            }
        }
    }

    pub fn get_sprites(&self) -> &Vec<GMSprite> {
        &self.sprites
    }

    pub fn get_sprites_mut(&mut self) -> &mut Vec<GMSprite> {
        &mut self.sprites
    }
}

gen_impl_active!(GMLine);

gen_impl_visible!(GMLine);

impl GMUpdateT for GMLine {
    fn update(&mut self) {
        if self.active {
            for sprite in &mut self.sprites {
                sprite.update();
            }
        }
    }

    fn update2(&mut self, _context: &mut GMContext) {
        self.update()
    }
}

impl GMDrawT for GMLine {
    fn draw(&self, context: &mut GMContext) {
        if self.visible {
            for sprite in &self.sprites {
                sprite.draw(context);
            }
        }
    }
}
