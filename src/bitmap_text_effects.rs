
use std::f32::consts::TAU;

use log::debug;
use nanorand::{Rng, WyRand, SeedableRng};

use crate::movement::{GMPositionT, GMRotationT, GMScaleT};
use crate::util::{GMActiveT, GMProperty};
use crate::bitmap_text::GMBitmapText;
use crate::context::GMContext;

use crate::gen_impl_active;

#[macro_export]
macro_rules! gen_get_set_amplitude {
    () => {
        pub fn set_amplitude(&mut self, amplitude: f32) {
            self.amplitude = amplitude
        }

        pub fn inc_amplitude(&mut self, inc: f32) {
            self.amplitude += inc;
        }

        pub fn get_amplitude(&self) -> f32 {
            self.amplitude
        }

        pub fn dec_amplitude(&mut self, dec: f32) {
            self.amplitude -= dec;
        }
    };
}

#[macro_export]
macro_rules! gen_get_set_speed {
    () => {
        pub fn set_speed(&mut self, speed: f32) {
            self.speed = speed
        }

        pub fn inc_speed(&mut self, inc: f32) {
            self.speed += inc;
        }

        pub fn get_speed(&self) -> f32 {
            self.speed
        }

        pub fn dec_speed(&mut self, dec: f32) {
            self.speed -= dec;
        }
    };
}

#[macro_export]
macro_rules! gen_get_set_offset {
    () => {
        pub fn set_offset(&mut self, offset: f32) {
            self.offset = offset
        }

        pub fn inc_offset(&mut self, inc: f32) {
            self.offset += inc;
        }

        pub fn get_offset(&self) -> f32 {
            self.offset
        }

        pub fn dec_offset(&mut self, dec: f32) {
            self.offset -= dec;
        }
    };
}

#[macro_export]
macro_rules! gen_get_set_radius {
    () => {
        pub fn set_radius(&mut self, radius: f32) {
            self.radius = radius
        }

        pub fn inc_radius(&mut self, inc: f32) {
            self.radius += inc;
        }

        pub fn get_radius(&self) -> f32 {
            self.radius
        }

        pub fn dec_radius(&mut self, dec: f32) {
            self.radius -= dec;
        }
    };
}

#[macro_export]
macro_rules! gen_get_set_base {
    () => {
        pub fn set_base(&mut self, base: f32) {
            self.base = base
        }

        pub fn inc_base(&mut self, inc: f32) {
            self.base += inc;
        }

        pub fn get_base(&self) -> f32 {
            self.base
        }

        pub fn dec_base(&mut self, dec: f32) {
            self.base -= dec;
        }
    };
}

pub trait GMTextEffectT {
    fn update(&mut self, text: &mut GMBitmapText, context: &mut GMContext);
    fn send_message(&mut self, message: &str, context: &mut GMContext);
    fn set_property(&mut self, name: &str, value: GMProperty, context: &mut GMContext);
    fn get_property(&self, name: &str, context: &mut GMContext) -> GMProperty;
}

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

    gen_get_set_amplitude!();

    gen_get_set_speed!();

    gen_get_set_offset!();
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

    gen_get_set_radius!();

    gen_get_set_speed!();
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

    gen_get_set_speed!();

    gen_get_set_offset!();
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

    gen_get_set_amplitude!();

    gen_get_set_base!();

    gen_get_set_speed!();

    gen_get_set_offset!();
}

gen_impl_active!(GMTEScale);
