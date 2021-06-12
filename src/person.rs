
use crate::sprite::GMSprite;

pub trait GMHealthBarT {
    fn draw(&self, sprite: &GMSprite, health: u32);
    fn update(&mut self);
}

pub struct GMPerson {
    sprite: GMSprite,
    health: u32,
    health_bar: Box<dyn GMHealthBarT>,
}

impl GMPerson {
    pub fn new(sprite: GMSprite, health_bar: Box<dyn GMHealthBarT>) -> Self {
        Self {
            sprite,
            health: 0,
            health_bar,
        }
    }
    pub fn set_health(&mut self, health: u32) {
        self.health = health;
    }
    pub fn get_health(&self) -> u32 {
        self.health
    }
    pub fn inc_health(&mut self, inc: u32) {
        self.health += inc;
    }
    pub fn dec_health(&mut self, dec: u32) {
        self.health -= dec;
    }
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }
    pub fn draw(&self) {
        self.sprite.draw();
        self.health_bar.draw(&self.sprite, self.health);
    }
    pub fn update(&mut self) {
        self.sprite.update();
        self.health_bar.update();
    }
}
