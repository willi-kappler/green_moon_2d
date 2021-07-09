
pub trait GMHealthBarT {
    fn draw(&self);
    fn update(&mut self);
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, x: f32);
    fn set_health(&mut self, health: u32);
    fn get_health(&self) -> u32;
    fn inc_health(&mut self, inc: u32);
    fn dec_health(&mut self, dec: u32);
    fn is_dead(&self) -> bool;
}

pub struct GMHealthBar {
    health_bar: Box<dyn GMHealthBarT>,
}

impl GMHealthBar {
    pub fn new<T: 'static + GMHealthBarT>(health_bar: T) -> Self {
        Self {
            health_bar: Box::new(health_bar),
        }
    }
    pub fn draw(&self) {
        self.health_bar.draw();
    }
    pub fn update(&mut self) {
        self.health_bar.update();
    }
    pub fn set_x(&mut self, x: f32) {
        self.health_bar.set_x(x);
    }
    pub fn set_y(&mut self, y: f32) {
        self.health_bar.set_y(y);
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
        self.health_bar.dec_health(dec)
    }
    pub fn is_dead(&self) -> bool {
        self.health_bar.is_dead()
    }
}

pub struct GMHealthBarTextText {

}

pub struct GMHealthBarTextSprite {

}

pub struct GMHealthBarSpriteText {

}

pub struct GMHealthBarSpriteSprite {

}
