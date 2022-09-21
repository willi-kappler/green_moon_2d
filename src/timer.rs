

use std::time::Instant;

#[derive(Clone, Debug)]
pub struct GMTimer {
    active: bool,
    duration: f32,
    instant: Instant,
}

impl GMTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            active: true,
            duration,
            instant: Instant::now(),
        }
    }

    pub fn finished(&mut self) -> bool {
        if self.active {
            if self.instant.elapsed().as_secs_f32() >= self.duration {
                self.active = false;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_duration(&self) -> f32 {
        self.duration
    }

    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }

    pub fn start(&mut self) {
        self.instant = Instant::now();
        self.active = true;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_active(&self) -> bool {
        self.active
    }
}
