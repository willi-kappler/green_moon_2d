use crate::resources::GMResourceManager;
use crate::sprite::GMSprite;
use crate::sound::GMSound;

use macroquad::time::get_time;

use std::rc::Rc;

// TODO:
// - add bullet duration / lifetime

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GMOffscreenMode {
    Keep,
    Destroy,
    WrapAround,
}

pub struct GMBulletManager {
    base_sprite: GMSprite,
    max_bullets: usize,
    delay: f64,
    prev_time: f64,
    offscreen_mode: GMOffscreenMode,
    bullets: Vec<GMSprite>,
    shoot_sound: Rc<GMSound>,

}

impl GMBulletManager {
    pub fn new(sprite: &GMSprite, max_bullets: usize, shoot_sound: &Rc<GMSound>) -> Self {
        Self {
            base_sprite: sprite.clone(),
            max_bullets,
            delay: 0.5,
            prev_time: 0.0,
            offscreen_mode: GMOffscreenMode::Destroy,
            bullets: Vec::new(),
            shoot_sound: shoot_sound.clone(),
        }
    }
    pub fn new_from_resource(resources: &GMResourceManager, sprite_name: &str, max_bullets: usize, shoot_sound_name: &str) -> Self {
        let sprite = resources.get_sprite(sprite_name).unwrap();
        let shoot_sound = resources.get_sound(shoot_sound_name).unwrap();

        Self::new(sprite, max_bullets, &shoot_sound)
    }
    pub fn set_delay(&mut self, delay: f64) {
        self.delay = delay;
    }
    pub fn set_offscreen_mode(&mut self, offscreen_mode: GMOffscreenMode) {
        self.offscreen_mode = offscreen_mode;
    }
    pub fn add_bullet(&mut self, x: f32, y: f32, vx: f32, vy: f32, rotation: f32, mid: bool) {
        // TODO: Reuse inactive bullets
        let current_time = get_time();
        if current_time - self.prev_time < self.delay {
            return
        } else {
            self.prev_time = current_time;
        }

        if self.bullets.len() < self.max_bullets {
            self.shoot_sound.play();

            let mut sprite = self.base_sprite.clone();
            if mid {
                sprite.set_mid_x(x);
                sprite.set_mid_y(y);
            } else {
                sprite.set_x(x);
                sprite.set_y(y);
            }
            sprite.set_vx(vx);
            sprite.set_vy(vy);
            sprite.set_rotation(rotation);
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
        }

        self.bullets.retain(|bullet| bullet.get_active() );
    }
    pub fn collides_single(&mut self, other: &GMSprite) -> bool {
        let mut result = false;

        for bullet in self.bullets.iter_mut() {
            if other.collides_with(bullet) {
                result = true;
                bullet.set_active(false);
            }
        }
        result
    }
}
