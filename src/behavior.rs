use std::any::Any;

pub struct GMKeyValue<'a> {
    pub key: &'a str,
    pub value: Box<dyn Any>,
}

impl<'a> GMKeyValue<'a> {
    pub fn new(key: &'a str, value: Box<dyn Any>) -> Self {
        Self {
            key,
            value,
        }
    }
}

pub trait GMBehaviorT {
    fn init(&mut self);
    fn update(&mut self, data: &GMKeyValue) -> GMKeyValue;
}
