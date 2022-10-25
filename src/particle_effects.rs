
use crate::context::GMContext;
use crate::particle_manager::{GMParticleManagerBase, GMParticleState};
use crate::effect::GMEffectT;
use crate::data::GMData;
use crate::util::{error_panic, random_range_f32};

pub type GMBoxParticleEffect = Box<dyn GMEffectT<GMParticleManagerBase>>;

#[derive(Debug, Clone)]
pub struct GMPESimple {
    pub messages: Vec<(usize, String, GMData)>,
    pub active: bool,
}

impl GMPESimple {
    pub fn new(messages: Vec<(usize, String, GMData)>) -> Self {
        Self {
            messages: messages,
            active: true,
        }
    }
}

impl GMEffectT<GMParticleManagerBase> for GMPESimple {
    fn update(&mut self, base: &mut GMParticleManagerBase, context: &mut GMContext) {
        if self.active && base.active {
            for (state, timer, sprite) in base.particles.iter_mut() {
                if sprite.base.active {
                    match state {
                        GMParticleState::Waiting => {
                            if timer.finished() {
                                let run_time = random_range_f32(base.run_time.0, base.run_time.1);
                                timer.set_duration(run_time);

                                *state = GMParticleState::Running;

                                sprite.base.position = base.position;

                                for (index, message, data) in self.messages.iter() {
                                    sprite.effects.send_message(*index, message, data.clone(), context)
                                }
                            }
                        }
                        GMParticleState::Running => {
                            if timer.finished() {
                                let wait_time = random_range_f32(base.wait_time.0, base.wait_time.1);
                                timer.set_duration(wait_time);
                                *state = GMParticleState::Waiting;
                            } else {
                                sprite.update(context);
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw(&self, base: &GMParticleManagerBase, context: &mut GMContext) {
        if base.visible {
            for (state, _, sprite) in base.particles.iter() {
                match state {
                    GMParticleState::Waiting => {
                        // Nothing to do...
                    }
                    GMParticleState::Running => {
                        sprite.draw(context);
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
