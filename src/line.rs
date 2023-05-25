
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefMut;
use std::any::Any;

use log::debug;

use crate::context::GMContext;
use crate::math::GMVec2D;
use crate::object_manager::GMObjectManager;
use crate::object::GMObjectT;
use crate::util::error_panic;
use crate::value::GMValue;
use crate::message::{GMMessage, msg1v};

#[derive(Debug, Clone)]
pub enum GMLineMode {
    Number(u32),
    Spacing(f32),
}

impl GMObjectT for GMLineMode {
    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

impl From<Rc<dyn Any>> for GMLineMode {
    fn from(object: Rc<dyn Any>) -> Self {
        let line_mode = object.downcast::<GMLineMode>().unwrap();
        (*line_mode).clone()
    }
}

#[derive(Debug, Clone)]
pub struct GMLine {
    pub start: GMVec2D,
    pub end: GMVec2D,
    pub init_element: Box<dyn GMObjectT>,
    pub elements: Vec<Box<dyn GMObjectT>>,
    pub line_mode: GMLineMode,
}

impl GMLine {
    pub fn new<T: Into<GMVec2D>, U: Into<GMVec2D>, V: Into<Box<dyn GMObjectT>>>(start: T, end: U, init_element: V, line_mode: GMLineMode) -> Self {
        let start = start.into();
        let end = end.into();
        debug!("GMLine::new(), start: '{:?}', end: '{:?}', line_mode: '{:?}'", start, end, line_mode);

        Self {
            start,
            end,
            init_element: init_element.into(),
            elements: Vec::new(),
            line_mode,
        }
    }

    pub fn set_start<V: Into<GMVec2D>>(&mut self, start: V) -> Vec<GMVec2D> {
        self.start = start.into();
        self.point_changed()
    }

    pub fn set_end<V: Into<GMVec2D>>(&mut self, end: V) -> Vec<GMVec2D> {
        self.end = end.into();
        self.point_changed()
    }

    pub fn point_changed(&mut self) -> Vec<GMVec2D> {
        let direction = self.end - self.start;
        let length = direction.len();

        match self.line_mode {
            GMLineMode::Number(number) => {
                let spacing = length / (number as f32);
                self.set_elements(number, spacing, direction)
            }
            GMLineMode::Spacing(spacing) => {
                let number = (length / spacing).floor() as u32;
                self.set_elements(number, spacing, direction)
            }
        }
    }

    pub fn set_number(&mut self, number: u32) -> Vec<GMVec2D> {
        self.line_mode = GMLineMode::Number(number);

        let direction = self.end - self.start;
        let length = direction.len();
        let spacing = length / (number as f32);

        self.set_elements(number, spacing, direction)
    }

    pub fn set_spacing(&mut self, spacing: f32) -> Vec<GMVec2D> {
        self.line_mode = GMLineMode::Spacing(spacing);

        let direction = self.end - self.start;
        let length = direction.len();
        let number = (length / spacing).floor() as u32;

        self.set_elements(number, spacing, direction)
    }

    pub fn set_elements(&mut self, number: u32, spacing: f32, mut direction: GMVec2D) -> Vec<GMVec2D> {
        direction.norm();

        // If more elements are needed just add them
        let diff = ((number as i32) - (self.elements.len() as i32)) as i32;

        for _ in 0..diff {
            self.elements.push(self.init_element.clone());
        }

        // Remove unneeded elements:
        self.elements.truncate(number as usize);

        // Now re-calculate the positions of all elements
        let mut result = Vec::new();

        for i in 0..number {
            let new_position = self.start + (direction * (spacing * (i as f32)));
            result.push(new_position);
        }

        result
    }
}

impl GMObjectT for GMLine {
    /*
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Custom0(name) if name == "get_line_mode" => {
                return GMValue::Any(Rc::new(self.line_mode.clone()))
            }
            GMMessage::Custom1(name, GMValue::Object(init_element)) if name == "set_init_element" => {
                self.init_element = init_element;
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_line_mode" => {
                self.line_mode = value.into();
            }

            // TODO: more messages...
            _ => {
                error_panic(&format!("Wrong message for GMMVFollow::send_message: '{:?}'", message))
            }

        }

        GMValue::None
    }
    */

    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "init" => {
                        let positions = self.point_changed();

                        for (element, position) in self.elements.iter_mut().zip(positions) {
                            element.send_message(msg1v("set_position", position), object_manager);
                        }
                    }
                    "get_line_mode" => {
                        // TODO:
                        todo!();
                    }
                    "set_line_mode" => {
                        // TODO:
                        todo!();
                    }
                    "get_init_element" => {
                        return self.init_element.clone().into();
                    }
                    "set_init_element" => {
                        // TODO:
                        todo!();
                    }
                    _ => {
                        error_panic(&format!("GMLine::send_message: Unknown method '{}', no tag", method));
                    }
                }
            }
            "start" => {
                return self.start.send_message(method, value);
            }
            "start2" => {
                let result = self.start.send_message(method, value);

                if result.is_none() {
                    self.point_changed();
                }

                return result;
            }
            "end" => {
                return self.end.send_message(method, value);
            }
            "end2" => {
                let result = self.end.send_message(method, value);

                if result.is_none() {
                    self.point_changed();
                }

                return result;
            }
            _ => {
                error_panic(&format!("GMLine::send_message: Unknown tag '{}'", tag));
            }
        }

        GMValue::None
    }

    fn draw(&self, context: &mut RefMut<&mut GMContext>) {
        for element in self.elements.iter() {
            element.draw(context);
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
