
use std::f32::consts::TAU;

use log::debug;
use nanorand::{Rng, WyRand, SeedableRng};

use crate::movement::{GMPositionT, GMRotationT, GMScaleT};
use crate::util::{GMActiveT, GMValue, GMMessage, GMSetProperty, GMGetProperty, error_panic};
use crate::bitmap_text::GMBitmapText;

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
    };
}

// TODO: write a macro to generate get / set property functions
// TODO: refactor message and property into separate traits

pub trait GMTextEffectT {
    fn update(&mut self, text: &mut GMBitmapText);
    fn send_message(&mut self, message: GMMessage);
    fn set_property(&mut self, property: GMSetProperty);
    fn get_property(&self, property: GMGetProperty) -> GMValue;
    fn clone_box(&self) -> Box<dyn GMTextEffectT>;
}

impl Clone for Box<dyn GMTextEffectT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
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

    gen_get_set_amplitude!();

    gen_get_set_speed!();

    gen_get_set_offset!();
}

gen_impl_active!(GMTEWave);

impl GMTextEffectT for GMTEWave {
    fn update(&mut self, text: &mut GMBitmapText) {
        if self.active {
            let mut offset = 0.0;

            if text.get_horizontal() {
                for bitmap_char in text.get_chars_mut() {
                    bitmap_char.add_position_y((self.time + offset).sin() * self.amplitude);
                    offset += self.offset;
                }
            } else {
                for bitmap_char in text.get_chars_mut() {
                    bitmap_char.add_position_x((self.time + offset).sin() * self.amplitude);
                    offset += self.offset;
                }
            }

            self.time += self.speed;

            if self.time > TAU {
                self.time -= TAU;
            }
        }
    }

    fn send_message(&mut self, message: GMMessage) {
        match message {
            GMMessage::IncAmplitude(value) => {
                self.inc_amplitude(value);
            }
            GMMessage::IncSpeed(value) => {
                self.inc_speed(value);
            }
            GMMessage::IncOffset(value) => {
                self.inc_offset(value);
            }
            _ => {
                error_panic(&format!("send_message(), unknown message '{:?}'", message));
            }
        }
    }

    fn set_property(&mut self, property: GMSetProperty) {
        match property {
            GMSetProperty::Amplitude(value) => {
                self.amplitude = value;
            }
            GMSetProperty::Speed(value) => {
                self.speed = value;
            }
            GMSetProperty::Offset(value) => {
                self.offset = value;
            }
            _ => {
                error_panic(&format!("set_property(), unknown property '{:?}'", property));
            }
        }
    }

    fn get_property(&self, property: GMGetProperty) -> GMValue {
        match property {
            GMGetProperty::Amplitude => {
                GMValue::F32(self.amplitude)
            }
            GMGetProperty::Speed => {
                GMValue::F32(self.speed)
            }
            GMGetProperty::Offset => {
                GMValue::F32(self.offset)
            }
            _ => {
                error_panic(&format!("get_property(), unknown property '{:?}'", property));
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMTextEffectT> {
        Box::new(self.clone())
    }
}

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

    gen_get_set_radius!();

    gen_get_set_speed!();
}

gen_impl_active!(GMTEShake);

impl GMTextEffectT for GMTEShake {
    fn update(&mut self, text: &mut GMBitmapText) {
        if self.active {
            self.time += self.speed;
            self.rng.reseed(u64::to_ne_bytes(self.seed));

            for bitmap_char in text.get_chars_mut() {
                let dx = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;
                let dy = ((self.rng.generate::<f32>() * 2.0) - 1.0) * self.radius;

                bitmap_char.add_position_xy(dx, dy);
            }

            if self.time > 1.0 {
                self.time = 0.0;
                self.seed += 1;
            }
        }
    }

    fn send_message(&mut self, message: GMMessage) {
        match message {
            GMMessage::IncRadius(value) => {
                self.inc_radius(value);
            }
            GMMessage::IncSpeed(value) => {
                self.inc_speed(value);
            }
            _ => {
                error_panic(&format!("send_message(), unknown message '{:?}'", message));
            }
        }
    }

    fn set_property(&mut self, property: GMSetProperty) {
        match property {
            GMSetProperty::Radius(value) => {
                self.radius = value;
            }
            GMSetProperty::Speed(value) => {
                self.speed = value;
            }
            _ => {
                error_panic(&format!("set_property(), unknown property '{:?}'", property));
            }
       }
    }

    fn get_property(&self, property: GMGetProperty) -> GMValue {
        match property {
            GMGetProperty::Radius => {
                GMValue::F32(self.radius)
            }
            GMGetProperty::Speed => {
                GMValue::F32(self.speed)
            }
            _ => {
                error_panic(&format!("get_property(), unknown property '{:?}'", property));
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMTextEffectT> {
        Box::new(self.clone())
    }
}


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

    gen_get_set_speed!();

    gen_get_set_offset!();
}

gen_impl_active!(GMTERotateChars);

impl GMTextEffectT for GMTERotateChars {
    fn update(&mut self, text: &mut GMBitmapText) {
        if self.active {
            let mut delta = 0.0;

            for bitmap_char in text.get_chars_mut() {
                bitmap_char.set_angle(self.time + delta);
                delta += self.offset;
            }

            self.time += self.speed;
        }
    }

    fn send_message(&mut self, message: GMMessage) {
        match message {
            GMMessage::IncSpeed(value) => {
                self.inc_speed(value);
            }
            GMMessage::IncOffset(value) => {
                self.inc_offset(value);
            }
            _ => {
                error_panic(&format!("send_message(), unknown message '{:?}'", message));
            }
        }
    }

    fn set_property(&mut self, property: GMSetProperty) {
        match property {
            GMSetProperty::Speed(value) => {
                self.speed = value;
            }
            GMSetProperty::Offset(value) => {
                self.offset = value;
            }
            _ => {
                error_panic(&format!("set_property(), unknown property '{:?}'", property));
            }
        }
    }

    fn get_property(&self, property: GMGetProperty) -> GMValue {
        match property {
            GMGetProperty::Speed => {
                GMValue::F32(self.speed)
            }
            GMGetProperty::Offset => {
                GMValue::F32(self.offset)
            }
            _ => {
                error_panic(&format!("get_property(), unknown property '{:?}'", property));
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMTextEffectT> {
        Box::new(self.clone())
    }
}


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

    gen_get_set_amplitude!();

    gen_get_set_base!();

    gen_get_set_speed!();

    gen_get_set_offset!();
}

gen_impl_active!(GMTEScale);

impl GMTextEffectT for GMTEScale {
    fn update(&mut self, text: &mut GMBitmapText) {
        let mut offset = 0.0;

        if self.active {
            for bitmap_char in text.get_chars_mut() {
                bitmap_char.set_scale(self.base + (self.amplitude * (self.time + offset).sin()));
                offset += self.offset;
            }

            self.time += self.speed;
        }
    }

    fn send_message(&mut self, message: GMMessage) {
        match message {
            GMMessage::IncAmplitude(value) => {
                self.inc_amplitude(value);
            }
            GMMessage::IncBase(value) => {
                self.inc_base(value);
            }
            GMMessage::IncSpeed(value) => {
                self.inc_speed(value);
            }
            GMMessage::IncOffset(value) => {
                self.inc_offset(value);
            }
            _ => {
                error_panic(&format!("send_message(), unknown message '{:?}'", message));
            }
        }
    }

    fn set_property(&mut self, property: GMSetProperty) {
        match property {
            GMSetProperty::Amplitude(value) => {
                self.amplitude = value;
            }
            GMSetProperty::Base(value) => {
                self.base = value;
            }
            GMSetProperty::Speed(value) => {
                self.speed = value;
            }
            GMSetProperty::Offset(value) => {
                self.offset = value;
            }
            _ => {
                error_panic(&format!("set_property(), unknown property '{:?}'", property));
            }
        }
    }

    fn get_property(&self, property: GMGetProperty) -> GMValue {
        match property {
            GMGetProperty::Amplitude => {
                GMValue::F32(self.amplitude)
            }
            GMGetProperty::Base => {
                GMValue::F32(self.base)
            }
            GMGetProperty::Speed => {
                GMValue::F32(self.speed)
            }
            GMGetProperty::Offset => {
                GMValue::F32(self.offset)
            }
            _ => {
                error_panic(&format!("get_property(), unknown property '{:?}'", property));
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMTextEffectT> {
        Box::new(self.clone())
    }
}
