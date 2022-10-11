
use std::collections::HashSet;

use crate::context::{GMContext, GMObjectMessage};
use crate::effect::GMEffectManager;
use crate::data::GMData;

pub trait GMObjectManagerT {
    fn update(&mut self, context: &mut GMContext);
    fn draw(&self, context: &mut GMContext);
    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext);
    fn name(&self) -> &str;
    fn groups(&self) -> &HashSet<String>;
}

pub struct GMObjectManager<T> {
    base: T,
    effects: GMEffectManager<T>,
}

impl<T: GMObjectManagerT> GMObjectManager<T> {
    pub fn update(&mut self, context: &mut GMContext) {
        self.base.update(context);
        self.effects.update(&mut self.base, context);
    }

    pub fn draw(&self, context: &mut GMContext) {
        self.base.draw(context);
        self.effects.draw(&self.base, context);
    }

    pub fn check_messages(&mut self, context: &mut GMContext) {
        let mut messages = context.get_object_messages(self.base.name());

        while let Some(message) = messages.pop_front() {
            match message {
                GMObjectMessage::Base(message, data) => {
                    self.base.send_message(&message, data, context);
                }
                GMObjectMessage::Effect(index, message, data) => {
                    self.effects.send_message(index, &message, data, context);
                }
            }
        }

        let mut messages = context.get_group_messages(self.base.groups());

        while let Some(message) = messages.pop_front() {
            match message {
                GMObjectMessage::Base(message, data) => {
                    self.base.send_message(&message, data, context);
                }
                GMObjectMessage::Effect(index, message, data) => {
                    self.effects.send_message(index, &message, data, context);
                }
            }
        }
    }
}
