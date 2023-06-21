

use crate::object::GMObjectT;
use crate::message::{GMMessage, msg0v, msg_set_position};
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
    pub top_left: Box<dyn GMObjectT>,
    pub top: GMLine,
    pub top_right: Box<dyn GMObjectT>,
    pub right: GMLine,
    pub bottom_right: Box<dyn GMObjectT>,
    pub bottom: GMLine,
    pub bottom_left: Box<dyn GMObjectT>,
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
        let top_left = object.clone();
        let top = GMLine::new((x1 + width, y1), (x2 - width, y1), object.clone(), GMLineMode::Spacing(width));
        let top_right = object.clone();
        let right = GMLine::new((x2, y1 + height), (x2, y2 - height), object.clone(), GMLineMode::Spacing(height));
        let bottom_right = object.clone();
        let bottom = GMLine::new((x1 + width, y2), (x2 - width, y2), object.clone(), GMLineMode::Spacing(width));
        let bottom_left = object.clone();
        let left = GMLine::new((x1, y1 + height), (x1, y2 - height), object, GMLineMode::Spacing(height));


        Self {
            base,
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

    pub fn set_object(&mut self, object: Box<dyn GMObjectT>) {
        self.top_left = object.clone();
        self.top.init_element = object.clone();
        self.top_right = object.clone();
        self.right.init_element = object.clone();
        self.bottom_right = object.clone();
        self.bottom.init_element = object.clone();
        self.bottom_left = object.clone();
        self.left.init_element = object;
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

    pub fn init_objects(&mut self, object_manager: &GMObjectManager) {
        let get_width = msg0v("get_width");
        let get_height = msg0v("get_height");

        let w1 = self.top_left.send_message(get_width.clone(), object_manager).into_f32();
        let w2 = self.top.init_element.send_message(get_width.clone(), object_manager).into_f32();
        let w3 = self.top_right.send_message(get_width.clone(), object_manager).into_f32();
        let w4 = self.bottom.init_element.send_message(get_width, object_manager).into_f32();

        let x1 = self.base.rectangle.top_left.x;
        let x2 = x1 + w1;
        let x4 = self.base.rectangle.bottom_right.x - w3;
        let x3 = x4 - w2;
        // let x5 = x4 - w4;


        let h1 = self.top_left.send_message(get_height.clone(), object_manager).into_f32();
        let h2 = self.left.init_element.send_message(get_height.clone(), object_manager).into_f32();
        let h3 = self.bottom_left.send_message(get_height.clone(), object_manager).into_f32();
        let h4 = self.right.init_element.send_message(get_height.clone(), object_manager).into_f32();

        let y1 = self.base.rectangle.top_left.y;
        let y2 = y1 + h1;
        let y4 = self.base.rectangle.bottom_right.y - h3;
        let y3 = y4 - h2;
        // let y5 = y4 - h4;

        self.top_left.send_message(msg_set_position((x1, y1)), object_manager);
        self.top.start.x = x2;
        self.top.start.y = y1;
        self.top.end.x = x3;
        self.top.end.y = y1;
        self.top_right.send_message(msg_set_position((x4, y1)), object_manager);

        self.right.start.x = x4;
        self.right.start.y = y2;
        self.right.end.x = x4;
        self.right.end.y = y3;

        self.bottom_left.send_message(msg_set_position((x1, y3)), object_manager);
        self.bottom.start.x = x2;
        self.bottom.start.y = y3;
        self.bottom.end.x = x3;
        self.bottom.end.y = y3;
        self.bottom_right.send_message(msg_set_position((x4, y3)), object_manager);

        self.left.start.x = x1;
        self.left.start.y = y2;
        self.left.end.x = x1;
        self.left.end.y = y3;

        self.top.set_spacing(w2);
        self.right.set_spacing(h4);
        self.bottom.set_spacing(w4);
        self.left.set_spacing(h2);

    }
}

impl GMObjectT for GMBorderSimple {
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
                    "get_width" => {
                        return self.base.rectangle.get_width().into();
                    }
                    "get_height" => {
                        return self.base.rectangle.get_height().into();
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
                return self.base.send_message(message, object_manager);
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
