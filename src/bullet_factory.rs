use crate::animation::GMAnimationT;
use crate::resource_manager::GMResourceManager;
use crate::sprite::GMSprite;

use macroquad::time::get_time;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GMOffscreenMode {
    Keep,
    Destroy,
    WrapAround,
}

pub enum GMBulletState {
    Moving,
    Exploding,
}

pub struct GMBulletFactory {
    base_sprite: GMSprite,
    explosion: Box<dyn GMAnimationT>,
    max_bullets: usize,
    delay: f64,
    prev_time: f64,
    offscreen_mode: GMOffscreenMode,
    bullets: Vec<GMSprite>,
}

impl GMBulletFactory {
    pub fn new(sprite: &GMSprite, explosion: Box<dyn GMAnimationT>, max_bullets: usize) -> Self {
        Self {
            base_sprite: sprite.clone_sprite(),
            explosion,
            max_bullets,
            delay: 0.5,
            prev_time: 0.0,
            offscreen_mode: GMOffscreenMode::Destroy,
            bullets: Vec::new(),
        }
    }
    pub fn new_from_resource(resources: &GMResourceManager, sprite_name: &str, explosion_name: &str, max_bullets: usize) -> Self {
        let sprite = resources.get_sprite(sprite_name).unwrap();
        let explosion = resources.get_animation(explosion_name).unwrap().clone_animation();

        Self::new(sprite, explosion, max_bullets)
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
            let mut sprite = self.base_sprite.clone_sprite();
            sprite.set_x(x);
            sprite.set_y(y);
            sprite.set_vx(vx);
            sprite.set_vy(vy);
            sprite.start_animation();
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

            if bullet.get_state_id() == 0 {
                match self.offscreen_mode {
                    Keep => {
                        // Nothing to do, just keep moving the bullet...
                    }
                    Destroy => {
                        if bullet.is_offscreen() {
                            bullet.set_active(false);
                        }
                    }
                    WrapAround => {
                        bullet.wrap_around();
                    }
                }
            } else {
                if bullet.animation_finished() {
                    bullet.set_active(false);
                }
            }
        }

        self.bullets.retain(|bullet| bullet.get_active() );
    }
    pub fn collides_single(&mut self, other: &GMSprite) -> bool {
        let mut result = false;

        for bullet in self.bullets.iter_mut() {
            if bullet.get_state_id() == 0 {
                if other.collides_with(bullet) {
                    result = true;
                    bullet.set_vx(0.0);
                    bullet.set_vy(0.0);
                    bullet.set_state_id(1);
                    bullet.set_animation(self.explosion.clone_animation());
                }
            }
        }
        result
    }
}
