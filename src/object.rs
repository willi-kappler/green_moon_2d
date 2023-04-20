
use std::cell::RefCell;
use std::fmt::Debug;
use std::collections::{HashSet, HashMap, VecDeque};

use crate::context::GMContext;
use crate::math::GMVec2D;

#[derive(Clone, Debug)]
pub enum GMMessage {
    AddVec2D(GMVec2D),
    AddX(f32),
    AddXY(f32, f32),
    AddY(f32),
    Custom(String),
    Reset,
}

#[derive(Clone, Debug)]
pub enum GMProperty {
    Custom(String),
    Size,
    Text,
    Vec2D,
    X,
    XY,
    Y,
}

#[derive(Clone, Debug)]
pub enum GMValue {
    Bool(bool),
    F32(f32),
    F64(f64),
    I16(i16),
    I32(i32),
    I64(i64),
    I8(i8),
    String(String),
    Tuple2(Box<GMValue>, Box<GMValue>),
    Tuple3(Box<GMValue>, Box<GMValue>, Box<GMValue>),
    U16(u16),
    U32(u32),
    U64(u64),
    U8(u8),
    Vec(Vec<GMValue>),
    Vec2D(GMVec2D),
}

#[derive(Clone, Debug)]
pub enum GMObjectManagerMessage {
    AddGroup(String, String),
    AddObject(String, Box<dyn GMObjectT>),
    ClearCustomProperties(String),
    ClearGroups(String),
    RemoveCustomProperty(String, String),
    RemoveGroup(String, String),
    RemoveObject(String),
    ReplaceObject(String, Box<dyn GMObjectT>),
    SetActive(String, bool),
    SetCustomProperty(String, String, GMValue),
    SetName(String, String),
    SetVisible(String, bool),
    SetZIndex(String, i32),
    ToggleActive(String),
    ToggleVisible(String),
}

// Maybe add custom properties for objects ?

pub struct GMObjectInfo {
    active: bool,
    custom_properties: HashMap<String, GMValue>,
    groups: HashSet<String>,
    inner: RefCell<Box<dyn GMObjectT>>,
    name: String,
    visible: bool,
    z_index: i32,
}

impl GMObjectInfo {
    pub fn new<T: Into<Box<dyn GMObjectT>>>(object: T) -> Self {
        Self {
            active: true,
            custom_properties: HashMap::new(),
            groups: HashSet::new(),
            inner: RefCell::new(object.into()),
            name: "".to_string(),
            visible: true,
            z_index: 0,
        }
    }
}

pub struct GMObjectManager {
    objects: Vec<GMObjectInfo>,
    manager_messages: RefCell<VecDeque<GMObjectManagerMessage>>,
}

impl GMObjectManager {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            manager_messages: RefCell::new(VecDeque::new()),
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
            active: true,
            custom_properties: HashMap::new(),
            groups: HashSet::new(),
            inner: RefCell::new(object.into()),
            name: name.to_string(),
            visible: true,
            z_index: 0,
        };

        self.objects.push(new_object);
    }

    pub fn add_custom_object(&mut self, new_object: GMObjectInfo) {
        self.objects.push(new_object);
    }

    pub fn replace_object<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, new_object: T) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.inner.replace(new_object.into());
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
            if object.active {
                if let Ok(mut object) = object.inner.try_borrow_mut() {
                    object.update(context, &self);
                }
            }
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        for object in self.objects.iter() {
            if object.visible {
                if let Ok(object) = object.inner.try_borrow() {
                    object.draw(context);
                }
            }
        }
    }

    pub fn sort_by_z_index(&mut self) {
        self.objects.sort_by(|a, b| a.z_index.cmp(&b.z_index));
    }

    pub fn set_z_index(&mut self, name: &str, z_index: i32) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.z_index = z_index;
                break
            }
        }
    }

    pub fn get_z_index(&self, name: &str) -> i32 {
        for object in self.objects.iter() {
            if name == object.name {
                return object.z_index;
            }
        }

        return 0
    }

    pub fn set_active(&mut self, name: &str, active: bool) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.active = active;
                break
            }
        }
    }

    pub fn get_active(&self, name: &str) -> bool {
        for object in self.objects.iter() {
            if name == object.name {
                return object.active;
            }
        }

        return false
    }

    pub fn toggle_active(&mut self, name: &str) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.active = !object.active;
                break
            }
        }
    }

    pub fn set_visible(&mut self, name: &str, visible: bool) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.visible = visible;
                break
            }
        }
    }

    pub fn get_visible(&self, name: &str) -> bool {
        for object in self.objects.iter() {
            if name == object.name {
                return object.visible;
            }
        }

        return false
    }

    pub fn toggle_visible(&mut self, name: &str) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.visible = !object.visible;
                break
            }
        }
    }

    pub fn set_name(&mut self, old_name: &str, new_name: &str) {
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

    pub fn is_in_group(&self, name: &str, group: &str) -> bool {
        for object in self.objects.iter() {
            if name == object.name {
                return object.groups.contains(group);
            }
        }

        return false
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

    pub fn set_custom_property(&mut self, name: &str, key: &str, value: GMValue) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.custom_properties.insert(key.to_string(), value);
                break
            }
        }
    }

    pub fn get_custom_property(&self, name: &str, key: &str) -> Option<&GMValue> {
        for object in self.objects.iter() {
            if name == object.name {
                return object.custom_properties.get(key);
            }
        }

        return None
    }

    pub fn remove_custom_property(&mut self, name: &str, key: &str) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.custom_properties.remove(key);
                break
            }
        }
    }

    pub fn clear_custom_properties(&mut self, name: &str) {
        for object in self.objects.iter_mut() {
            if name == object.name {
                object.custom_properties.clear();
                break
            }
        }
    }

    pub fn send_message(&self, name: &str, message: &GMMessage, context: &mut GMContext) -> Option<GMValue> {
        for object in self.objects.iter() {
            if name == object.name {
                if let Ok(mut maybe_object) = object.inner.try_borrow_mut() {
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
                if let Ok(mut maybe_object) = object.inner.try_borrow_mut() {
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
                if let Ok(mut object) = object.inner.try_borrow_mut() {
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
                if let Ok(mut object) = object.inner.try_borrow_mut() {
                    object.set_property(property, value);
                    break
                }
            }
        }
    }

    pub fn set_multi_property(&self, name: &str, properties: Vec<(GMProperty, GMValue)>) {
        for object in self.objects.iter() {
            if name == object.name {
                if let Ok(mut object) = object.inner.try_borrow_mut() {
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
                if let Ok(object) = object.inner.try_borrow() {
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
                if let Ok(object) = object.inner.try_borrow() {
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
            messages.push_back(message.clone());
        }
    }

    pub fn process_manager_messages(&mut self) {
        use GMObjectManagerMessage::*;

        let mut messages = self.manager_messages.take();

        while let Some(message) = messages.pop_front() {
            match message {
                AddGroup(object_name, group) => {
                    self.add_group(&object_name, &group);
                }
                AddObject(object_name, object) => {
                    self.add_object(&object_name, object);
                }
                ClearCustomProperties(object_name) => {
                    self.clear_custom_properties(&object_name);
                }
                ClearGroups(object_name) => {
                    self.clear_groups(&object_name);
                }
                RemoveCustomProperty(object_name, key) => {
                    self.remove_custom_property(&object_name, &key);
                }
                RemoveGroup(object_name, group) => {
                    self.remove_group(&object_name, &group);
                }
                RemoveObject(object_name) => {
                    self.remove_object(&object_name);
                }
                ReplaceObject(object_name, object) => {
                    self.replace_object(&object_name, object);
                }
                SetActive(object_name, active) => {
                    self.set_active(&object_name, active);
                }
                SetCustomProperty(object_name, key, value) => {
                    self.set_custom_property(&object_name, &key, value);
                }
                SetName(object_name, name) => {
                    self.set_name(&object_name, &name);
                }
                SetVisible(object_name, visible) => {
                    self.set_visible(&object_name, visible);
                }
                SetZIndex(object_name, z_index) => {
                    self.set_z_index(&object_name, z_index);
                }
                ToggleActive(object_name) => {
                    self.toggle_active(&object_name);
                }
                ToggleVisible(object_name) => {
                    self.toggle_visible(&object_name);
                }
            }
        }
    }
}


// TODO: Add pre-processing for messages: position, active, visible, ...

pub trait GMObjectT: Debug {
    fn send_message(&mut self, _message: &GMMessage, _context: &mut GMContext, _object_manager: &GMObjectManager) -> Option<GMValue> {
        None
    }

    fn set_property(&mut self, _property: &GMProperty, _value: &GMValue) {
    }

    fn get_property(&self, _property: &GMProperty) -> Option<GMValue> {
        None
    }

    fn update(&mut self, _context: &mut GMContext, _object_manager: &GMObjectManager) {
    }

    fn draw(&self, _context: &mut GMContext) {
    }

    fn clone_box(&self) -> Box<dyn GMObjectT>;
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
