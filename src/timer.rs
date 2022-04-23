

use std::time::Instant;

#[derive(Clone, Debug)]
pub struct GMTimer {
    duration: f32,
    instant: Instant,
}

impl GMTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            instant: Instant::now(),
        }
    }

    pub fn finished(&self) -> bool {
        self.instant.elapsed().as_secs_f32() >= self.duration
    }

    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
        self.instant = Instant::now();
    }

    pub fn start(&mut self) {
        self.instant = Instant::now();
    }
}
