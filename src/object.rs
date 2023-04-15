
use std::cell::RefCell;

use crate::context::GMContext;
use crate::math::GMVec2D;

#[derive(Clone, Debug)]
pub enum GMMessage {
    AddVec2d(GMVec2D),
    AddX(f32),
    AddXY(f32, f32),
    AddY(f32),
    Draw,
    Reset,
    ToggleActive,
    ToggleVisible,
    Update,
    InGroup(String),
}

#[derive(Clone, Debug)]
pub enum GMProperty {
    Active,
    Name,
    Size,
    Text,
    Vec2D,
    Visible,
    X,
    XY,
    Y,
    ZIndex,
}

#[derive(Clone, Debug)]
pub enum GMValue {
    Bool(bool),
    String(String),
}

pub struct GMObjectManager {
    objects: Vec<RefCell<Box<dyn GMObjectT>>>,
}

impl GMObjectManager {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn GMObjectT>) {
        self.objects.push(RefCell::new(object));
    }

    pub fn add_object2<T: GMObjectT + 'static>(&mut self, object: T) {
        self.add_object(Box::new(object));
    }

    pub fn remove_object(&mut self, name: &str) {
        self.objects.retain(|o| {
            let object = o.borrow(); // TODO: use try borrow
            let maybe_name = object.get_property(GMProperty::Name);

            if let Some(GMValue::String(name2)) = maybe_name {
                name2 != name
            } else {
                true
            }
        });
    }

    pub fn update(&self, context: &mut GMContext) {
        for object in self.objects.iter() {
            let mut object = object.borrow_mut(); // TODO: use try borrow
            object.send_message(GMMessage::Update, context, &self);
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        for object in self.objects.iter() {
            let mut object = object.borrow_mut(); // TODO: use try borrow
            object.send_message(GMMessage::Draw, context, &self);
        }
    }

    pub fn sort_by_z_index(&mut self) {
        // TODO: sort vector
        todo!();
    }

    pub fn send_message(&self, name: &str, message: GMMessage, context: &mut GMContext) -> Option<GMValue> {
        for object in self.objects.iter() {
            let mut object = object.borrow_mut(); // TODO: use try borrow
            let maybe_name = object.get_property(GMProperty::Name);

            if let Some(GMValue::String(name2)) = maybe_name {
                if name2 == name {
                    return object.send_message(message, context, &self)
                }
            }
        }

        None
    }

    pub fn send_message_group(&self, group: &str, message: GMMessage, context: &mut GMContext) -> Vec<Option<GMValue>> {
        let mut result = Vec::new();

        for object in self.objects.iter() {
            let mut object = object.borrow_mut(); // TODO: use try borrow
            let maybe_in_group = object.send_message(GMMessage::InGroup(group.to_string()), context, &self);

            if let Some(GMValue::Bool(in_group)) = maybe_in_group {
                if in_group {
                    let value = object.send_message(message.clone(), context, &self);
                    result.push(value);
                }
            }
        }

        result
    }

    pub fn set_property(&self, name: &str, property: GMProperty, value: GMValue) {
        for object in self.objects.iter() {
            let mut object = object.borrow_mut(); // TODO: use try borrow
            let maybe_name = object.get_property(GMProperty::Name);

            if let Some(GMValue::String(name2)) = maybe_name {
                if name2 == name {
                    object.set_property(property, value);
                    break;
                }
            }
        }
    }

    pub fn get_property(&self, name: &str, property: GMProperty) -> Option<GMValue> {
        for object in self.objects.iter() {
            let object = object.borrow(); // TODO: use try borrow
            let maybe_name = object.get_property(GMProperty::Name);

            if let Some(GMValue::String(name2)) = maybe_name {
                if name2 == name {
                    return object.get_property(property)
                }
            }
        }

        None
    }

}


// TODO: Add pre-processing for messages: position, active, visible, ...

pub trait GMObjectT {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> Option<GMValue>;
    // TODO: send multiple messages

    fn set_property(&mut self, property: GMProperty, value: GMValue);
    // TODO: set multiple properties

    fn get_property(&self, property: GMProperty) -> Option<GMValue>;
    // TODO: get multiple properties
}
