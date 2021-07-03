use crate::resource_manager::GMResourceManager;
use crate::sprite::GMSprite;

use macroquad::time::get_time;
use macroquad::rand::gen_range;

use std::f32::consts;

pub struct GMParticle {
    sprite: GMSprite,
    time: f64,
}

impl GMParticle {
    pub fn new(sprite: GMSprite, time: f64) -> Self {
        Self {
            sprite,
            time,
        }
    }
    pub fn draw(&self) {
        self.sprite.draw();
    }
    pub fn update(&mut self) {
        self.sprite.update();
    }
}

pub struct GMParticleEmitter {
    sprite: GMSprite,
    x: f32,
    y: f32,
    active: bool,
    speed_min: f32,
    speed_max: f32,
    rotation_speed_min: f32,
    rotation_speed_max: f32,
    duration: f64,
    delay: f64,
    delay_t: f64,
    particles: Vec<GMParticle>,
}

// TODO: Set maximum number of particles
// Instead of deleting a particle, reuse it.

impl GMParticleEmitter {
    pub fn new(sprite: &GMSprite, x: f32, y: f32) -> Self {
        Self {
            sprite: sprite.clone(),
            x,
            y,
            active: false,
            speed_min: 1.0,
            speed_max: 5.0,
            rotation_speed_min: 0.0,
            rotation_speed_max: 0.0,
            duration: 5.0,
            delay: 0.1,
            delay_t: 0.0,
            particles: Vec::new(),
        }
    }
    pub fn new_from_resource(resources: &GMResourceManager, sprite_name: &str, x: f32, y: f32) -> Self {
        let sprite = resources.get_sprite(sprite_name).unwrap();
        Self::new(sprite, x, y)
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn set_speed_min(&mut self, min: f32) {
        self.speed_min = min;
    }
    pub fn set_speed_max(&mut self, max: f32) {
        self.speed_max = max;
    }
    pub fn set_rot_speed_min(&mut self, min: f32) {
        self.rotation_speed_min = min;
    }
    pub fn set_rot_speed_max(&mut self, max: f32) {
        self.rotation_speed_max = max;
    }
    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration;
    }
    pub fn set_delay(&mut self, delay: f64) {
        self.delay = delay;
    }
    pub fn draw(&self) {
        if self.active {
            for p in self.particles.iter() {
                p.draw();
            }
        }
    }
    pub fn update(&mut self) {
        if self.active {
            for p in self.particles.iter_mut() {
                p.update();
            }

            let time = get_time();
            let duration = self.duration;

            self.particles.retain(|p| time - p.time < duration);

            if time - self.delay_t > self.delay {
                self.delay_t = time;

                let mut new_sprite = self.sprite.clone();
                new_sprite.set_x(self.x);
                new_sprite.set_y(self.y);
                new_sprite.set_active(true);
                new_sprite.start_animation();

                let move_speed = gen_range(self.speed_min, self.speed_max);
                let direction = gen_range(0.0, consts::TAU);
                let vx = move_speed * direction.sin();
                let vy = move_speed * direction.cos();
                new_sprite.set_vx(vx);
                new_sprite.set_vy(vy);

                let rot_speed = gen_range(self.rotation_speed_min, self.rotation_speed_max);
                new_sprite.set_rot_speed(rot_speed);

                let new_particle = GMParticle::new(new_sprite, time);

                self.particles.push(new_particle);
            }
        }
    }
}
