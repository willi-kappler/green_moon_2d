

use std::collections::HashSet;

use log::debug;

use crate::sprite::GMSprite;
use crate::timer::GMTimer;
use crate::math::GMVec2D;
use crate::effect::{GMEffectManager, GMEffectT};
use crate::context::GMContext;
use crate::data::GMData;
use crate::util::error_panic;
use crate::object_manager::{GMObjectBaseT, GMObjectManager};


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
    pub fn new(particle_sprite: GMSprite) -> Self {
        debug!("GMParticleManagerBase::new()");

        Self {
            max_num_of_particles: 0,
            particle_sprite,
            life_time: 0.0,
            particles: Vec::new(),
            active: true,
            visible: true,
            position: GMVec2D::new(0.0, 0.0),
            name: "".to_string(),
            groups: HashSet::new(),
        }
    }

}

impl GMObjectBaseT for GMParticleManagerBase {
    fn update(&mut self, context: &mut GMContext) {
        if self.active {
            for (_, sprite) in self.particles.iter_mut() {
                sprite.update(context);
            }
        }
    }

    fn draw(&self, context: &mut GMContext) {
        if self.visible {
            for (_, sprite) in self.particles.iter() {
                sprite.draw(context);
            }
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
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

    fn name(&self) -> &str {
        &self.name
    }

    fn groups(&self) -> &HashSet<String> {
        &self.groups
    }
}


pub type GMParticleManager = GMObjectManager<GMParticleManagerBase>;

impl GMParticleManager {
    pub fn new(particle_sprite: GMSprite) -> Self {
        debug!("GMParticleManager::new()");

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

    pub fn with_life_time(mut self, life_time: f32) -> Self {
        debug!("GMParticleManager::with_life_time(), life_time: {}", life_time);
        let base = &mut self.particle_manager.base;
        base.life_time = life_time;

        for (duration, _) in base.particles.iter_mut() {
            duration.set_duration(life_time);
        }

        self
    }

    pub fn with_max_num_of_particles(mut self, max_num_of_particles: usize) -> Self {
        debug!("GMParticleManager::with_max_num_of_particles(), max_num_of_particles: {}", max_num_of_particles);
        let base = &mut self.particle_manager.base;
        base.max_num_of_particles = max_num_of_particles;
        base.particles = Vec::with_capacity(max_num_of_particles);

        for _ in 0..max_num_of_particles {
            base.particles.push((
                GMTimer::new(base.life_time), base.particle_sprite.clone()
            ));
        }

        self
    }

    pub fn with_position<T: Into<GMVec2D>>(mut self, position: T) -> Self {
        let position = position.into();
        debug!("GMParticleManagerBuilder::with_position(), position: '{:?}'", position);
        let base = &mut self.particle_manager.base;
        base.position = position;
        base.particle_sprite.base.position = position;

        for (_, sprite) in base.particles.iter_mut() {
            sprite.base.position = position;
        }

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


}
