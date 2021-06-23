use std::rc::Rc;
use std::any::Any;

pub trait GMBehaviorT {
    fn init(&mut self);
    fn update(&mut self, key: &str, value: Rc<dyn Any>) -> (&str, Rc<dyn Any>);
}
