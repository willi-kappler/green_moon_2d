
use std::fmt::Debug;

// use log::debug;
use hecs::World;

use crate::timer::GMTimer;
use crate::util::{GMRepetition, GMActive};
use crate::texture::GMTextureIndex;

#[derive(Clone, Debug)]
pub struct GMAnimation {
    pub current_frame: usize,
    pub frames: Vec<(u32, f32)>, // index, duration in seconds
    pub timer: GMTimer,
    pub repetition: GMRepetition,
}

impl GMAnimation {
    pub fn new(frames: &[(u32, f32)], repetition: GMRepetition) -> Self {
        Self {
            current_frame: 0,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
            repetition,
        }
    }

    pub fn texture_index(&self) -> u32 {
        self.frames[self.current_frame].0
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

pub fn gm_static_animation() -> GMAnimation {
    GMAnimation {
        current_frame: 0,
        frames: vec![(0, 0.0)],
        timer: GMTimer::new(0.0),
        repetition: GMRepetition::OnceForward,
    }
}

// ECS
pub fn gm_process_animations(world: &mut World) {
    for (_, (animation,
             texture_index,
             active)) in
        world.query_mut::<(&mut GMAnimation, &mut GMTextureIndex, &mut GMActive)>() {
        if active.0 && animation.timer.finished() {
            match animation.repetition {
                GMRepetition::OnceForward => {
                    if animation.frame_at_end() {
                        active.0 = false;
                    } else {
                        animation.current_frame += 1;
                        animation.set_new_timer_duration();
                        texture_index.0 = animation.texture_index();
                    }
                }
                GMRepetition::OnceBackward => {
                    if animation.frame_at_start() {
                        active.0 = false;
                    } else {
                        animation.current_frame -= 1;
                        animation.set_new_timer_duration();
                        texture_index.0 = animation.texture_index();
                    }
                }
                GMRepetition::LoopForward => {
                    if animation.frame_at_end() {
                        // Restart animation
                        animation.current_frame = 0;
                    } else {
                        animation.current_frame += 1;
                    }
                    animation.set_new_timer_duration();
                    texture_index.0 = animation.texture_index();
                }
                GMRepetition::LoopBackward => {
                    if animation.frame_at_start() {
                        // Restart animation
                        animation.current_frame = animation.frames.len() - 1;
                    } else {
                        animation.current_frame -= 1;
                    }
                    animation.set_new_timer_duration();
                    texture_index.0 = animation.texture_index();
                }
                GMRepetition::PingPongForward => {
                    if animation.frame_at_end() {
                        animation.repetition =  GMRepetition::PingPongBackward;
                    } else {
                        animation.current_frame += 1;
                    }
                    animation.set_new_timer_duration();
                    texture_index.0 = animation.texture_index();
                }
                GMRepetition::PingPongBackward => {
                    if animation.frame_at_start() {
                        animation.repetition =  GMRepetition::PingPongForward;
                    } else {
                        animation.current_frame -= 1;
                    }
                    animation.set_new_timer_duration();
                    texture_index.0 = animation.texture_index();
                }
            }
        }
    }
}
