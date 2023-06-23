

use crate::object::GMObjectT;
use crate::message::{GMMessage, msgt0v, msg_set_position};
use crate::value::GMValue;
use crate::object_manager::GMObjectManager;
use crate::context::GMContext;
use crate::math::{GMVec2D, GMRectangle};
use crate::line::{GMLine, GMLineMode};
use crate::util::error_panic;

#[derive(Debug, Clone)]
pub struct GMBorder {
    pub rectangle: GMRectangle,
    pub top_left: Box<dyn GMObjectT>,
    pub top: GMLine,
    pub top_right: Box<dyn GMObjectT>,
    pub right: GMLine,
    pub bottom_right: Box<dyn GMObjectT>,
    pub bottom: GMLine,
    pub bottom_left: Box<dyn GMObjectT>,
    pub left: GMLine,

}

impl GMBorder {
    pub fn new<U: Into<GMVec2D>, V: Into<GMVec2D>, O: Into<Box<dyn GMObjectT>>>(top_left: U, bottom_right: V, object: O) -> Self {
        let rectangle = GMRectangle::new4(top_left, bottom_right);

        let object = object.into();
        let top_left = object.clone();
        let top = GMLine::new((0.0, 0.0), (1.0, 1.0), object.clone(), GMLineMode::Number(1));
        let top_right = object.clone();
        let right = GMLine::new((0.0, 0.0), (1.0, 1.0), object.clone(), GMLineMode::Number(1));
        let bottom_right = object.clone();
        let bottom = GMLine::new((0.0, 0.0), (1.0, 1.0), object.clone(), GMLineMode::Number(1));
        let bottom_left = object.clone();
        let left = GMLine::new((0.0, 0.0), (1.0, 1.0), object, GMLineMode::Number(1));


        Self {
            rectangle,
            top_left,
            top,
            top_right,
            right,
            bottom_right,
            bottom,
            bottom_left,
            left,

        }
    }

    pub fn new2<U: Into<GMVec2D>, O: Into<Box<dyn GMObjectT>>>(top_left: U, width: f32, height: f32, object: O) -> Self {
        let top_left = top_left.into();
        let bottom_left = GMVec2D::new(top_left.x + width, top_left.y + height);
        Self::new(top_left, bottom_left, object)
    }

    pub fn set_object<O: Into<Box<dyn GMObjectT>>>(&mut self, object: O) {
        let object = object.into();

        self.top_left = object.clone();
        self.top.init_element = object.clone();
        self.top_right = object.clone();
        self.right.init_element = object.clone();
        self.bottom_right = object.clone();
        self.bottom.init_element = object.clone();
        self.bottom_left = object.clone();
        self.left.init_element = object;
    }

    pub fn set_2_objects<U: Into<Box<dyn GMObjectT>>, V: Into<Box<dyn GMObjectT>>>(&mut self, corner: U, side: V) {
        let corner = corner.into();
        let side = side.into();

        self.top_left = corner.clone();
        self.top.init_element = side.clone();
        self.top_right = corner.clone();
        self.right.init_element = side.clone();
        self.bottom_right = corner.clone();
        self.bottom.init_element = side.clone();
        self.bottom_left = corner.clone();
        self.left.init_element = side;
    }

    pub fn set_4_objects(&mut self, top: Box<dyn GMObjectT>, right: Box<dyn GMObjectT>, bottom: Box<dyn GMObjectT>, left: Box<dyn GMObjectT>) {
        self.top_left = top.clone();
        self.top.init_element = top.clone();
        self.top_right = top;
        self.right.init_element = right;
        self.bottom_right = bottom.clone();
        self.bottom.init_element = bottom.clone();
        self.bottom_left = bottom;
        self.left.init_element = left;
    }

    pub fn set_8_objects(&mut self, mut objects: Vec<Box<dyn GMObjectT>>) {
        assert_eq!(objects.len(), 8);

        let mut drained = objects.drain(0..);

        self.top_left = drained.next().unwrap();
        self.top.init_element = drained.next().unwrap();
        self.top_right = drained.next().unwrap();
        self.right.init_element = drained.next().unwrap();
        self.bottom_right = drained.next().unwrap();
        self.bottom.init_element = drained.next().unwrap();
        self.bottom_left = drained.next().unwrap();
        self.left.init_element = drained.next().unwrap();

    }

    pub fn set_corners<O: Into<Box<dyn GMObjectT>>>(&mut self, object: O) {
        let object = object.into();

        self.top_left = object.clone();
        self.top_right = object.clone();
        self.bottom_right = object.clone();
        self.bottom_left = object.clone();
    }

    pub fn set_sides<O: Into<Box<dyn GMObjectT>>>(&mut self, object: O) {
        let object = object.into();

        self.top.init_element = object.clone();
        self.right.init_element = object.clone();
        self.bottom.init_element = object.clone();
        self.left.init_element = object.clone();
    }

    pub fn init_objects(&mut self, object_manager: &GMObjectManager) {
        // All objects must have the same width and height
        let size = self.top_left.send_message(msgt0v("size", "get"), object_manager).into_size();
        let width = size.width;
        let height = size.height;

        let x1 = self.rectangle.top_left.x;
        let x3 = self.rectangle.bottom_right.x;
        let nw = ((x3 - x1) / width).floor();
        let dx = (x3 - x1) / nw;
        let x2 = x1 + dx;

        let y1 = self.rectangle.top_left.y;
        let y3 = self.rectangle.bottom_right.y;
        let nh = ((y3 - y1) / height).floor();
        let dy = (y3 - y1) / nh;
        let y2 = y1 + dy;

        self.top_left.send_message(msg_set_position((x1, y1)), object_manager);
        self.top.start.x = x2;
        self.top.start.y = y1;
        self.top.end.x = x3;
        self.top.end.y = y1;
        self.top_right.send_message(msg_set_position((x3, y1)), object_manager);

        self.right.start.x = x3;
        self.right.start.y = y2;
        self.right.end.x = x3;
        self.right.end.y = y3;

        self.bottom_left.send_message(msg_set_position((x1, y3)), object_manager);
        self.bottom.start.x = x2;
        self.bottom.start.y = y3;
        self.bottom.end.x = x3;
        self.bottom.end.y = y3;
        self.bottom_right.send_message(msg_set_position((x3, y3)), object_manager);

        self.left.start.x = x1;
        self.left.start.y = y2;
        self.left.end.x = x1;
        self.left.end.y = y3;

        self.top.set_number2((nw as u32) - 1, object_manager);
        self.right.set_number2((nh as u32) - 1, object_manager);
        self.bottom.set_number2((nw as u32) - 1, object_manager);
        self.left.set_number2((nh as u32) - 1, object_manager);

    }
}

impl GMObjectT for GMBorder {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "" => {
                match method {
                    "init" => {
                        self.init_objects(object_manager);
                    }
                    "set_object" => {
                        let object = message.value.into_object();
                        self.set_object(object);
                        self.init_objects(object_manager);
                    }
                    "set_4_objects" => {
                        let (top, right, bottom, left) =
                            message.value.into_generic::<(Box<dyn GMObjectT>, Box<dyn GMObjectT>, Box<dyn GMObjectT>, Box<dyn GMObjectT>)>();
                        self.set_4_objects(top, right, bottom, left);
                        self.init_objects(object_manager);
                    }
                    "set_8_objects" => {
                        let objects = message.value.into_generic::<Vec<Box<dyn GMObjectT>>>();
                        self.set_8_objects(objects);
                        self.init_objects(object_manager);
                    }
                    "set_top_left" => {
                        let object = message.value.into_object();
                        self.top_left = object;
                    }
                    "set_top" => {
                        let object = message.value.into_object();
                        self.top.init_element = object;
                    }
                    "set_top_right" => {
                        let object = message.value.into_object();
                        self.top_right = object;
                    }
                    "set_right" => {
                        let object = message.value.into_object();
                        self.right.init_element = object;
                    }
                    "set_bottom_right" => {
                        let object = message.value.into_object();
                        self.bottom_right = object;
                    }
                    "set_bottom" => {
                        let object = message.value.into_object();
                        self.bottom.init_element = object;
                    }
                    "set_bottom_left" => {
                        let object = message.value.into_object();
                        self.bottom_left = object;
                    }
                    "set_left" => {
                        let object = message.value.into_object();
                        self.left.init_element = object;
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
            "top_left" => {
                return self.top_left.send_message(message, object_manager);
            }
            "top_right" => {
                return self.top_right.send_message(message, object_manager);
            }
            "bottom_left" => {
                return self.bottom_left.send_message(message, object_manager);
            }
            "bottom_right" => {
                return self.bottom_right.send_message(message, object_manager);
            }
            _ => {
                message.pre_tag(tag);
                return self.rectangle.send_message(message);
            }
        }

        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager, context: &mut GMContext) {
        self.top_left.update(object_manager, context);
        self.top.update(object_manager, context);
        self.top_right.update(object_manager, context);
        self.right.update(object_manager, context);
        self.bottom_right.update(object_manager, context);
        self.bottom.update(object_manager, context);
        self.bottom_left.update(object_manager, context);
        self.left.update(object_manager, context);
    }

    fn draw(&self, context: &mut GMContext) {
        self.top_left.draw(context);
        self.top.draw(context);
        self.top_right.draw(context);
        self.right.draw(context);
        self.bottom_right.draw(context);
        self.bottom.draw(context);
        self.bottom_left.draw(context);
        self.left.draw(context);
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
