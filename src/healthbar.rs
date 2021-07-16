use crate::animation::GMAnimation;
use crate::spritesheet::GMSpriteSheet;
use crate::text::GMText;


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

pub trait GMLabelT {
    fn draw(&self);
    fn update(&mut self);
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
}

pub struct GMLabel {
    label: Box<dyn GMLabelT>,
}

impl GMLabel {
    pub fn draw(&self) {
        self.label.draw();
    }
    pub fn update(&mut self) {
        self.label.update();
    }
    pub fn set_x(&mut self, x: f32) {
        self.label.set_x(x);
    }
    pub fn set_y(&mut self, y: f32) {
        self.label.set_y(y);
    }
    pub fn get_x(&self) -> f32 {
        self.label.get_x()
    }
    pub fn get_y(&self) -> f32 {
        self.label.get_y()
    }
}

pub struct GMLabelText {
    text: GMText,
}

pub struct GMLabelSprite {
    x: f32,
    y: f32,
    sprite_sheet: GMSpriteSheet,
    animation: GMAnimation,
}

pub trait GMBarT {
    fn draw(&self);
    fn update(&mut self);
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn set_value(&mut self, value: u32);
}

pub struct GMBar {
    bar: Box<dyn GMBarT>,
}

impl GMBar {
    pub fn draw(&self) {
        self.bar.draw();
    }
    pub fn update(&mut self) {
        self.bar.update();
    }
    pub fn set_x(&mut self, x: f32) {
        self.bar.set_x(x);
    }
    pub fn set_y(&mut self, y: f32) {
        self.bar.set_y(y);
    }
    pub fn get_x(&self) -> f32 {
        self.bar.get_x()
    }
    pub fn get_y(&self) -> f32 {
        self.bar.get_y()
    }
    pub fn set_value(&mut self, value: u32) {
        self.bar.set_value(value);
    }
}


pub struct GMHealthBarSimple {
    x: f32,
    y: f32,
    health: u32,
    label: GMLabel,
    bar: GMBar,
}
