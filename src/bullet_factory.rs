use crate::animation::GMAnimationT;
use crate::sprite::GMSprite;
use crate::spritesheet::GMSpriteSheet;

use macroquad::time::get_time;

use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GMOffscreenMode {
    Keep,
    Destroy,
    WrapAround,
}

pub struct GMBulletFactory {
    pub(crate) sprite_sheet: Rc<GMSpriteSheet>,
    pub(crate) animation: Box<dyn GMAnimationT>,
    pub(crate) max_bullets: usize,
    pub(crate) delay: f64,
    pub(crate) prev_time: f64,
    pub(crate) offscreen_mode: GMOffscreenMode,
    pub(crate) bullets: Vec<GMSprite>,
}

impl GMBulletFactory {
    pub fn new(sprite_sheet: &Rc<GMSpriteSheet>, animation: Box<dyn GMAnimationT>, max_bullets: usize) -> Self {
        Self {
            sprite_sheet: sprite_sheet.clone(),
            animation,
            max_bullets,
            delay: 0.0,
            prev_time: 0.0,
            offscreen_mode: GMOffscreenMode::Destroy,
            bullets: Vec::new(),
        }
    }
    pub fn set_delay(&mut self, delay: f64) {
        self.delay = delay;
    }
    pub fn set_offscreen_mode(&mut self, offscreen_mode: GMOffscreenMode) {
        self.offscreen_mode = offscreen_mode;
    }
    pub fn add_bullet(&mut self, x: f32, y: f32, vx: f32, vy: f32) {
        let current_time = get_time();
        if current_time - self.prev_time < self.delay {
            return
        } else {
            self.prev_time = current_time;
        }

        if self.bullets.len() < self.max_bullets {
            let mut sprite = GMSprite::new(&self.sprite_sheet, self.animation.clone_animation(), x, y);
            sprite.set_vx(vx);
            sprite.set_vy(vy);
            self.bullets.push(sprite);
        }
    }
    pub fn draw(&self) {
        for bullet in self.bullets.iter() {
            bullet.draw();
        }
    }
    pub fn update(&mut self) {
        use GMOffscreenMode::*;

        for bullet in self.bullets.iter_mut() {
            bullet.update();

            match self.offscreen_mode {
                Keep => {
                    // Nothing to do, just keep moving the bullet...
                }
                Destroy => {
                    if bullet.is_offscreen() {
                        bullet.active = false;
                    }
                }
                WrapAround => {
                    bullet.wrap_around();
                }
            }
        }

        self.bullets.retain(|bullet| bullet.active );
    }
    pub fn collides_single(&mut self, other: &GMSprite) -> bool {
        let mut result = false;

        for bullet in self.bullets.iter_mut() {
            if other.collides_with(bullet) {
                result = true;
                bullet.active = false;
            }
        }
        result
    }
}
