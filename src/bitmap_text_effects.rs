
use std::f32::consts::TAU;

use log::debug;

use crate::movement::GMPositionT;
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

