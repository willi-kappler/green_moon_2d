

use std::collections::HashSet;

use log::debug;

use crate::sprite::GMSprite;
use crate::timer::GMTimer;
use crate::math::GMVec2D;
use crate::effect::{GMEffectManager, GMEffectT};
use crate::context::GMContext;
use crate::data::GMData;
use crate::util::{error_panic, random_range_f32};
use crate::object_manager::{GMObjectBaseT, GMObjectManager};

use crate::return_name_and_groups;

#[derive(Debug, Clone)]
pub enum GMParticleState {
    Waiting,
    Running,
}

#[derive(Debug, Clone)]
pub struct GMParticleManagerBase {
    pub max_num_of_particles: usize,
    pub particle_sprite: GMSprite,
    pub wait_time: (f32, f32),
    pub run_time: (f32, f32),
    pub particles: Vec<(GMParticleState, GMTimer, GMSprite)>,
    pub active: bool,
    pub visible: bool,
    pub position: GMVec2D,
    pub name: String,
    pub groups: HashSet<String>,
}

impl GMParticleManagerBase {
    pub fn new(particle_sprite: GMSprite) -> Self {
        debug!("GMParticleManagerBase::new()");

        Self {
            max_num_of_particles: 0,
            particle_sprite,
            wait_time: (0.0, 0.0),
            run_time: (0.0, 0.0),
            particles: Vec::new(),
            active: true,
            visible: true,
            position: GMVec2D::new(0.0, 0.0),
            name: "".to_string(),
            groups: HashSet::new(),
        }
    }

    pub fn set_max_num_of_particles(&mut self, max_num_of_particles: usize) {
        self.max_num_of_particles = max_num_of_particles;
        self.particles = Vec::with_capacity(max_num_of_particles);

        for _ in 0..max_num_of_particles {
            let wait_time = random_range_f32(self.wait_time.0, self.wait_time.1);

            self.particles.push((
                GMParticleState::Waiting,
                GMTimer::new(wait_time),
                self.particle_sprite.clone()
            ));
        }
    }

    pub fn set_position<T: Into<GMVec2D>>(&mut self, position: T) {
        self.position = position.into();
        self.particle_sprite.base.position = self.position;

        for (_, _, sprite) in self.particles.iter_mut() {
            sprite.base.position = self.position;
        }
    }
}

impl GMObjectBaseT for GMParticleManagerBase {
    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_position" => {
                let position: GMVec2D = data.into();
                self.set_position(position);
            }
            "set_wait_time" => {
                self.wait_time = data.into();
            }
            "set_run_time" => {
                self.run_time = data.into();
            }
            "set_max_number_of_particles" => {
                let max_num_of_particles: usize = data.into();
                self.set_max_num_of_particles(max_num_of_particles);
            }
            "set_visible" => {
                self.visible = data.into();
            }
            "set_active" => {
                self.active = data.into();
            }
            "set_name" => {
                self.name = data.into();
            }
            "add_group" => {
                self.groups.insert(data.into());
            }
            "remove_group" => {
                let group: String = data.into();
                self.groups.remove(&group);
            }
            "clear_group" => {
                self.groups.clear();
            }
            _ => {
                error_panic(&format!("GMParticleBase::send_message(), unknown message: '{}'", message))
            }
        }
    }

    return_name_and_groups!();
}

pub type GMParticleManager = GMObjectManager<GMParticleManagerBase>;

impl GMParticleManager {
    pub fn new(particle_sprite: GMSprite) -> Self {
        Self {
            base: GMParticleManagerBase::new(particle_sprite),
            effects: GMEffectManager::new(),
        }
    }
}

pub struct GMParticleManagerBuilder {
    particle_manager: GMParticleManager,
}

impl GMParticleManagerBuilder {
    pub fn new(particle_sprite: GMSprite) -> Self {
        Self {
            particle_manager: GMParticleManager::new(particle_sprite),
        }
    }

    pub fn with_wait_time(mut self, min: f32, max: f32) -> Self {
        debug!("GMParticleManager::with_wait_time(), min: '{}', max: '{}'", min, max);

        self.particle_manager.base.wait_time = (min, max);

        self
    }

    pub fn with_run_time(mut self, min: f32, max: f32) -> Self {
        debug!("GMParticleManager::with_run_time(), min: '{}', max: '{}'", min, max);

        self.particle_manager.base.run_time = (min, max);

        self
    }

    pub fn with_max_num_of_particles(mut self, max_num_of_particles: usize) -> Self {
        debug!("GMParticleManager::with_max_num_of_particles(), max_num_of_particles: {}", max_num_of_particles);

        self.particle_manager.base.set_max_num_of_particles(max_num_of_particles);

        self
    }

    pub fn with_position<T: Into<GMVec2D>>(mut self, position: T) -> Self {
        let position = position.into();

        debug!("GMParticleManagerBuilder::with_position(), position: '{:?}'", position);

        self.particle_manager.base.set_position(position);

        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        debug!("GMParticleManagerBuilder::with_visible(), visible: '{}'", visible);

        self.particle_manager.base.visible = visible;
        self
    }


    pub fn with_active(mut self, active: bool) -> Self {
        debug!("GMParticleManagerBuilder::with_active(), active: '{}'", active);

        self.particle_manager.base.active = active;
        self
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        let name = name.into();

        debug!("GMParticleManagerBuilder::with_name(), name: '{}'", name);

        self.particle_manager.base.name = name;
        self
    }

    pub fn with_group<S: Into<String>>(mut self, group: S) -> Self {
        let group = group.into();

        debug!("GMParticleManagerBuilder::with_group(), group: '{}'", group);

        self.particle_manager.base.groups.insert(group);
        self
    }

    pub fn with_groups(mut self, groups: HashSet<String>) -> Self {
        debug!("GMParticleManagerBuilder::with_groups(), groups: '{:?}'", groups);

        self.particle_manager.base.groups = groups;
        self
    }

    pub fn with_effect<T: 'static + GMEffectT<GMParticleManagerBase>>(mut self, effect: T) -> Self {
        debug!("GMParticleManagerBuilder::with_effect()");

        self.particle_manager.effects.add_effect(effect);
        self
    }

    pub fn with_effect2(mut self, effect: Box<dyn GMEffectT<GMParticleManagerBase>>) -> Self {
        debug!("GMParticleManagerBuilder::with_effect2()");

        self.particle_manager.effects.add_effect2(effect);
        self
    }

    pub fn with_effects(mut self, effects: Vec<Box<dyn GMEffectT<GMParticleManagerBase>>>) -> Self {
        debug!("GMParticleManagerBuilder::with_effects()");

        self.particle_manager.effects.set_effects(effects);
        self
    }

    pub fn build(self) -> GMParticleManager {
        self.particle_manager
    }
}
