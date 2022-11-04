
use crate::math::GMVec2D;
use crate::sprite::GMSprite;
use crate::context::GMContext;

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

    pub fn update(&mut self, context: &mut GMContext) {
        for sprite in self.sprites.iter_mut() {
            sprite.update(context);
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        for sprite in self.sprites.iter() {
            sprite.draw(context);
        }
    }
}
