

use crate::object::GMObjectT;
use crate::message::GMMessage;
use crate::value::GMValue;
use crate::object_manager::GMObjectManager;
use crate::context::GMContext;
use crate::math::{GMVec2D, GMRectangle};
use crate::line::{GMLine, GMLineMode};
use crate::util::error_panic;


#[derive(Debug, Clone)]
pub struct GMBorderBase {
    pub rectangle: GMRectangle,
}

impl GMObjectT for GMBorderBase {
    fn send_message(&mut self, message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        self.rectangle.send_message(message)
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct GMBorderSimple {
    pub base: GMBorderBase,
    pub top: GMLine,
    pub right: GMLine,
    pub bottom: GMLine,
    pub left: GMLine,
}

impl GMBorderSimple {
    pub fn new<U: Into<GMVec2D>, V: Into<GMVec2D>>(top_left: U, bottom_right: V, object: Box<dyn GMObjectT>) -> Self {
        let base = GMBorderBase {
            rectangle: GMRectangle::new4(top_left, bottom_right),
        };

        let x1 = base.rectangle.top_left.x;
        let y1 = base.rectangle.top_left.y;

        let x2 = base.rectangle.bottom_right.x;
        let y2 = base.rectangle.bottom_right.y;

        // TODO: Get width and height from object
        let width = 16.0;
        let height = 16.0;

        // TODO: Make left and right line shorter, they overlap with top and bottom line
        let top = GMLine::new((x1, y1), (x2, y1), object.clone(), GMLineMode::Spacing(width));
        let right = GMLine::new((x2, y1 + height), (x2, y2 - height), object.clone(), GMLineMode::Spacing(height));
        let bottom = GMLine::new((x1, y2), (x2, y2), object.clone(), GMLineMode::Spacing(width));
        let left = GMLine::new((x1, y1 + height), (x1, y2 - height), object.clone(), GMLineMode::Spacing(height));

        Self {
            base,
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn set_object(&mut self, object: Box<dyn GMObjectT>) {
        self.top.init_element = object.clone();
        self.right.init_element = object.clone();
        self.bottom.init_element = object.clone();
        self.left.init_element = object;
    }

    pub fn set_4_objects(&mut self, top: Box<dyn GMObjectT>, right: Box<dyn GMObjectT>, bottom: Box<dyn GMObjectT>, left: Box<dyn GMObjectT>) {
        self.top.init_element = top;
        self.right.init_element = right;
        self.bottom.init_element = bottom;
        self.left.init_element = left;
    }

    pub fn set_8_objects(&mut self, objects: &[Box<dyn GMObjectT>]) {
        // TODO: Implement corner and sides
    }
}

impl GMObjectT for GMBorderSimple {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "" => {
                match method {
                    "set_object" => {
                        let object = message.value.into_object();
                        self.set_object(object);
                    }
                    "set_4_objects" => {
                        // TODO:
                    }
                    "set_8_objects" => {
                        // TODO:
                    }
                    _ => {
                        error_panic(&format!("GMBorderSimple::send_message: Unknown method '{}', no tag", method));
                    }    
                }
            }
            "top" => {
                return self.top.send_message(message, object_manager);
            }
            "right" => {
                return self.right.send_message(message, object_manager);
            }
            "bottom" => {
                return self.bottom.send_message(message, object_manager);
            }
            "left" => {
                return self.left.send_message(message, object_manager);
            }
            _ => {
                return self.base.send_message(message, object_manager);
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager, context: &mut GMContext) {
        self.top.update(object_manager, context);
        self.right.update(object_manager, context);
        self.bottom.update(object_manager, context);
        self.left.update(object_manager, context);
    }

    fn draw(&self, context: &mut GMContext) {
        self.top.draw(context);
        self.right.draw(context);
        self.bottom.draw(context);
        self.left.draw(context);
    }


    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
