
use crate::math::GMVec2D;
use crate::sprite::GMSprite;
use crate::context::GMContext;

pub struct GMLine {
    start: GMVec2D,
    end: GMVec2D,
    pub init_sprite: GMSprite,
    sprites: Vec<GMSprite>,
    number_set: bool,
}

impl GMLine {
    fn new_intern(start: GMVec2D, end: GMVec2D, sprite: GMSprite, direction: GMVec2D, number: u32, spacing: f32) -> Self {
        let mut sprites = Vec::new();

        for i in 0..number {
            let pos = start + (direction * (spacing * (i as f32)));
            let mut new_sprite = sprite.clone();
            new_sprite.base.position = pos;
            sprites.push(new_sprite);
        }

        Self {
            start,
            end,
            init_sprite: sprite,
            sprites,
            number_set: false,
        }
    }

    pub fn new<S: Into<GMVec2D>, E: Into<GMVec2D>>(start: S, end: E, sprite: GMSprite, number: u32) -> Self {
        let start = start.into();
        let end = end.into();
        let mut direction = end - start;
        let length = direction.len();
        let spacing = length / (number as f32);
        direction.norm();

        Self::new_intern(start, end, sprite, direction, number, spacing)
    }

    pub fn new2<S: Into<GMVec2D>, E: Into<GMVec2D>>(start: S, end: E, sprite: GMSprite, spacing: f32) -> Self {
        let start = start.into();
        let end = end.into();
        let mut direction = end - start;
        let length = direction.len();
        let number = (length / spacing).floor() as u32;
        direction.norm();

        Self::new_intern(start, end, sprite, direction, number, spacing)
    }

    pub fn set_start(&mut self, start: GMVec2D) {

    }

    pub fn set_end(&mut self, end: GMVec2D) {

    }

    pub fn set_number(&mut self, number: u32) {

    }

    pub fn set_spacing(&mut self, spacing: f32) {

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
