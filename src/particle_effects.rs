

// use std::f32::consts::{PI};

// use log::debug;

/*

use crate::context::GMContext;
use crate::particle_manager::{GMParticleManagerBase, GMParticleState};
use crate::effect::GMEffectT;
use crate::data::GMData;
use crate::timer::GMTimer;
use crate::util::{error_panic, random_range_f32};

pub type GMBoxParticleEffect = Box<dyn GMEffectT<GMParticleManagerBase>>;


// TODO: Refactor, lots of duplicated code

#[derive(Debug, Clone)]
pub struct GMPESimple {
    pub messages: Vec<(usize, String, GMData)>,
    pub active: bool,
}

impl GMPESimple {
    pub fn new<S: Into<String>>(mut messages: Vec<(usize, S, GMData)>) -> Self {
        Self {
            messages: messages.drain(0..).map(|(i, s, d)| (i, s.into(), d)).collect(),
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
                                // debug!("GMPESimple::update(), Waiting, timer finished");
                                let run_time = random_range_f32(base.run_time.0, base.run_time.1);
                                timer.set_duration(run_time);
                                timer.start();

                                *state = GMParticleState::Running;

                                sprite.base.position = base.position;

                                for (index, message, data) in self.messages.iter() {
                                    sprite.effects.send_message(*index, message, data.clone(), context)
                                }
                            }
                        }
                        GMParticleState::Running => {
                            if timer.finished() {
                                // debug!("GMPESimple::update(), Running, timer finished");
                                let wait_time = random_range_f32(base.wait_time.0, base.wait_time.1);
                                timer.set_duration(wait_time);
                                timer.start();

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
            "add_message" => {
                todo!()
            }
            "remove_message" => {
                todo!()
            }
            _ => {
                error_panic(&format!("GMPESimple::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMParticleManagerBase>> {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "active" => {
                self.active.into()
            }
            "message" => {
                todo!()
            }
            "messages" => {
                todo!()
            }
            _ => {
                error_panic(&format!("GMPESimple::get_property(), unknown property: '{}'", name))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMPESpiral {
    pub messages: Vec<(usize, String, GMData)>,
    pub active: bool,
    pub angle: f32,
    pub angle_speed: f32,
    pub particle_speed: f32,
    pub wait_time: f32,
    timer: GMTimer,
}

impl GMPESpiral {
    pub fn new<S: Into<String>>(angle_speed: f32, particle_speed: f32, mut messages: Vec<(usize, S, GMData)>) -> Self {
        let mut result = GMPESpiral::new2(angle_speed, particle_speed);
        result.messages = messages.drain(0..).map(|(i, s, d)| (i, s.into(), d)).collect();

        result
    }

    pub fn new2(angle_speed: f32, particle_speed: f32) -> Self {
        Self {
            messages: Vec::new(),
            active: true,
            angle: 0.0,
            angle_speed,
            particle_speed,
            wait_time: 0.1,
            timer: GMTimer::new(1.0),
        }
    }

    pub fn set_wait_time(&mut self, wait_time: f32) {
        self.wait_time = wait_time;
        self.timer.set_duration(wait_time);
        self.timer.start();
    }
}

impl GMEffectT<GMParticleManagerBase> for GMPESpiral {
    fn update(&mut self, base: &mut GMParticleManagerBase, context: &mut GMContext) {
        if self.active && base.active {
            for (state, timer, sprite) in base.particles.iter_mut() {
                if sprite.base.active {
                    match state {
                        GMParticleState::Waiting => {
                            sprite.base.position = base.position;

                            for (index, message, data) in self.messages.iter() {
                                sprite.effects.send_message(*index, message, data.clone(), context)
                            }

                            if self.timer.finished() {
                                self.timer.set_duration(self.wait_time);
                                self.timer.start();

                                let rad = self.angle * PI / 180.0;
                                let vx: f32 = rad.cos() * self.particle_speed;
                                let vy: f32 = rad.sin() * self.particle_speed;

                                sprite.effects.send_message(0, "set_velocity", (vx, vy).into(), context);

                                self.angle += self.angle_speed;

                                if self.angle > 360.0 {
                                    self.angle -= 360.0;
                                }

                                let run_time = random_range_f32(base.run_time.0, base.run_time.1);
                                timer.set_duration(run_time);
                                timer.start();

                                *state = GMParticleState::Running;
                            }
                        }
                        GMParticleState::Running => {
                            if timer.finished() {
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
            for (_, _, sprite) in base.particles.iter() {
                sprite.draw(context);
            }
        }
    }


    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_active" => {
                self.active = data.into();
            }
            "set_angle" => {
                self.angle = data.into();
            }
            "set_angle_speed" => {
                self.angle_speed = data.into();
            }
            "set_particle_speed" => {
                self.particle_speed = data.into();
            }
            "set_wait_time" => {
                self.wait_time = data.into();
            }
            _ => {
                error_panic(&format!("GMPESpiral::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMParticleManagerBase>> {
        Box::new(self.clone())
    }

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "angle" => {
                self.angle.into()
            }
            "angle_speed" => {
                self.angle_speed.into()
            }
            "particle_speed" => {
                self.particle_speed.into()
            }
            "wait_time" => {
                self.wait_time.into()
            }
            _ => {
                error_panic(&format!("GMPESpiral::get_property(), unknown property: '{}'", name))
            }
        }
    }
}
*/
