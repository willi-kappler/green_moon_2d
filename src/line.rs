
use std::fmt::Debug;

use log::debug;

use crate::context::GMContext;
use crate::math::{GMVec2D, GMSize};
use crate::object_manager::GMObjectManager;
use crate::object::{GMObjectT, GMObjectBox};
use crate::util::error_panic;
use crate::value::GMValue;
use crate::message::{GMMessage, msgt1v, msgt0v};

#[derive(Debug, Clone)]
pub enum GMLineMode {
    Number(u32),
    Spacing(f32),
}

impl GMObjectT for GMLineMode {
    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMLine {
    pub start: GMVec2D,
    pub end: GMVec2D,
    pub init_element: GMObjectBox,
    pub elements: Vec<GMObjectBox>,
    pub line_mode: GMLineMode,
}

impl GMLine {
    pub fn new<T: Into<GMVec2D>, U: Into<GMVec2D>, V: Into<GMObjectBox>>(start: T, end: U, init_element: V, line_mode: GMLineMode) -> Self {
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

    pub fn set_number2(&mut self, number: u32, object_manager: &GMObjectManager) {
        let positions = self.set_number(number);
        self.set_positions(positions, object_manager);
    }

    pub fn set_spacing(&mut self, spacing: f32) -> Vec<GMVec2D> {
        self.line_mode = GMLineMode::Spacing(spacing);

        let direction = self.end - self.start;
        let length = direction.len();
        let number = (length / spacing).floor() as u32;

        self.set_elements(number, spacing, direction)
    }

    pub fn set_spacing2(&mut self, spacing: f32, object_manager: &GMObjectManager) {
        let positions = self.set_spacing(spacing);
        self.set_positions(positions, object_manager);
    }

    pub fn set_elements(&mut self, number: u32, spacing: f32, mut direction: GMVec2D) -> Vec<GMVec2D> {
        direction.norm();

        let diff = ((number as i32) - (self.elements.len() as i32)) as i32;

        if diff > 0 {
            // If more elements are needed just add them
            for _ in 0..diff {
                self.elements.push(self.init_element.clone());
            }
        } else if diff < 0 {
            // Remove unneeded elements:
            self.elements.truncate(number as usize);
        }

        // Now re-calculate the positions of all elements
        let mut result = Vec::new();

        for i in 0..number {
            let new_position = self.start + (direction * (spacing * (i as f32)));
            result.push(new_position);
        }

        result
    }

    pub fn set_positions(&mut self, positions: Vec<GMVec2D>, object_manager: &GMObjectManager) {
        for (element, position) in self.elements.iter_mut().zip(positions) {
            element.send_message(msgt1v("position", "set", position), object_manager);
        }
    }
}

impl GMObjectT for GMLine {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag1 = message.next_tag();
        let tag2 = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag1.as_str() {
            "" => {
                match method {
                    "init" => {
                        let positions = self.point_changed();
                        self.set_positions(positions, object_manager);
                    }
                    "get_line_mode" => {
                        return self.line_mode.clone().into();
                    }
                    "set_line_mode" => {
                        self.line_mode = value.into_line_mode();
                    }
                    "get_init_element" => {
                        return self.init_element.clone().into();
                    }
                    "set_init_element" => {
                        self.init_element = value.into_object();
                    }
                    "set_all_elements" => {
                        self.elements.clear();

                        let positions = self.point_changed();
                        self.set_positions(positions, object_manager);
                    }
                    "set_some_elements" => {
                        let indices = value.into_generic::<Vec<usize>>();

                        for index in indices {
                            let position = self.elements[index].send_message(msgt0v("position", "get"), object_manager);
                            self.elements[index] = self.init_element.clone();
                            self.elements[index].send_message(msgt1v("position", "set", position), object_manager);
                        }
                    }
                    "set_one_element" => {
                        let index = value.into_usize();
                        let position = self.elements[index].send_message(msgt0v("position", "get"), object_manager);
                        self.elements[index] = self.init_element.clone();
                        self.elements[index].send_message(msgt1v("position", "set", position), object_manager);
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
                    let positions = self.point_changed();
                    self.set_positions(positions, object_manager);
                }

                return result;
            }
            "end" => {
                return self.end.send_message(method, value);
            }
            "end2" => {
                let result = self.end.send_message(method, value);

                if result.is_none() {
                    let positions = self.point_changed();
                    self.set_positions(positions, object_manager);
                }

                return result;
            }
            "object" => {
                let (index, new_value) = value.into_generic::<(usize, GMValue)>();
                self.elements[index].send_message(msgt1v(tag2, method, new_value), object_manager);
            }
            "some_objects" => {
                let mut new_values = value.into_generic::<Vec<(usize, GMValue)>>();

                for (index, new_value) in new_values.drain(0..) {
                    self.elements[index].send_message(msgt1v(tag2.as_str(), method, new_value), object_manager);
                }
            }
            "all_objects" => {
                for element in self.elements.iter_mut() {
                    element.send_message(msgt1v(tag2.as_str(), method, value.clone()), object_manager);
                }
            }
            "all_objects2" => {
                let new_values = value.into_generic::<Vec<GMValue>>();

                for (element, new_value) in self.elements.iter_mut().zip(new_values) {
                    element.send_message(msgt1v(tag2.as_str(), method, new_value), object_manager);
                }
            }
            "size" => {
                match method {
                    "get" => {
                        let width = (self.end.x - self.start.x).abs();
                        let height = (self.end.y - self.start.y).abs();
                        let size = GMSize::new(width, height);
                        return size.into();
                    }
                    _ => {
                        error_panic(&format!("GMLine::send_message: Unknown method '{}', tag: 'size'", method));
                    }
                }
            }
            _ => {
                error_panic(&format!("GMLine::send_message: Unknown tag '{}'", tag1));
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager, context: &mut GMContext) {
        for element in self.elements.iter_mut() {
            element.update(object_manager, context);
        }
    }

    fn draw(&self, context: &mut GMContext) {
        for element in self.elements.iter() {
            element.draw(context);
        }
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}
