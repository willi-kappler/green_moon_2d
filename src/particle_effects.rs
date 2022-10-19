
use crate::context::GMContext;
use crate::particle_manager::GMParticleManagerBase;
use crate::effect::GMEffectT;
use crate::data::GMData;
use crate::util::error_panic;

pub type GMBoxParticleEffect = Box<dyn GMEffectT<GMParticleManagerBase>>;

#[derive(Debug, Clone)]
pub struct GMPESimple {
    messages: Vec<(usize, String, GMData)>,
    active: bool,
}

impl GMPESimple {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            active: true,
        }
    }
}

impl GMEffectT<GMParticleManagerBase> for GMPESimple {
    fn update(&mut self, base: &mut GMParticleManagerBase, context: &mut GMContext) {
        if self.active && base.active {
            for (duration, sprite) in base.particles.iter_mut() {
                if sprite.base.active {
                    if duration.finished() {
                        duration.start();
                        sprite.base.position = base.position;
                        for (index, message, data) in self.messages.iter() {
                            sprite.effects.send_message(*index, message, data.clone(), context)
                        }
                    }
                }
            }
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMPESimple::send_message_data(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMParticleManagerBase>> {
        Box::new(self.clone())
    }
}
