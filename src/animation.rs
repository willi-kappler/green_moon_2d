
use std::fmt::Debug;

use log::debug;

use crate::timer::GMTimer;
use crate::util::{GMRepetition};

#[derive(Clone, Debug)]
pub struct GMAnimation {
    pub active: bool,
    pub current_frame: usize,
    pub repetition: GMRepetition,
    frames: Vec<(u32, f32)>, // (index, duration in seconds)
    timer: GMTimer,
}

impl GMAnimation {
    pub fn new(frames: &[(u32, f32)], repetition: GMRepetition) -> Self {
        debug!("GMAnimation::new(), number of frames: {}, repetition: {:?}", frames.len(), repetition);

        Self {
            active: true,
            current_frame: 0,
            repetition,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
        }
    }

    pub fn texture_index(&self) -> u32 {
        self.frames[self.current_frame].0
    }

    pub fn frame_at_end(&self) -> bool {
        self.current_frame >= self.frames.len() - 1
    }

    pub fn frame_at_start(&self) -> bool {
        self.current_frame == 0
    }

    fn set_new_timer_duration(&mut self) {
        self.timer.duration = self.frames[self.current_frame].1;
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
        self.repetition.reverse();
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
                        self.repetition.reverse();
                    } else {
                        self.current_frame += 1;
                    }
                    self.set_new_timer_duration();
                }
                GMRepetition::PingPongBackward => {
                    if self.frame_at_start() {
                        self.repetition.reverse();
                    } else {
                        self.current_frame -= 1;
                    }
                    self.set_new_timer_duration();
                }
            }
        }
    }
}
