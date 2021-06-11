
use crate::error::GMError;

use macroquad::audio::{Sound, load_sound, play_sound, stop_sound, set_sound_volume, PlaySoundParams};

use std::cell::Cell;
use std::rc::Rc;

pub struct GMSound {
    pub(crate) data: Sound,
    pub(crate) looped: Cell<bool>,
    pub(crate) volume: Cell<f32>,
}

impl GMSound {
    pub async fn new(file_name: &str) -> Result<Self, GMError> {
        let data = load_sound(file_name).await?;
        let sound = Self {
            data,
            looped: Cell::new(false),
            volume: Cell::new(1.0),
        };
        Ok(sound)
    }
    pub async fn new_rc(file_name: &str) -> Result<Rc<Self>, GMError> {
        let sound = Self::new(file_name).await?;
        Ok(Rc::new(sound))
    }
    pub fn play(&self) {
        let params = PlaySoundParams { looped: self.looped.get(), volume: self.volume.get() };
        play_sound(self.data, params);
    }
    pub fn stop(&self) {
        stop_sound(self.data);
    }
    pub fn set_sound_volume(&self, volume: f32) {
        self.volume.set(volume);
        set_sound_volume(self.data, volume);
    }
    pub fn set_loop(&self, looped: bool) {
        self.looped.set(looped);
    }
}
