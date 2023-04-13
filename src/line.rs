
use std::fmt::Debug;

use crate::math::GMVec2D;
use crate::util::{GMActiveT, GMVisibleT, GMDrawT, GMUpdateT};
use crate::context::GMContext;
use crate::movement::{GMPositionT, GMPositionMultipleT};

use crate::{gen_impl_active, gen_impl_visible};

#[derive(Debug, Clone)]
pub enum GMLineMode {
    Number(u32),
    Spacing(f32),
}

pub trait GMLineT: GMActiveT + GMVisibleT + GMDrawT + GMUpdateT + GMPositionT + Debug {
    fn clone_box(&self) -> Box<dyn GMLineT>;
}

impl Clone for Box<dyn GMLineT> {
    fn clone(&self) -> Box<dyn GMLineT> {
        self.clone_box()
    }
}

impl<U: GMLineT + 'static> From<U> for Box<dyn GMLineT> {
    fn from(line: U) -> Self {
        Box::new(line)
    }
}

impl From<&dyn GMLineT> for Box<dyn GMLineT> {
    fn from(line: &dyn GMLineT) -> Self {
        line.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct GMLine {
    pub start: GMVec2D,
    pub end: GMVec2D,
    pub init_element: Box<dyn GMLineT>,
    pub elements: Vec<Box<dyn GMLineT>>,
    pub line_mode: GMLineMode,
    pub active: bool,
    pub visible: bool,
}

impl GMLine {
    pub fn new<T: Into<GMVec2D>, U: Into<GMVec2D>, V: Into<Box<dyn GMLineT>>>(start: T, end: U, init_element: V, line_mode: GMLineMode) -> Self {
        let mut line = Self {
            start: start.into(),
            end: end.into(),
            init_element: init_element.into(),
            elements: Vec::new(),
            line_mode,
            active: true,
            visible: true,
        };

        line.end_point_changed();

        line
    }

    pub fn set_start<V: Into<GMVec2D>>(&mut self, start: V) {
        self.start = start.into();
    }

    pub fn set_start2<V: Into<GMVec2D>>(&mut self, start: V) {
        self.start = start.into();
        self.end_point_changed();
    }

    pub fn set_end<V: Into<GMVec2D>>(&mut self, end: V) {
        self.end = end.into();
    }

    pub fn set_end2<V: Into<GMVec2D>>(&mut self, end: V) {
        self.end = end.into();
        self.end_point_changed();
    }

    pub fn end_point_changed(&mut self) {
        let direction = self.end - self.start;
        let length = direction.len();

        match self.line_mode {
            GMLineMode::Number(number) => {
                let spacing = length / (number as f32);
                self.set_elements(number, spacing, direction);
            }
            GMLineMode::Spacing(spacing) => {
                let number = (length / spacing).floor() as u32;
                self.set_elements(number, spacing, direction);
            }
        }
    }

    pub fn set_number(&mut self, number: u32) {
        self.line_mode = GMLineMode::Number(number);

        let direction = self.end - self.start;
        let length = direction.len();
        let spacing = length / (number as f32);

        self.set_elements(number, spacing, direction);
    }

    pub fn set_spacing(&mut self, spacing: f32) {
        self.line_mode = GMLineMode::Spacing(spacing);

        let direction = self.end - self.start;
        let length = direction.len();
        let number = (length / spacing).floor() as u32;

        self.set_elements(number, spacing, direction);
    }

    pub fn set_elements(&mut self, number: u32, spacing: f32, mut direction: GMVec2D) {
        direction.norm();

        // If more elements are needed just add them
        let diff = ((number as i32) - (self.elements.len() as i32)) as i32;

        for _ in 0..diff {
            self.elements.push(self.init_element.clone());
        }

        // Now re-calculate the positions of all elements, and disable the ones that are not needed.
        for i in 0..self.elements.len() {
            let element = &mut self.elements[i];

            if i <= (number as usize) {
                let new_position = self.start + (direction * (spacing * (i as f32)));
                element.set_position_vec2d(new_position);
                element.set_active(true);
                element.set_visible(true);
            } else {
                element.set_active(false);
                element.set_visible(false);
            }
        }
    }

    pub fn get_elements(&self) -> &Vec<Box<dyn GMLineT>> {
        &self.elements
    }

    pub fn get_elements_mut(&mut self) -> &mut Vec<Box<dyn GMLineT>> {
        &mut self.elements
    }
}

gen_impl_active!(GMLine);

gen_impl_visible!(GMLine);

impl GMUpdateT for GMLine {
    fn update(&mut self) {
        if self.active {
            for element in &mut self.elements {
                element.update();
            }
        }
    }

    fn update2(&mut self, _context: &mut GMContext) {
        self.update()
    }
}

impl GMDrawT for GMLine {
    fn draw(&self, context: &mut GMContext) {
        if self.visible {
            for element in &self.elements {
                element.draw(context);
            }
        }
    }
}

impl GMPositionMultipleT for GMLine {
    fn set_position_x_n(&mut self, x: f32, index: usize) {
        if index == 0 {
            self.start.x = x;
        } else {
            self.end.x = x;
        }
    }

    fn set_position_y_n(&mut self, y: f32, index: usize) {
        if index == 0 {
            self.start.y = y;
        } else {
            self.end.y = y;

        }
    }

    fn get_position_x_n(&self, index: usize) -> f32 {
        if index == 0 {
            self.start.x
        } else {
            self.end.x
        }
    }

    fn get_position_y_n(&self, index: usize) -> f32 {
        if index == 0 {
            self.start.y
        } else {
            self.end.y
        }
    }
}
