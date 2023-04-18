
use std::cell::RefCell;
use std::fmt::Debug;
use std::collections::HashSet;

use crate::context::GMContext;
use crate::math::GMVec2D;

#[derive(Clone, Debug)]
pub enum GMMessage {
    AddVec2d(GMVec2D),
    AddX(f32),
    AddXY(f32, f32),
    AddY(f32),
    Custom(String),
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
    Custom(String),
    Size,
    Text,
    Vec2D,
    Visible,
    X,
    XY,
    Y,
}

#[derive(Clone, Debug)]
pub enum GMValue {
    Bool(bool),
    String(String),
}

#[derive(Clone, Debug)]
pub enum GMObjectManagerMessage {
    AddObject(String, Box<dyn GMObjectT>),
    ChangeName(String, String),
    ChangeZIndex(String, i32),
    RemoveObject(String),
    ReplaceObject(String, Box<dyn GMObjectT>),
    AddGroup(String, String),
    RemoveGroup(String, String),
    ClearGroups(String),
}

struct GMObjectInfo {
    z_index: i32,
    name: String,
    groups: HashSet<String>,
    object: RefCell<Box<dyn GMObjectT>>,
}

pub struct GMObjectManager {
    objects: Vec<GMObjectInfo>,
    manager_messages: RefCell<Vec<GMObjectManagerMessage>>,
}

impl GMObjectManager {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            manager_messages: RefCell::new(Vec::new()),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();

        if let Ok(mut messages) = self.manager_messages.try_borrow_mut() {
            messages.clear();
        }
    }

    pub fn add_object<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, object: T) {
        let new_object = GMObjectInfo {
            z_index: 0,
            name: name.to_string(),
            groups: HashSet::new(),
            object: RefCell::new(object.into()),
        };

        self.objects.push(new_object);
    }

    pub fn replace_object<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, new_object: T) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.object.replace(new_object.into());
                break
            }
        }
    }

    pub fn remove_object(&mut self, name: &str) {
        self.objects.retain(|o| {
            name != o.name
        });
    }

    pub fn update(&self, context: &mut GMContext) {
        for object in self.objects.iter() {
            if let Ok(mut object) = object.object.try_borrow_mut() {
                object.update(context, &self);
            }
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        for object in self.objects.iter() {
            if let Ok(mut object) = object.object.try_borrow_mut() {
                object.draw(context, &self);
            }
        }
    }

    pub fn sort_by_z_index(&mut self) {
        // TODO: sort vector
        todo!("sort by z order");
    }

    pub fn change_z_index(&mut self, name: &str, z_index: i32) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.z_index = z_index;
                break
            }
        }
    }

    pub fn change_name(&mut self, old_name: &str, new_name: &str) {
        for object in self.objects.iter_mut() {
            if old_name == object.name {
                object.name = new_name.to_string();
                break
            }
        }
    }

    pub fn add_group(&mut self, name: &str, group: &str) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.groups.insert(group.to_string());
            }
        }
    }

    pub fn remove_group(&mut self, name: &str, group: &str) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.groups.remove(group);
            }
        }
    }

    pub fn clear_groups(&mut self, name: &str) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.groups.clear();
            }
        }
    }

    pub fn send_message(&self, name: &str, message: &GMMessage, context: &mut GMContext) -> Option<GMValue> {
        for object in self.objects.iter() {
            if name == object.name {
                if let Ok(mut maybe_object) = object.object.try_borrow_mut() {
                    return maybe_object.send_message(message, context, &self)
                }
            }
        }

        None
    }

    pub fn send_multi_message(&self, name: &str, messages: Vec<GMMessage>, context: &mut GMContext) -> Vec<Option<GMValue>> {
        let mut result = Vec::new();

        for object in self.objects.iter() {
            if name == object.name {
                if let Ok(mut maybe_object) = object.object.try_borrow_mut() {
                    for message in messages.iter() {
                        result.push(maybe_object.send_message(message, context, &self));
                    }
                }
            }
        }

        result
    }

    pub fn send_message_group(&self, group: &str, message: &GMMessage, context: &mut GMContext) -> Vec<Option<GMValue>> {
        let mut result = Vec::new();

        for object in self.objects.iter() {
            if object.groups.contains(group) {
                if let Ok(mut object) = object.object.try_borrow_mut() {
                    result.push(object.send_message(message, context, &self));
                }
            }
        }

        result
    }

    // TODO: send_multi_message_group

    pub fn set_property(&self, name: &str, property: &GMProperty, value: &GMValue) {
        for object in self.objects.iter() {
            if name == object.name {
                if let Ok(mut object) = object.object.try_borrow_mut() {
                    object.set_property(property, value);
                    break
                }
            }
        }
    }

    pub fn set_multi_property(&self, name: &str, properties: Vec<(GMProperty, GMValue)>) {
        for object in self.objects.iter() {
            if name == object.name {
                if let Ok(mut object) = object.object.try_borrow_mut() {
                    for (property, value) in properties.iter() {
                        object.set_property(property, value);
                    }
                    break       
                }
            }
        }
    }

    pub fn get_property(&self, name: &str, property: &GMProperty) -> Option<GMValue> {
        for object in self.objects.iter() {
            if name == object.name {
                if let Ok(object) = object.object.try_borrow() {
                    return object.get_property(property)
                }
            }
        }

        None
    }

    pub fn get_multi_property(&self, name: &str, properties: Vec<GMProperty>) -> Vec<Option<GMValue>> {
        let mut result = Vec::new();

        for object in self.objects.iter() {
            if name == object.name {
                if let Ok(object) = object.object.try_borrow() {
                    for property in properties.iter() {
                        result.push(object.get_property(property));
                    }
                }
            }
        }

        result
    }

    pub fn send_manager_message(&self, message: &GMObjectManagerMessage) {
        if let Ok(mut messages) = self.manager_messages.try_borrow_mut() {
            messages.push(message.clone());
        }
    }

    pub fn process_manager_messages(&mut self) {
        if let Ok(mut messages) = self.manager_messages.try_borrow_mut() {
            for message in messages.iter() {
                match message {
                    _ => {
                        // TODO: process message
                        println!("{:?}", message);
                    }
                }
            }

            messages.clear();
        }
    }
}


// TODO: Add pre-processing for messages: position, active, visible, ...

pub trait GMObjectT: Debug {
    fn send_message(&mut self, message: &GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> Option<GMValue>;

    fn set_property(&mut self, property: &GMProperty, value: &GMValue);

    fn get_property(&self, property: &GMProperty) -> Option<GMValue>;

    fn clone_box(&self) -> Box<dyn GMObjectT>;

    fn update(&mut self, context: &mut GMContext, object_manager: &GMObjectManager);

    fn draw(&mut self, context: &mut GMContext, object_manager: &GMObjectManager);
}


impl Clone for Box<dyn GMObjectT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<U: GMObjectT + 'static> From<U> for Box<dyn GMObjectT> {
    fn from(object: U) -> Self {
        Box::new(object)
    }
}

impl From<&dyn GMObjectT> for Box<dyn GMObjectT> {
    fn from(object: &dyn GMObjectT) -> Self {
        object.clone_box()
    }
}
