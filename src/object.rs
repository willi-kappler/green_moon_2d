
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
    AddCustomObject(String, GMObjectInfo),
    AddDrawObject(String, Box<dyn GMObjectT>, i32, i32),
    AddGroup(String, String),
    AddNormalObject(String, Box<dyn GMObjectT>, i32),
    ClearCustomProperties(String),
    ClearGroups(String),
    RemoveCustomProperty(String, String),
    RemoveGroup(String, String),
    RemoveObject(String),
    ReplaceObject(String, Box<dyn GMObjectT>),
    SetActive(String, bool),
    SetCustomProperty(String, String, GMValue),
    SetDrawIndex(String, i32),
    SetUpdateIndex(String, i32),
    SetVisible(String, bool),
    ToggleActive(String),
    ToggleVisible(String),
}

// Maybe add custom properties for objects ?

#[derive(Clone, Debug)]
pub struct GMObjectInfo {
    active: bool,
    custom_properties: HashMap<String, GMValue>,
    draw_index: i32,
    groups: HashSet<String>,
    inner: RefCell<Box<dyn GMObjectT>>,
    update_index: i32,
    visible: bool,
}

impl GMObjectInfo {
    pub fn new<T: Into<Box<dyn GMObjectT>>>(object: T) -> Self {
        Self {
            active: true,
            custom_properties: HashMap::new(),
            draw_index: 0,
            groups: HashSet::new(),
            inner: RefCell::new(object.into()),
            update_index: 0,
            visible: true,
        }
    }
}

pub struct GMObjectManager {
    objects: HashMap<String, GMObjectInfo>,
    manager_messages: RefCell<VecDeque<GMObjectManagerMessage>>,
}

impl GMObjectManager {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            manager_messages: RefCell::new(VecDeque::new()),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();

        let mut messages = self.manager_messages.borrow_mut();
        messages.clear();
    }

    pub fn add_normal_object<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, object: T, update_index: i32) {
        let new_object = GMObjectInfo {
            active: true,
            custom_properties: HashMap::new(),
            draw_index: 0,
            groups: HashSet::new(),
            inner: RefCell::new(object.into()),
            update_index: update_index,
            visible: false,
        };

        self.objects.insert(name.to_string(), new_object);
    }

    pub fn add_draw_object<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, object: T, update_index: i32, draw_index: i32) {
        let new_object = GMObjectInfo {
            active: true,
            custom_properties: HashMap::new(),
            draw_index: draw_index,
            groups: HashSet::new(),
            inner: RefCell::new(object.into()),
            update_index: update_index,
            visible: true,
        };

        self.objects.insert(name.to_string(), new_object);
    }

    pub fn add_custom_object(&mut self, name: &str, new_object: GMObjectInfo) {
        self.objects.insert(name.to_string(), new_object);
    }

    pub fn replace_object<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, new_object: T) {
        if let Some(object) = self.objects.get(name) {
            object.inner.replace(new_object.into());
        }
    }

    pub fn remove_object(&mut self, name: &str) {
        self.objects.remove(name);
    }

    pub fn update(&self, context: &mut GMContext) {
        let mut objects: Vec<&GMObjectInfo> = self.objects.values().filter(|o| o.active).collect();
        objects.sort_by(|a, b| a.update_index.cmp(&b.update_index));

        for o in objects {
            o.inner.borrow_mut().update(context, &self);
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        let mut objects: Vec<&GMObjectInfo> = self.objects.values().filter(|o| o.visible).collect();
        objects.sort_by(|a, b| a.draw_index.cmp(&b.draw_index));

        for o in objects {
            o.inner.borrow().draw(context);
        }
    }

    pub fn set_draw_index(&mut self, name: &str, draw_index: i32) {
        if let Some(object) = self.objects.get_mut(name) {
            object.draw_index = draw_index;
        }
    }

    pub fn get_draw_index(&self, name: &str) -> i32 {
        if let Some(object) = self.objects.get(name) {
            return object.draw_index;
        }

        0
    }

    pub fn set_update_index(&mut self, name: &str, update_index: i32) {
        if let Some(object) = self.objects.get_mut(name) {
            object.update_index = update_index;
        }
    }

    pub fn get_update_index(&self, name: &str) -> i32 {
        if let Some(object) = self.objects.get(name) {
            return object.update_index;
        }

        0
    }

    pub fn set_active(&mut self, name: &str, active: bool) {
        if let Some(object) = self.objects.get_mut(name) {
            object.active = active;
        }
    }

    pub fn get_active(&self, name: &str) -> bool {
        if let Some(object) = self.objects.get(name) {
            return object.active;
        }

        false
    }

    pub fn toggle_active(&mut self, name: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.active = !object.active;
        }
    }

    pub fn set_visible(&mut self, name: &str, visible: bool) {
        if let Some(object) = self.objects.get_mut(name) {
            object.visible = visible;
        }
    }

    pub fn get_visible(&self, name: &str) -> bool {
        if let Some(object) = self.objects.get(name) {
            return object.visible;
        }

        false
    }

    pub fn toggle_visible(&mut self, name: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.visible = !object.visible;
        }
    }

    pub fn add_group(&mut self, name: &str, group: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.groups.insert(group.to_string());
        }
    }

    pub fn is_in_group(&self, name: &str, group: &str) -> bool {
        if let Some(object) = self.objects.get(name) {
            return object.groups.contains(group);
        }

        false
    }

    pub fn remove_group(&mut self, name: &str, group: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.groups.remove(group);
        }
    }

    pub fn clear_groups(&mut self, name: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.groups.clear();
        }
    }

    pub fn set_custom_property(&mut self, name: &str, key: &str, value: GMValue) {
        if let Some(object) = self.objects.get_mut(name) {
            object.custom_properties.insert(key.to_string(), value);
        }
    }

    pub fn get_custom_property(&self, name: &str, key: &str) -> Option<&GMValue> {
        if let Some(object) = self.objects.get(name) {
            return object.custom_properties.get(key);
        }

        return None
    }

    pub fn remove_custom_property(&mut self, name: &str, key: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.custom_properties.remove(key);
        }
    }

    pub fn clear_custom_properties(&mut self, name: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.custom_properties.clear();
        }
    }

    pub fn send_message(&self, name: &str, message: &GMMessage, context: &mut GMContext) -> Option<GMValue> {
        if let Some(object) = self.objects.get(name) {
            let mut borrowed_object = object.inner.borrow_mut();
            return borrowed_object.send_message(message, context, &self);
        }

        None
    }

    pub fn send_multi_message(&self, name: &str, messages: Vec<GMMessage>, context: &mut GMContext) -> Vec<Option<GMValue>> {
        let mut result = Vec::new();

        if let Some(object) = self.objects.get(name) {
            let mut borrowed_object = object.inner.borrow_mut();

            for message in messages {
                result.push(borrowed_object.send_message(&message, context, &self));
            }
        }

        result
    }

    pub fn send_message_group(&self, group: &str, message: &GMMessage, context: &mut GMContext) -> Vec<(String, Option<GMValue>)> {
        let mut result = Vec::new();

        for (name, object) in self.objects.iter() {
            if object.groups.contains(group) {
                let mut borrowed_object = object.inner.borrow_mut();
                result.push((name.clone(), borrowed_object.send_message(message, context, &self)));
            }
        }

        result
    }

    // TODO: send_multi_message_group

    pub fn set_property(&self, name: &str, property: &GMProperty, value: &GMValue) {
        if let Some(object) = self.objects.get(name) {
            let mut borrowed_object = object.inner.borrow_mut();
            borrowed_object.set_property(property, value);
        }
    }

    pub fn set_multi_property(&self, name: &str, properties: Vec<(GMProperty, GMValue)>) {
        if let Some(object) = self.objects.get(name) {
            let mut borrowed_object = object.inner.borrow_mut();
            for (property, value) in properties.iter() {
                borrowed_object.set_property(property, value);
            }
        }
    }

    pub fn get_property(&self, name: &str, property: &GMProperty) -> Option<GMValue> {
        if let Some(object) = self.objects.get(name) {
            let borrowed_object = object.inner.borrow();
            return borrowed_object.get_property(property)
        }

        None
    }

    pub fn get_multi_property(&self, name: &str, properties: Vec<GMProperty>) -> Vec<Option<GMValue>> {
        let mut result = Vec::new();

        if let Some(object) = self.objects.get(name) {
            let borrowed_object = object.inner.borrow();

            for property in properties.iter() {
                result.push(borrowed_object.get_property(property));
            }
        }

        result
    }

    pub fn send_manager_message(&self, message: &GMObjectManagerMessage) {
        let mut messages = self.manager_messages.borrow_mut();
        messages.push_back(message.clone());
    }

    pub fn process_manager_messages(&mut self) {
        use GMObjectManagerMessage::*;

        let mut messages = self.manager_messages.take();

        while let Some(message) = messages.pop_front() {
            match message {
                AddCustomObject(object_name, object_info) => {
                    self.add_custom_object(&object_name, object_info);
                }
                AddDrawObject(object_name, object, update_index , draw_index) => {
                    self.add_draw_object(&object_name, object, update_index, draw_index);
                }
                AddGroup(object_name, group) => {
                    self.add_group(&object_name, &group);
                }
                AddNormalObject(object_name, object, update_index) => {
                    self.add_normal_object(&object_name, object, update_index);
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
                SetDrawIndex(object_name, draw_index) => {
                    self.set_draw_index(&object_name, draw_index);
                }
                SetUpdateIndex(object_name, update_index) => {
                    self.set_update_index(&object_name, update_index);
                }
                SetVisible(object_name, visible) => {
                    self.set_visible(&object_name, visible);
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
