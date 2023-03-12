
use std::f32::consts::TAU;

use log::debug;
use nanorand::{Rng, WyRand, SeedableRng};

use crate::movement::{GMPositionT, GMRotationT, GMScaleT};
use crate::util::{GMActiveT};
use crate::bitmap_text::GMBitmapText;
use crate::context::GMContext;

use crate::gen_impl_active;

#[derive(Debug, Clone)]
pub struct GMTEWave {
    amplitude: f32,
    speed: f32,
    offset: f32,
    time: f32,
    active: bool,
}

impl GMTEWave {
    pub fn new(amplitude: f32, speed: f32, offset: f32) -> Self {
        debug!("GMTEWave::new(), amplitude: '{}', speed: '{}', offset: '{}'", amplitude, speed, offset);

        Self {
            amplitude,
            speed,
            offset,
            time: 0.0,
            active: true,
        }
    }

    pub fn update(&mut self, text: &mut GMBitmapText, _context: &mut GMContext) {
        if self.active {
            let mut offset = 0.0;

            if text.get_horizontal() {
                for bitmap_char in text.get_chars_mut() {
                    bitmap_char.add_position_y(self.amplitude * (self.time + offset).sin());
                    offset += self.offset;
                }
            } else {
                for bitmap_char in text.get_chars_mut() {
                    bitmap_char.add_position_x(self.amplitude * (self.time + offset).sin());
                    offset += self.offset;
                }
            }

            self.time += self.speed;

            if self.time > TAU {
                self.time -= TAU;
            }
        }
    }
}

gen_impl_active!(GMTEWave);

#[derive(Debug, Clone)]
pub struct GMTEShake {
    radius: f32,
    speed: f32,
    time: f32,
    seed: u64,
    rng: WyRand,
    active: bool,
}

impl GMTEShake {
    pub fn new(radius: f32, speed: f32) -> Self {
        debug!("GMTEShake::new(), radius: '{}'", radius);

        let seed = 1;
        let rng = WyRand::new();

        Self {
            radius,
            speed,
            time: 0.0,
            seed,
            rng,
            active: true,
        }
    }

    pub fn update(&mut self, text: &mut GMBitmapText, _context: &mut GMContext) {
        if self.active {
            self.time += self.speed;
            self.rng.reseed(u64::to_ne_bytes(self.seed));

            for bitmap_char in text.get_chars_mut() {
                let dx = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;
                let dy = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;

                bitmap_char.add_position((dx, dy));
            }

            if self.time > 1.0 {
                self.time = 0.0;
                self.seed += 1;
            }
        }
    }
}

gen_impl_active!(GMTEShake);

#[derive(Debug, Clone)]
pub struct GMTERotateChars {
    speed: f32,
    offset: f32,
    time: f32,
    active: bool,
}

impl GMTERotateChars {
    pub fn new(speed: f32, offset: f32) -> Self {
        debug!("GMTERotateChars::new(), speed: '{}', offset: '{}'", speed, offset);

        Self {
            speed,
            offset,
            time: 0.0,
            active: true,
        }
    }

    pub fn update(&mut self, text: &mut GMBitmapText, _context: &mut GMContext) {
        if self.active {
            let mut delta = 0.0;

            for bitmap_char in text.get_chars_mut() {
                bitmap_char.set_angle(self.time + delta);
                delta += self.offset;
            }

            self.time += self.speed;
        }
    }

}

gen_impl_active!(GMTERotateChars);

#[derive(Debug, Clone)]
pub struct GMTEScale {
    amplitude: f32,
    base: f32,
    speed: f32,
    offset: f32,
    time: f32,
    active: bool,
}

impl GMTEScale {
    pub fn new(amplitude: f32, base: f32, speed: f32, offset: f32) -> Self {
        debug!("GMTEScale::new(), amplitude: '{}', base: '{}', speed: '{}', offset: '{}'", amplitude, base, speed, offset);

        Self {
            amplitude,
            base,
            speed,
            offset,
            time: 0.0,
            active: true,
        }
    }

    pub fn update(&mut self, text: &mut GMBitmapText, _context: &mut GMContext) {
        let mut offset = 0.0;

        if self.active {
            for bitmap_char in text.get_chars_mut() {
                bitmap_char.set_scale(self.base + (self.amplitude * (self.time + offset).sin()));
                offset += self.offset;
            }

            self.time += self.speed;
        }
    }

}

gen_impl_active!(GMTEScale);
