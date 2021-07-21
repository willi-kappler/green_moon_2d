use crate::sprite::GMSpriteSimple;
use crate::text::GMText;

pub trait GMHealthBarT {
    fn draw(&self);
    fn update(&mut self);
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, x: f32);
    fn get_extend(&self) -> (f32, f32);
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
    pub fn get_extend(&self) -> (f32, f32) {
        self.health_bar.get_extend()
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
    fn get_extend(&self) -> (f32, f32);
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
    pub fn get_extend(&self) -> (f32, f32) {
        self.label.get_extend()
    }
}

pub struct GMLabelText {
    text: GMText,
}

impl GMLabelText {
    pub fn new(text: GMText) -> Self {
        Self {
            text,
        }
    }
}

impl GMLabelT for GMLabelText {
    fn draw(&self) {
        self.text.draw();
    }
    fn update(&mut self) {
        self.text.update();
    }
    fn set_x(&mut self, x: f32) {
        self.text.set_x(x);
    }
    fn set_y(&mut self, y: f32) {
        self.text.set_y(y);
    }
    fn get_x(&self) -> f32 {
        self.text.get_x()
    }
    fn get_y(&self) -> f32 {
        self.text.get_y()
    }
    fn get_extend(&self) -> (f32, f32) {
        self.text.get_extend()
    }
}

pub struct GMLabelSprite {
    sprite: GMSpriteSimple,
}

impl GMLabelSprite {
    pub fn new(sprite: &GMSpriteSimple) -> Self {
        Self {
            sprite: sprite.clone(),
        }
    }
}

impl GMLabelT for GMLabelSprite {
    fn draw(&self) {
        self.sprite.draw();
    }
    fn update(&mut self) {
        self.sprite.update();
    }
    fn set_x(&mut self, x: f32) {
        self.sprite.set_x(x);
    }
    fn set_y(&mut self, y: f32) {
        self.sprite.set_y(y);
    }
    fn get_x(&self) -> f32 {
        self.sprite.get_x()
    }
    fn get_y(&self) -> f32 {
        self.sprite.get_y()
    }
    fn get_extend(&self) -> (f32, f32) {
        self.sprite.get_extend()
    }
}

pub trait GMBarT {
    fn draw(&self);
    fn update(&mut self);
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_extend(&self) -> (f32, f32);
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
    pub fn get_extend(&self) -> (f32, f32) {
        self.bar.get_extend()
    }
    pub fn set_value(&mut self, value: u32) {
        self.bar.set_value(value);
    }
}

pub struct GMBarText {
    text: GMText,
}

impl GMBarText {
    pub fn new(text: GMText) -> Self {
        Self {
            text,
        }
    }
}

impl GMBarT for GMBarText {
    fn draw(&self) {
        self.text.draw();
    }
    fn update(&mut self) {
        self.text.update();
    }
    fn set_x(&mut self, x: f32) {
        self.text.set_x(x);
    }
    fn set_y(&mut self, y: f32) {
        self.text.set_y(y);
    }
    fn get_x(&self) -> f32 {
        self.text.get_x()
    }
    fn get_y(&self) -> f32 {
        self.text.get_y()
    }
    fn set_value(&mut self, value: u32) {
        let text_value = format!("{}", value);
        self.text.set_text(&text_value);
    }
    fn get_extend(&self) -> (f32, f32) {
        self.text.get_extend()
    }
}

pub struct GMBarSpriteSingle {
    sprite: GMSpriteSimple,
}

pub struct GMBarSpriteMultiple {
    sprite: GMSpriteSimple,
}

pub struct GMHealthBarSimple {
    x: f32,
    y: f32,
    health: u32,
    label: GMLabel,
    bar: GMBar,
}

impl GMHealthBarT for GMHealthBarSimple {
    fn draw(&self) {
        self.label.draw();
        self.bar.draw();
    }
    fn update(&mut self) {
        self.label.update();
        self.bar.update();
    }
    fn set_x(&mut self, x: f32) {
        self.x = x;
        self.label.set_x(x);
        let (w, _) = self.label.get_extend();
        self.bar.set_x(x + w);
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
        self.label.set_y(y);
        self.bar.set_y(y);
    }
    fn set_health(&mut self, health: u32) {
        self.health = health;
    }
    fn get_health(&self) -> u32 {
        self.health
    }
    fn inc_health(&mut self, inc: u32) {
        self.health += inc;
    }
    fn dec_health(&mut self, dec: u32) {
        if dec > self.health {
            self.health = 0;
        } else {
            self.health -= dec;
        }
    }
    fn is_dead(&self) -> bool {
        self.health == 0
    }
    fn get_extend(&self) -> (f32, f32) {
        let (w1, h1) = self.label.get_extend();
        let (w2, h2) = self.bar.get_extend();
        (w1 + w2, h1.max(h2))
    }
}
