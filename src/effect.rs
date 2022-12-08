
use std::fmt::Debug;

use log::debug;

use crate::context::GMContext;
use crate::data::GMData;

pub trait GMEffectT<T>: Debug {
    fn update(&mut self, _base: &mut T, _context: &mut GMContext) {
    }

    fn draw(&self, _base: &T, _context: &mut GMContext) {
    }

    fn send_message(&mut self, _message: &str, _data: GMData, _context: &mut GMContext) {
    }

    fn send_message2(&mut self, message: &str, context: &mut GMContext) {
        self.send_message(message, GMData::None, context);
    }

    fn set_active(&mut self, active: bool);

    fn clone_box(&self) -> Box<dyn GMEffectT<T>>;

    fn get_property(&self, name: &str) -> GMData;

    fn init(&mut self, _base: &mut T) {
    }
}

impl<T> Clone for Box<dyn GMEffectT<T>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub struct GMEffectManager<T> {
    effects: Vec<Box<dyn GMEffectT<T>>>,
}

impl<T> GMEffectManager<T> {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn new2(effects: Vec<Box<dyn GMEffectT<T>>>) -> Self {
        Self {
            effects,
        }
    }

    pub fn update(&mut self, base: &mut T, context: &mut GMContext) {
        for effect in self.effects.iter_mut() {
            effect.update(base, context);
        }
    }

    pub fn draw(&self, base: &T, context: &mut GMContext) {
        for effect in self.effects.iter() {
            effect.draw(base, context);
        }
    }

    pub fn add_effect<E: 'static + GMEffectT<T>>(&mut self, effect: E) {
        debug!("GMEffectManager::add_effect()");
        self.add_effect2(Box::new(effect));
    }

    pub fn add_effect2(&mut self, effect: Box<dyn GMEffectT<T>>) {
        debug!("GMEffectManager::add_effect2()");
        self.effects.push(effect);
    }

    pub fn set_effects(&mut self, effects: Vec<Box<dyn GMEffectT<T>>>) {
        debug!("GMEffectManager::set_effects()");
        self.effects = effects;
    }

    pub fn replace_effect(&mut self, index: usize, effect: Box<dyn GMEffectT<T>>) {
        debug!("GMEffectManager::replace_effect()");
        self.effects[index] = effect;
    }

    pub fn remove_effect(&mut self, index: usize) {
        debug!("GMEffectManager::remove_effect(), index: {}", index);
        self.effects.remove(index);
    }

    pub fn clear_effects(&mut self) {
        debug!("GMEffectManager::clear_effects()");
        self.effects.clear();
    }

    pub fn swap_effects(&mut self, index1: usize, index2: usize) {
        debug!("GMEffectManager::swap_effect(), index1: {}, index2: {}", index1, index2);
        self.effects.swap(index1, index2);
    }

    pub fn send_message(&mut self, index: usize, message: &str, data: GMData, context: &mut GMContext) {
        self.effects[index].send_message(message, data, context)
    }

    pub fn send_message2(&mut self, index: usize, message: &str, context: &mut GMContext) {
        self.effects[index].send_message2(message, context)
    }
}
