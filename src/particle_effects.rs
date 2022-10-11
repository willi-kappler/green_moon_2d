
use crate::context::GMContext;
use crate::particle_manager::GMParticleManagerBase;
use crate::effect::GMEffectT;
use crate::data::GMData;
use crate::util::error_panic;

pub type GMBoxParticleEffect = Box<dyn GMEffectT<GMParticleManagerBase>>;

#[derive(Debug, Clone)]
pub struct GMPECircular {
    pub min_angle: f32,
    pub max_angle: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub active: bool,
}

impl GMPECircular {
    pub fn new() -> Self {
        todo!();
    }
}

impl GMEffectT<GMParticleManagerBase> for GMPECircular {
    fn update(&mut self, particle_manager: &mut GMParticleManagerBase, _context: &mut GMContext) {
        if self.active {
            todo!();
        }
    }

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_active" => {
                self.active = data.into();
            }
            _ => {
                error_panic(&format!("GMSELinearMovement::send_message_data(), unknown message: '{}'", message))
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
