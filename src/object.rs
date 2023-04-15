
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

    // TODO: use Into<Box<dyn GMObjectT>>

    pub fn add_object(&mut self, object: Box<dyn GMObjectT>) {
        self.objects.push(RefCell::new(object));
    }

    pub fn add_object2<T: GMObjectT + 'static>(&mut self, object: T) {
        self.add_object(Box::new(object));
    }

    pub fn replace_object(&mut self, name: &str, object: Box<dyn GMObjectT>) {
        // TODO: replace object with same name
        todo!("replace object");
    }

    pub fn replace_object2<T: GMObjectT + 'static>(&mut self, name: &str, object: T) {
        self.replace_object(name, Box::new(object));
    }

    pub fn remove_object(&mut self, name: &str) {
        self.objects.retain(|o| {
            if let Ok(object) = o.try_borrow() {
                let maybe_name = object.get_property(GMProperty::Name);

                if let Some(GMValue::String(name2)) = maybe_name {
                    name2 != name
                } else {
                    true
                }
            } else {
                false
            }
        });
    }

    pub fn update(&self, context: &mut GMContext) {
        for object in self.objects.iter() {
            if let Ok(mut object) = object.try_borrow_mut() {
                object.send_message(GMMessage::Update, context, &self);
            }
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        for object in self.objects.iter() {
            if let Ok(mut object) = object.try_borrow_mut() {
                object.send_message(GMMessage::Draw, context, &self);
            }
        }
    }

    pub fn sort_by_z_index(&mut self) {
        // TODO: sort vector
        todo!("sort by z order");
    }

    pub fn send_multi_message(&self, name: &str, messages: Vec<GMMessage>, context: &mut GMContext) -> Vec<GMValue> {
        for object in self.objects.iter() {
            if let Ok(mut object) = object.try_borrow_mut() {
                let maybe_name = object.get_property(GMProperty::Name);

                if let Some(GMValue::String(name2)) = maybe_name {
                    if name2 == name {
                        return object.send_multi_message(messages, context, &self)
                    }
                }
            }
        }

        Vec::new()
    }

    pub fn send_message(&self, name: &str, message: GMMessage, context: &mut GMContext) -> Option<GMValue> {
        let mut result = self.send_multi_message(name, vec![message], context);
        result.pop()
    }

    pub fn send_message_group(&self, group: &str, message: GMMessage, context: &mut GMContext) -> Vec<GMValue> {
        let mut result = Vec::new();

        for object in self.objects.iter() {
            if let Ok(mut object) = object.try_borrow_mut() {
                let maybe_in_group = object.send_message(GMMessage::InGroup(group.to_string()), context, &self);

                if let Some(GMValue::Bool(in_group)) = maybe_in_group {
                    if in_group {
                        if let Some(value) = object.send_message(message.clone(), context, &self) {
                            result.push(value);
                        }
                    }
                }
            }
        }

        result
    }

    // TODO: send_multi_message_group

    pub fn set_multi_property(&self, name: &str, properties: Vec<(GMProperty, GMValue)>) {
        for object in self.objects.iter() {
            if let Ok(mut object) = object.try_borrow_mut() {
                let maybe_name = object.get_property(GMProperty::Name);

                if let Some(GMValue::String(name2)) = maybe_name {
                    if name2 == name {
                        object.set_multi_property(properties);
                        break;
                    }
                }
            }
        }
    }

    pub fn set_property(&self, name: &str, property: GMProperty, value: GMValue) {
        self.set_multi_property(name, vec![(property, value)]);
    }

    pub fn get_multi_property(&self, name: &str, properties: Vec<GMProperty>) -> Vec<GMValue> {
        for object in self.objects.iter() {
            if let Ok(object) = object.try_borrow() {
                let maybe_name = object.get_property(GMProperty::Name);

                if let Some(GMValue::String(name2)) = maybe_name {
                    if name2 == name {
                        return object.get_multi_property(properties)
                    }
                }
            }
        }

        Vec::new()
    }

    pub fn get_property(&self, name: &str, property: GMProperty) -> Option<GMValue> {
        let mut result = self.get_multi_property(name, vec![property]);
        result.pop()
    }

}


// TODO: Add pre-processing for messages: position, active, visible, ...

pub trait GMObjectT {
    fn send_multi_message(&mut self, messages: Vec<GMMessage>, context: &mut GMContext, object_manager: &GMObjectManager) -> Vec<GMValue>;

    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> Option<GMValue> {
        let mut result = self.send_multi_message(vec![message], context, object_manager);
        result.pop()
    }

    fn set_multi_property(&mut self, properties: Vec<(GMProperty, GMValue)>);

    fn set_property(&mut self, property: GMProperty, value: GMValue) {
        self.set_multi_property(vec![(property, value)])
    }

    fn get_multi_property(&self, properties: Vec<GMProperty>) -> Vec<GMValue>;

    fn get_property(&self, property: GMProperty) -> Option<GMValue> {
        let mut result = self.get_multi_property(vec![property]);
        result.pop()
    }
}
