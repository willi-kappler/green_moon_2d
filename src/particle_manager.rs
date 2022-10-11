

use std::collections::HashSet;

use log::debug;

use crate::sprite::GMSprite;
use crate::timer::GMTimer;
use crate::math::GMVec2D;
use crate::effect::{GMEffectManager, GMEffectT};
use crate::context::{GMContext, GMObjectMessage};
use crate::data::GMData;
use crate::util::error_panic;


pub struct GMParticleManagerBase {
    pub max_num_of_particles: usize,
    pub particle_sprite: GMSprite,
    pub life_time: f32,
    pub particles: Vec<(GMTimer, GMSprite)>,
    pub active: bool,
    pub visible: bool,
    pub position: GMVec2D,
    pub name: String,
    pub groups: HashSet<String>,
}

impl GMParticleManagerBase {
    pub fn new(max_num_of_particles: usize, particle_sprite: GMSprite, life_time: f32, position: GMVec2D) -> Self {
        debug!("GMParticleManagerBase::new(), max_num_of_particles: '{}', life_time: '{}', position: '{:?}'",
            max_num_of_particles, life_time, position);

        let mut particles = Vec::with_capacity(max_num_of_particles);

        for _ in 0..max_num_of_particles {
            let mut sprite = particle_sprite.clone();
            sprite.base.position = position;

            particles.push((GMTimer::new(life_time), sprite));
        }

        Self {
            max_num_of_particles,
            particle_sprite,
            life_time,
            particles,
            active: true,
            visible: true,
            position,
            name: "".to_string(),
            groups: HashSet::new(),
        }
    }

    pub fn update(&mut self, context: &mut GMContext) {
        if self.active {
            for (_, sprite) in self.particles.iter_mut() {
                sprite.update(context);
            }
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        if self.visible {
            for (_, sprite) in self.particles.iter() {
                sprite.draw(context);
            }
        }
    }

    pub fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_position" => {
                self.position = data.into();
            }
            "set_visible" => {
                self.visible = data.into();
            }
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSpriteBase::send_message(), unknown message: '{}'", message))
            }
        }
    }

    pub fn send_message2(&mut self, message: &str, context: &mut GMContext) {
        self.send_message(message, GMData::None, context);
    }
}

pub struct GMParticleManager {
    pub base: GMParticleManagerBase,
    pub effects: GMEffectManager<GMParticleManagerBase>,
}

impl GMParticleManager {
    pub fn new(max_num_of_particles: usize, particle_sprite: GMSprite, life_time: f32, position: GMVec2D) -> Self {
        Self {
            base: GMParticleManagerBase::new(max_num_of_particles, particle_sprite, life_time, position),
            effects: GMEffectManager::new(),
        }
    }

    pub fn update(&mut self, context: &mut GMContext) {
        self.base.update(context);
        self.effects.update(&mut self.base, context);
    }

    pub fn draw(&self, context: &mut GMContext) {
        self.base.draw(context);
        self.effects.draw(&self.base, context);
    }

    pub fn check_messages(&mut self, context: &mut GMContext) {
        let mut messages = context.get_object_messages(&self.base.name);

        while let Some(message) = messages.pop_front() {
            match message {
                GMObjectMessage::Base(message, data) => {
                    self.base.send_message(&message, data, context);
                }
                GMObjectMessage::Effect(index, message, data) => {
                    self.effects.send_message(index, &message, data, context);
                }
            }
        }

        let mut messages = context.get_group_messages(&self.base.groups);

        while let Some(message) = messages.pop_front() {
            match message {
                GMObjectMessage::Base(message, data) => {
                    self.base.send_message(&message, data, context);
                }
                GMObjectMessage::Effect(index, message, data) => {
                    self.effects.send_message(index, &message, data, context);
                }
            }
        }
    }
}

pub struct GMParticleManagerBuilder {

}

impl GMParticleManagerBuilder {
    pub fn new() -> Self {
        Self {

        }
    }
}
