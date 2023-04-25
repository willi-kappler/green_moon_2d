

use std::time::Instant;

#[derive(Clone, Debug)]
pub struct GMTimer {
    pub active: bool,
    pub duration: f32,
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

    pub fn start(&mut self) {
        self.instant = Instant::now();
        self.active = true;
    }
}
