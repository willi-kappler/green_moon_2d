
use std::cell::RefCell;
use std::fmt::Debug;
use std::collections::{HashSet, HashMap, VecDeque};
use std::any::Any;
use std::rc::Rc;

use crate::context::GMContext;
use crate::math::{GMVec2D, GMSize};
use crate::util::{error_panic};

#[derive(Clone, Debug)]
pub enum GMMessage {
    AddPosition(GMVec2D),
    AddX(f32),
    AddY(f32),
    Custom1(String),
    Custom2(String, GMValue),
    GetChild(String),
    GetChildCount,
    GetCustom(String),
    GetMessage,
    GetPosition,
    GetSize,
    GetTarget,
    GetX,
    GetY,
    Multiple(Vec<GMMessage>),
    OMAddCustomObject(String, GMObjectInfo),
    OMAddDrawObject(String, Box<dyn GMObjectT>, i32, i32),
    OMAddGroup(String, String),
    OMAddNormalObject(String, Box<dyn GMObjectT>, i32),
    OMClearCustomProperties(String),
    OMClearGroups(String),
    OMRemoveCustomProperty(String, String),
    OMRemoveGroup(String, String),
    OMRemoveObject(String),
    OMReplaceObject(String, Box<dyn GMObjectT>),
    OMSetActive(String, bool),
    OMSetCustomProperty(String, String, GMValue),
    OMSetDrawIndex(String, i32),
    OMSetUpdateIndex(String, i32),
    OMSetVisible(String, bool),
    OMToggleActive(String),
    OMToggleVisible(String),
    Reset,
    SetChild(String, Box<dyn GMObjectT>),
    SetCustom(String, GMValue),
    SetMessage(Box<GMMessage>),
    SetPosition(GMVec2D),
    SetSize(GMSize),
    SetTarget(GMTarget),
    SetX(f32),
    SetY(f32),
    Tuple2(Box<GMMessage>, Box<GMMessage>),
    Tuple3(Box<GMMessage>, Box<GMMessage>, Box<GMMessage>),
    Tuple4(Box<GMMessage>, Box<GMMessage>, Box<GMMessage>, Box<GMMessage>),
}

impl From<Vec<GMMessage>> for GMMessage {
    fn from(messages: Vec<GMMessage>) -> Self {
        Self::Multiple(messages)
    }
}

impl From<(&str, GMValue)> for GMMessage {
    fn from((name, value): (&str, GMValue)) -> Self {
        Self::Custom2(name.to_string(), value)
    }
}

impl From<(GMMessage, GMMessage)> for GMMessage {
    fn from((m1, m2): (GMMessage, GMMessage)) -> Self {
        Self::Tuple2(Box::new(m1), Box::new(m2))
    }
}

impl From<(GMMessage, GMMessage, GMMessage)> for GMMessage {
    fn from((m1, m2, m3): (GMMessage, GMMessage, GMMessage)) -> Self {
        Self::Tuple3(Box::new(m1), Box::new(m2), Box::new(m3))
    }
}

impl From<(GMMessage, GMMessage, GMMessage, GMMessage)> for GMMessage {
    fn from((m1, m2, m3, m4): (GMMessage, GMMessage, GMMessage, GMMessage)) -> Self {
        Self::Tuple4(Box::new(m1), Box::new(m2), Box::new(m3), Box::new(m4))
    }
}

#[derive(Clone, Debug)]
pub enum GMValue {
    Any(Rc<dyn Any>),
    Bool(bool),
    Custom1(String),
    Custom2(String, Box<GMValue>),
    F32(f32),
    F64(f64),
    I16(i16),
    I32(i32),
    I64(i64),
    I8(i8),
    Message(Box<GMMessage>),
    Multiple(Vec<GMValue>),
    Name(String),
    None,
    Object(Box<dyn GMObjectT>),
    Position(GMVec2D),
    SharedObject(Rc<dyn GMObjectT>),
    Size(GMSize),
    String(String),
    Target(GMTarget),
    Tuple2(Box<GMValue>, Box<GMValue>),
    Tuple3(Box<GMValue>, Box<GMValue>, Box<GMValue>),
    Tuple4(Box<GMValue>, Box<GMValue>, Box<GMValue>, Box<GMValue>),
    U16(u16),
    U32(u32),
    U64(u64),
    U8(u8),
    USize(usize),
    Vec2D(GMVec2D),
}

impl From<()> for GMValue {
    fn from(_value: ()) -> Self {
        Self::None
    }
}

impl From<bool> for GMValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<u8> for GMValue {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl From<u16> for GMValue {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for GMValue {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for GMValue {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<i8> for GMValue {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}

impl From<i16> for GMValue {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}

impl From<i32> for GMValue {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for GMValue {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<usize> for GMValue {
    fn from(value: usize) -> Self {
        Self::USize(value)
    }
}

impl From<f32> for GMValue {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<GMVec2D> for GMValue {
    fn from(value: GMVec2D) -> Self {
        Self::Vec2D(value)
    }
}

impl From<String> for GMValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<(f32, f32)> for GMValue {
    fn from((v1, v2): (f32, f32)) -> Self {
        Self::Tuple2(Box::new(GMValue::F32(v1)), Box::new(GMValue::F32(v2)))
    }
}

impl From<(f32, f32, f32)> for GMValue {
    fn from((v1, v2, v3): (f32, f32, f32)) -> Self {
        Self::Tuple3(Box::new(GMValue::F32(v1)), Box::new(GMValue::F32(v2)), Box::new(GMValue::F32(v3)))
    }
}

impl From<GMMessage> for GMValue {
    fn from(value: GMMessage) -> Self {
        Self::Message(Box::new(value))
    }
}

impl From<GMTarget> for GMValue {
    fn from(value: GMTarget) -> Self {
        Self::Target(value)
    }
}

impl From<(GMValue, GMValue)> for GMValue {
    fn from((v1, v2): (GMValue, GMValue)) -> Self {
        Self::Tuple2(Box::new(v1), Box::new(v2))
    }
}

impl From<(GMValue, GMValue, GMValue)> for GMValue {
    fn from((v1, v2, v3): (GMValue, GMValue, GMValue)) -> Self {
        Self::Tuple3(Box::new(v1), Box::new(v2), Box::new(v3))
    }
}

impl From<(GMValue, GMValue, GMValue, GMValue)> for GMValue {
    fn from((v1, v2, v3, v4): (GMValue, GMValue, GMValue, GMValue)) -> Self {
        Self::Tuple4(Box::new(v1), Box::new(v2), Box::new(v3), Box::new(v4))
    }
}

impl From<Vec<GMValue>> for GMValue {
    fn from(value: Vec<GMValue>) -> Self {
        Self::Multiple(value)
    }
}

impl From<(&str, GMValue)> for GMValue {
    fn from((name, value): (&str, GMValue)) -> Self {
        Self::Custom2(name.to_string(), Box::new(value))
    }
}

#[derive(Clone, Debug)]
pub enum GMTarget {
    Single(String),
    Multiple(Vec<String>),
    Group(String),
    MultipleGroups(Vec<String>),
    ObjectManager,
}

impl From<&str> for GMTarget {
    fn from(value: &str) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<String> for GMTarget {
    fn from(value: String) -> Self {
        Self::Single(value)
    }
}

impl From<&[&str]> for GMTarget {
    fn from(value: &[&str]) -> Self {
        let vec = value.to_vec();
        let vec2: Vec<String> = vec.iter().map(|s| s.to_string()).collect();
        Self::Multiple(vec2)
    }
}

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
    manager_messages: RefCell<VecDeque<GMMessage>>,
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
        } else {
            error_panic(&format!("GMObjectManager::replace_object: object {} not found", name));
        }
    }

    pub fn remove_object(&mut self, name: &str) {
        self.objects.remove(name);
    }

    fn update_objects(&self, context: &mut GMContext) {
        let mut objects: Vec<&GMObjectInfo> = self.objects.values().filter(|o| o.active).collect();
        objects.sort_by(|a, b| a.update_index.cmp(&b.update_index));

        for o in objects {
            o.inner.borrow_mut().update(context, &self);
        }
    }

    pub fn update(&mut self, context: &mut GMContext) {
        self.update_objects(context);
        self.process_manager_messages();
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
        } else {
            error_panic(&format!("GMObjectManager::set_draw_index: object {} not found", name));
        }
    }

    pub fn get_draw_index(&self, name: &str) -> i32 {
        if let Some(object) = self.objects.get(name) {
            return object.draw_index;
        } else {
            error_panic(&format!("GMObjectManager::get_draw_index: object {} not found", name));
        }
    }

    pub fn set_update_index(&mut self, name: &str, update_index: i32) {
        if let Some(object) = self.objects.get_mut(name) {
            object.update_index = update_index;
        } else {
            error_panic(&format!("GMObjectManager::set_update_index: object {} not found", name));
        }
    }

    pub fn get_update_index(&self, name: &str) -> i32 {
        if let Some(object) = self.objects.get(name) {
            return object.update_index;
        } else {
            error_panic(&format!("GMObjectManager::get_update_index: object {} not found", name));
        }
    }

    pub fn set_active(&mut self, name: &str, active: bool) {
        if let Some(object) = self.objects.get_mut(name) {
            object.active = active;
        } else {
            error_panic(&format!("GMObjectManager::set_active: object {} not found", name));
        }
    }

    pub fn get_active(&self, name: &str) -> bool {
        if let Some(object) = self.objects.get(name) {
            return object.active;
        } else {
            error_panic(&format!("GMObjectManager::get_active: object {} not found", name));
        }
    }

    pub fn toggle_active(&mut self, name: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.active = !object.active;
        } else {
            error_panic(&format!("GMObjectManager::toggle_active: object {} not found", name));
        }
    }

    pub fn set_visible(&mut self, name: &str, visible: bool) {
        if let Some(object) = self.objects.get_mut(name) {
            object.visible = visible;
        } else {
            error_panic(&format!("GMObjectManager::set_visible: object {} not found", name));
        }
    }

    pub fn get_visible(&self, name: &str) -> bool {
        if let Some(object) = self.objects.get(name) {
            return object.visible;
        } else {
            error_panic(&format!("GMObjectManager::get_visible: object {} not found", name));
        }
    }

    pub fn toggle_visible(&mut self, name: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.visible = !object.visible;
        } else {
            error_panic(&format!("GMObjectManager::toggle_visible: object {} not found", name));
        }
    }

    pub fn add_group(&mut self, name: &str, group: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.groups.insert(group.to_string());
        } else {
            error_panic(&format!("GMObjectManager::add_group: object {} not found", name));
        }
    }

    pub fn is_in_group(&self, name: &str, group: &str) -> bool {
        if let Some(object) = self.objects.get(name) {
            return object.groups.contains(group);
        } else {
            error_panic(&format!("GMObjectManager::is_in_group: object {} not found", name));
        }
    }

    pub fn remove_group(&mut self, name: &str, group: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.groups.remove(group);
        } else {
            error_panic(&format!("GMObjectManager::remove_group: object {} not found", name));
        }
    }

    pub fn clear_groups(&mut self, name: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.groups.clear();
        } else {
            error_panic(&format!("GMObjectManager::clear_groups: object {} not found", name));
        }
    }

    pub fn set_custom_property(&mut self, name: &str, key: &str, value: GMValue) {
        if let Some(object) = self.objects.get_mut(name) {
            object.custom_properties.insert(key.to_string(), value);
        } else {
            error_panic(&format!("GMObjectManager::set_custom_property: object {} not found", name));
        }
    }

    pub fn get_custom_property(&self, name: &str, key: &str) -> Option<&GMValue> {
        if let Some(object) = self.objects.get(name) {
            return object.custom_properties.get(key);
        } else {
            error_panic(&format!("GMObjectManager::get_custom_property: object {} not found", name));
        }
    }

    pub fn remove_custom_property(&mut self, name: &str, key: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.custom_properties.remove(key);
        } else {
            error_panic(&format!("GMObjectManager::remove_custom_property: object {} not found", name));
        }
    }

    pub fn clear_custom_properties(&mut self, name: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.custom_properties.clear();
        } else {
            error_panic(&format!("GMObjectManager::clear_custom_properties: object {} not found", name));
        }
    }

    pub fn send_message<T: Into<GMTarget>>(&self, target: T, message: GMMessage, context: &mut GMContext) -> GMValue {
        let target = target.into();

        match target {
            GMTarget::Single(name) => {
                if let Some(object) = self.objects.get(&name) {
                    let mut borrowed_object = object.inner.borrow_mut();
                    return borrowed_object.send_message(message, context, &self);
                } else {
                    error_panic(&format!("GMObjectManager::send_message: object {} not found", name));
                }
            }
            GMTarget::Multiple(names) => {
                let mut result = Vec::new();

                for name in names {
                    if let Some(object) = self.objects.get(&name) {
                        let mut borrowed_object = object.inner.borrow_mut();
                        let value = borrowed_object.send_message(message.clone(), context, &self);
                        result.push(GMValue::Tuple2(Box::new(GMValue::Name(name.clone())), Box::new(value)));
                    } else {
                        error_panic(&format!("GMObjectManager::send_message: object {} not found", name));
                    }
                }

                return GMValue::Multiple(result);
            }
            GMTarget::Group(group) => {
                let mut result = Vec::new();

                for (name, object) in self.objects.iter() {
                    if object.groups.contains(&group) {
                        let mut borrowed_object = object.inner.borrow_mut();
                        let value = borrowed_object.send_message(message.clone(), context, &self);
                        result.push(GMValue::Tuple2(Box::new(GMValue::Name(name.clone())), Box::new(value)));
                    }
                }

                return GMValue::Multiple(result);
            }
            GMTarget::MultipleGroups(groups) => {
                let mut result = Vec::new();

                for (name, object) in self.objects.iter() {
                    for group in groups.iter() {
                        if object.groups.contains(group) {
                            let mut borrowed_object = object.inner.borrow_mut();
                            let value = borrowed_object.send_message(message.clone(), context, &self);
                            result.push(GMValue::Tuple2(Box::new(GMValue::Name(name.clone())), Box::new(value)));
                            // This break ensures that the message is not sent multiple times
                            // to the same object if it is in multiple matching groups.
                            break;
                        }
                    }
                }

                return GMValue::Multiple(result);
            }
            GMTarget::ObjectManager => {
                let mut messages = self.manager_messages.borrow_mut();
                messages.push_back(message);
            }
        }

        GMValue::None
    }

    pub fn process_manager_messages(&mut self) {
        let mut messages = self.manager_messages.take();

        while let Some(message) = messages.pop_front() {
            match message {
                GMMessage::OMAddCustomObject(object_name, object_info) => {
                    self.add_custom_object(&object_name, object_info);
                }
                GMMessage::OMAddDrawObject(object_name, object, update_index , draw_index) => {
                    self.add_draw_object(&object_name, object, update_index, draw_index);
                }
                GMMessage::OMAddGroup(object_name, group) => {
                    self.add_group(&object_name, &group);
                }
                GMMessage::OMAddNormalObject(object_name, object, update_index) => {
                    self.add_normal_object(&object_name, object, update_index);
                }
                GMMessage::OMClearCustomProperties(object_name) => {
                    self.clear_custom_properties(&object_name);
                }
                GMMessage::OMClearGroups(object_name) => {
                    self.clear_groups(&object_name);
                }
                GMMessage::OMRemoveCustomProperty(object_name, key) => {
                    self.remove_custom_property(&object_name, &key);
                }
                GMMessage::OMRemoveGroup(object_name, group) => {
                    self.remove_group(&object_name, &group);
                }
                GMMessage::OMRemoveObject(object_name) => {
                    self.remove_object(&object_name);
                }
                GMMessage::OMReplaceObject(object_name, object) => {
                    self.replace_object(&object_name, object);
                }
                GMMessage::OMSetActive(object_name, active) => {
                    self.set_active(&object_name, active);
                }
                GMMessage::OMSetCustomProperty(object_name, key, value) => {
                    self.set_custom_property(&object_name, &key, value);
                }
                GMMessage::OMSetDrawIndex(object_name, draw_index) => {
                    self.set_draw_index(&object_name, draw_index);
                }
                GMMessage::OMSetUpdateIndex(object_name, update_index) => {
                    self.set_update_index(&object_name, update_index);
                }
                GMMessage::OMSetVisible(object_name, visible) => {
                    self.set_visible(&object_name, visible);
                }
                GMMessage::OMToggleActive(object_name) => {
                    self.toggle_active(&object_name);
                }
                GMMessage::OMToggleVisible(object_name) => {
                    self.toggle_visible(&object_name);
                }
                _ => {
                    error_panic(&format!("Wrong message for GMObjectManager::process_manager_messages: {:?}", message))
                }
            }
        }
    }
}


// TODO: Add pre-processing for messages: position, active, visible, ...

pub trait GMObjectT: Debug {
    fn send_message(&mut self, _message: GMMessage, _context: &mut GMContext, _object_manager: &GMObjectManager) -> GMValue {
        GMValue::None
    }

    fn send_tuple2_message(&mut self, message1: GMMessage, message2: GMMessage, context: &mut GMContext,
        object_manager: &GMObjectManager) -> GMValue {
        let result1 = self.send_message(message1, context, object_manager);
        let result2 = self.send_message(message2, context, object_manager);
        (result1, result2).into()
    }

    fn send_tuple3_message(&mut self, message1: GMMessage, message2: GMMessage, message3: GMMessage,
        context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        let result1 = self.send_message(message1, context, object_manager);
        let result2 = self.send_message(message2, context, object_manager);
        let result3 = self.send_message(message3, context, object_manager);
        (result1, result2, result3).into()
    }

    fn send_tuple4_message(&mut self, message1: GMMessage, message2: GMMessage, message3: GMMessage, message4: GMMessage,
        context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        let result1 = self.send_message(message1, context, object_manager);
        let result2 = self.send_message(message2, context, object_manager);
        let result3 = self.send_message(message3, context, object_manager);
        let result4 = self.send_message(message4, context, object_manager);
        (result1, result2, result3, result4).into()
    }

    fn send_multi_message(&mut self, messages: Vec<GMMessage>, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        let mut result = Vec::new();

        for message in messages.iter() {
            result.push(self.send_message(message.clone(), context, object_manager));
        }

        return result.into()
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
