
use std::fmt::Debug;

use log::debug;

use crate::timer::GMTimer;
use crate::util::GMRepetition;
use crate::context::GMContext;
use crate::animation_effect::GMAnimationEffectT;

#[derive(Clone, Debug)]
pub struct GMAnimationBase {
    active: bool,
    current_frame: usize,
    frames: Vec<(u32, f32)>, // index, duration in seconds
    timer: GMTimer,
    repetition: GMRepetition,
}

impl GMAnimationBase {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        debug!("GMAnimationBase::new(), frames: '{:?}'", frames);

        Self {
            active: true,
            current_frame: 0,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
            repetition: GMRepetition::OnceForward,
        }
    }

    // Other methods
    pub fn texture_index(&self) -> u32 {
        self.frames[self.current_frame].0
    }

    pub fn active(&mut self) -> bool{
        self.active
    }

    pub fn active_mut(&mut self) -> &mut bool {
        &mut self.active
    }

    pub fn frame(&mut self) -> usize {
        self.current_frame
    }

    pub fn frame_mut(&mut self) -> &mut usize {
        &mut self.current_frame
    }

    pub fn inc_frame(&mut self, amount: usize) {
        self.current_frame += amount;
    }

    pub fn dec_frame(&mut self, amount: usize) {
        self.current_frame -= amount;
    }

    pub fn frame_at_end(&self) -> bool {
        self.current_frame >= self.frames.len() - 1
    }

    pub fn frame_at_start(&self) -> bool {
        self.current_frame == 0
    }

    pub fn last_frame(&self) -> usize {
        self.frames.len() - 1
    }

    pub fn timer_finished(&mut self) -> bool {
        self.timer.finished()
    }

    pub fn set_new_timer_duration(&mut self) {
        self.timer.set_duration(self.frames[self.current_frame].1);
        self.timer.start();
    }

    pub fn repetition(&mut self) -> GMRepetition {
        self.repetition
    }

    pub fn repetition_mut(&mut self) -> &mut GMRepetition {
        &mut self.repetition
    }

    pub fn update(&mut self) {
        if self.active && self.timer.finished() {
            match self.repetition {
                GMRepetition::OnceForward => {
                    if self.frame_at_end() {
                        self.active = false;
                    } else {
                        self.current_frame += 1;
                        self.set_new_timer_duration();
                    }
                }
                GMRepetition::OnceBackward => {
                    if self.frame_at_start() {
                        self.active = false;
                    } else {
                        self.current_frame -= 1;
                        self.set_new_timer_duration();
                    }
                }
                GMRepetition::LoopForward => {
                    if self.frame_at_end() {
                        // Restart animation
                        self.current_frame = 0;
                    } else {
                        self.current_frame += 1;
                    }
                    self.set_new_timer_duration();
                }
                GMRepetition::LoopBackward => {
                    if self.frame_at_start() {
                        // Restart animation
                        self.current_frame = self.frames.len() - 1;
                    } else {
                        self.current_frame -= 1;
                    }
                    self.set_new_timer_duration();
                }
                GMRepetition::PingPongForward => {
                    if self.frame_at_end() {
                        self.repetition =  GMRepetition::PingPongBackward;
                    } else {
                        self.current_frame += 1;
                    }
                    self.set_new_timer_duration();
                }
                GMRepetition::PingPongBackward => {
                    if self.frame_at_start() {
                        self.repetition =  GMRepetition::PingPongForward;
                    } else {
                        self.current_frame -= 1;
                    }
                    self.set_new_timer_duration();
                }
            }
        }
    }

    pub fn finished(&self) -> bool {
        match self.repetition {
            GMRepetition::OnceForward => {
                self.frame_at_end()
            }
            GMRepetition::OnceBackward => {
                self.frame_at_start()
            }
            _ => {
                false
            }
        }
    }

    pub fn reverse(&mut self) {
        match self.repetition {
            GMRepetition::OnceForward => {
                self.repetition = GMRepetition::OnceBackward;
            }
            GMRepetition::OnceBackward => {
                self.repetition = GMRepetition::OnceForward;
            }
            GMRepetition::LoopForward => {
                self.repetition = GMRepetition::LoopBackward;
            }
            GMRepetition::LoopBackward => {
                self.repetition = GMRepetition::LoopForward;
            }
            GMRepetition::PingPongForward => {
                self.repetition = GMRepetition::PingPongBackward;
            }
            GMRepetition::PingPongBackward => {
                self.repetition = GMRepetition::PingPongForward;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimation {
    base: GMAnimationBase,
    effects: Vec<Box<dyn GMAnimationEffectT>>,
}

impl GMAnimation {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            base: GMAnimationBase::new(frames),
            effects: Vec::new(),
        }
    }

    pub fn update(&mut self, context: &mut GMContext) {
        self.base.update();

        if self.base.active() {
            for effect in self.effects.iter_mut() {
                effect.update(&mut self.base, context);
            }
        }
    }

    pub fn base(&self) -> &GMAnimationBase {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut GMAnimationBase {
        &mut self.base
    }

    // Animation effect methods
    pub fn add_effect<T: 'static + GMAnimationEffectT>(&mut self, effect: T) {
        debug!("GMAnimation::add_effect()");
        self.add_effect2(Box::new(effect));
    }

    pub fn add_effect2(&mut self, effect: Box<dyn GMAnimationEffectT>) {
        debug!("GMAnimation::add_effect2()");
        self.effects.push(effect);
    }

    pub fn set_effects(&mut self, effects: Vec<Box<dyn GMAnimationEffectT>>) {
        debug!("GMAnimation::set_effects()");
        self.effects = effects;
    }

    pub fn remove_effect(&mut self, index: usize) {
        debug!("GMAnimation::remove_effect(), index: {}", index);
        self.effects.remove(index);
    }

    pub fn swap_effects(&mut self, index1: usize, index2: usize) {
        debug!("GMAnimation::swap_effects(), index1: {}, index2: {}", index1, index2);
        self.effects.swap(index1, index2);
    }

    pub fn send_effect_message(&mut self, index: usize, message: &str, context: &mut GMContext) {
        self.effects[index].send_message(message, context)
    }
}


pub struct GMAnimationBuilder {
    animation: GMAnimation,
}

impl GMAnimationBuilder {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            animation: GMAnimation::new(frames),
        }
    }

    pub fn with_active(mut self, active: bool) -> Self {
        debug!("GMAnimationSimpleBuilder::with_active(), active: '{}'", active);

        self.animation.base.active = active;
        self
    }

    pub fn with_current_frame(mut self, current_frame: usize) -> Self {
        debug!("GMAnimationSimpleBuilder::with_current_frame(), current_frame: '{}'", current_frame);

        self.animation.base.current_frame = current_frame;
        self
    }

    pub fn with_repetition(mut self, repetition: GMRepetition) -> Self {
        debug!("GMAnimationSimpleBuilder::with_repetition(), repetition: '{:?}'", repetition);

        self.animation.base.repetition = repetition;
        self
    }

    pub fn build(self) -> GMAnimation {
        self.animation
    }
}
