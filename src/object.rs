
use std::cell::RefCell;
use std::fmt::Debug;
use std::collections::{HashSet, HashMap, VecDeque};
use std::rc::Rc;

use crate::context::GMContext;
use crate::math::{GMVec2D, GMSize};
use crate::util::{GMAlign, error_panic};
use crate::bitmap_text::GMBitmapFont;

#[derive(Clone, Debug)]
pub enum GMMessage {
    AddPosition(GMVec2D),
    AddSpacing(GMVec2D),
    AddSpacingX(f32),
    AddSpacingY(f32),
    AddX(f32),
    AddY(f32),
    Custom(String),
    GetAlign,
    GetAll(Box<GMMessage>),
    GetCustom(String),
    GetElementIndices,
    GetFont,
    GetHorizontal,
    GetMessage,
    GetNumElements,
    GetPosition,
    GetRepeat,
    GetSize,
    GetSpacing,
    GetSpacingX,
    GetSpacingY,
    GetTarget,
    GetText,
    GetTimeout,
    GetX,
    GetXY,
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
    ResetChars,
    ResetPosition,
    SetAlign(GMAlign),
    SetCustom(String, GMValue),
    SetElementIndices(Vec<usize>),
    SetFont(Rc<GMBitmapFont>),
    SetFontName(String),
    SetHorizontal(bool),
    SetMessage(Box<GMMessage>),
    SetNumElements(usize),
    SetPosition(GMVec2D),
    SetRepeat(bool),
    SetSize(GMSize),
    SetSpacing(GMVec2D),
    SetSpacingX(f32),
    SetSpacingY(f32),
    SetTarget(GMTarget),
    SetText(String),
    SetTimeout(f32),
    SetValueOf(usize, GMValue),
    SetX(f32),
    SetXY(f32, f32),
    SetY(f32),
    ToAllElements(Box<GMMessage>),
    ToElementN(usize, Box<GMMessage>),
    ToggleHorizontal,
    Trigger,
    Tuple2(Box<GMMessage>, Box<GMMessage>),
    Tuple3(Box<GMMessage>, Box<GMMessage>, Box<GMMessage>),
    Tuple4(Box<GMMessage>, Box<GMMessage>, Box<GMMessage>, Box<GMMessage>),
}

#[derive(Clone, Debug)]
pub enum GMValue {
    Align(GMAlign),
    Bool(bool),
    ElementIndices(Vec<usize>),
    F32(f32),
    F64(f64),
    Font(Rc<GMBitmapFont>),
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
    Repeat(bool),
    Size(GMSize),
    String(String),
    Target(GMTarget),
    Text(String),
    Timeout(f32),
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

#[derive(Clone, Debug)]
pub enum GMTarget {
    Single(String),
    Multiple(Vec<String>),
    Group(String),
    MultipleGroups(Vec<String>),
    ObjectManager,
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

    /*
    fn send_message_inner(&self, object: &GMObjectInfo, message: GMMessage, context: &mut GMContext) -> GMValue {
        let mut borrowed_object = object.inner.borrow_mut();

        match message {
            GMMessage::Multiple(messages) => {
                let mut result = Vec::new();

                for inner_message in messages.iter() {
                    result.push(borrowed_object.send_message(*inner_message, context, &self));
                }

                GMValue::Multiple(result)
            }
            _ => {
                borrowed_object.send_message(message, context, &self)
            }
        }
    }
    */

    pub fn send_message<T: Into<GMTarget>>(&self, target: T, message: GMMessage, context: &mut GMContext) -> GMValue {
        let target = target.into();

        match target {
            GMTarget::Single(name) => {
                if let Some(object) = self.objects.get(&name) {
                    let mut borrowed_object = object.inner.borrow_mut();
                    return borrowed_object.send_message(message, context, &self);
                }                        
            }
            GMTarget::Multiple(names) => {
                let mut result = Vec::new();

                for name in names {
                    if let Some(object) = self.objects.get(&name) {
                        let mut borrowed_object = object.inner.borrow_mut();
                        let value = borrowed_object.send_message(message.clone(), context, &self);
                        result.push(GMValue::Tuple2(Box::new(GMValue::Name(name.clone())), Box::new(value)));
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
