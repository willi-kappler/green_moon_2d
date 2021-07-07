
use crate::sprite::GMSprite;
use crate::health_bar::GMHealthBar;


pub struct GMPerson {
    sprite: GMSprite,
    health_bar: GMHealthBar,
}

impl GMPerson {
    pub fn new(sprite: &GMSprite, health_bar: GMHealthBar) -> Self {
        Self {
            sprite: sprite.clone(),
            health_bar,
        }
    }
    pub fn set_health(&mut self, health: u32) {
        self.health_bar.set_health(health);
    }
    pub fn get_health(&self) -> u32 {
        self.health_bar.get_health()
    }
    pub fn inc_health(&mut self, inc: u32) {
        self.health_bar.inc_health(inc);
    }
    pub fn dec_health(&mut self, dec: u32) {
        self.health_bar.dec_health(dec);
    }
    pub fn is_dead(&self) -> bool {
        self.health_bar.is_dead()
    }
    pub fn draw(&self) {
        self.sprite.draw();
        self.health_bar.draw();
    }
    pub fn update(&mut self) {
        self.sprite.update();
        self.health_bar.update();
    }
}
