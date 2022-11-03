
use crate::math::GMVec2D;
use crate::sprite::GMSprite;
use crate::context::GMContext;

#[derive(Debug, Clone)]
enum GMLineMode {
    Number(u32),
    Spacing(f32),
}

#[derive(Debug, Clone)]
pub struct GMLine {
    start: GMVec2D,
    end: GMVec2D,
    pub init_sprite: GMSprite,
    sprites: Vec<GMSprite>,
    line_mode: GMLineMode,
}

impl GMLine {
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

    pub fn get_start(&self) -> GMVec2D {
        self.start
    }

    pub fn get_end(&self) -> GMVec2D {
        self.end
    }

    pub fn set_start<V: Into<GMVec2D>>(&mut self, start: V) {
        self.start = start.into();

        self.end_point_changed();
    }

    pub fn set_end<V: Into<GMVec2D>>(&mut self, end: V) {
        self.end = end.into();

        self.end_point_changed();
    }

    fn end_point_changed(&mut self) {
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

    fn set_sprites(&mut self, number: u32, spacing: f32, mut direction: GMVec2D) {
        direction.norm();

        self.sprites.clear();

        for i in 0..number {
            let pos = self.start + (direction * (spacing * (i as f32)));
            let mut new_sprite = self.init_sprite.clone();
            new_sprite.base.position = pos;
            self.sprites.push(new_sprite);
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
