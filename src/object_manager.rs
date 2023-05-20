
use std::collections::{HashMap, HashSet, VecDeque};
use std::cell::RefCell;

use log::{info, debug};

use crate::value::GMValue;
use crate::object::GMObjectT;
use crate::message::GMMessage;
use crate::util::error_panic;
use crate::context::GMContext;
use crate::target::GMTarget;
use crate::state::GMState;

#[derive(Clone, Debug)]
pub struct GMObjectInfo {
    pub active: bool,
    pub draw_index: i32,
    pub groups: HashSet<String>,
    pub inner: RefCell<Box<dyn GMObjectT>>,
    pub state: RefCell<GMState>,
    pub update_index: i32,
    pub visible: bool,
}

impl GMObjectInfo {
    pub fn new<T: Into<Box<dyn GMObjectT>>>(object: T) -> Self {
        Self {
            active: true,
            draw_index: 0,
            groups: HashSet::new(),
            inner: RefCell::new(object.into()),
            state: RefCell::new(GMState::new()),
            update_index: 0,
            visible: true,
        }
    }
}

#[derive(Debug)]
pub struct GMObjectManager {
    objects: HashMap<String, GMObjectInfo>,
    manager_messages: RefCell<VecDeque<GMMessage>>,
    init_objects: RefCell<Vec<String>>,
}

impl GMObjectManager {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            manager_messages: RefCell::new(VecDeque::new()),
            init_objects: RefCell::new(Vec::new()),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();

        let mut messages = self.manager_messages.borrow_mut();
        messages.clear();
    }

    fn insert_object(&mut self, name: &str, object: GMObjectInfo) {
        if let Some(_) = self.objects.insert(name.to_string(), object) {
            info!("GMObjectManager: object '{}' has been replaced", name);
        }
    }

    pub fn add_normal_object<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, object: T, update_index: i32) {
        let mut new_object = GMObjectInfo::new(object);
        new_object.visible = false;
        new_object.update_index = update_index;

        self.insert_object(name, new_object);
    }

    pub fn add_normal_object_group<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, object: T, update_index: i32, group: &str) {
        let mut groups = HashSet::new();
        groups.insert(group.to_string());

        let mut new_object = GMObjectInfo::new(object);
        new_object.visible = false;
        new_object.update_index = update_index;
        new_object.groups = groups;

        self.insert_object(name, new_object);
    }

    pub fn add_draw_object<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, object: T, update_index: i32, draw_index: i32) {
        let mut new_object = GMObjectInfo::new(object);
        new_object.draw_index = draw_index;
        new_object.update_index = update_index;

        self.insert_object(name, new_object);
    }

    pub fn add_draw_object_group<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, object: T, update_index: i32, draw_index: i32, group: &str) {
        let mut groups = HashSet::new();
        groups.insert(group.to_string());

        let mut new_object = GMObjectInfo::new(object);
        new_object.draw_index = draw_index;
        new_object.update_index = update_index;
        new_object.groups = groups;

        self.insert_object(name, new_object);
    }

    pub fn add_custom_object(&mut self, name: &str, new_object: GMObjectInfo) {
        self.insert_object(name, new_object);
    }

    pub fn replace_object<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, new_object: T) {
        if let Some(object) = self.objects.get(name) {
            object.inner.replace(new_object.into());
        } else {
            error_panic(&format!("GMObjectManager::replace_object: object '{}' not found", name));
        }
    }

    pub fn remove_object(&mut self, name: &str) {
        self.objects.remove(name);
    }

    pub fn remove_objects_in_group(&mut self, group: &str) {
        self.objects.retain(|_, v| !v.groups.contains(group));
    }

    pub fn remove_objects_not_in_group(&mut self, group: &str) {
        self.objects.retain(|_, v| v.groups.contains(group));
    }

    pub fn get_object(&self, name: &str) -> &GMObjectInfo {
        if let Some(object) = self.objects.get(name) {
            return object;
        } else {
            error_panic(&format!("GMObjectManager::get_object: object '{}' not found", name));
        }
    }

    pub fn get_object_mut(&mut self, name: &str) -> &mut GMObjectInfo {
        if let Some(object) = self.objects.get_mut(name) {
            return object;
        } else {
            error_panic(&format!("GMObjectManager::get_object: object '{}' not found", name));
        }
    }

    pub fn initialize_object(&self, name: &str) {
        debug!("Initialize object: '{}'", name);

        let mut init = self.init_objects.borrow_mut();
        init.push(name.to_string());
    }

    fn process_initialization(&self, context: &mut GMContext) {
        let mut init = self.init_objects.borrow_mut();

        if init.is_empty() {
            return;
        }

        for name in init.iter() {
            if let Some(object) = self.objects.get(name) {
                object.inner.borrow_mut().send_message(GMMessage::Init, context, self);
            } else {
                error_panic(&format!("GMObjectManager::get_object: object '{}' not found", name));
            }
        }

        init.clear();
    }

    fn update_objects(&self, context: &mut GMContext) {
        let mut objects: Vec<&GMObjectInfo> = self.objects.values().filter(|o| o.active).collect();
        objects.sort_by(|a, b| a.update_index.cmp(&b.update_index));

        for o in objects {
            o.inner.borrow_mut().update(context, &self);
        }
    }

    pub fn update(&mut self, context: &mut GMContext) {
        self.process_initialization(context);
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
            error_panic(&format!("GMObjectManager::set_draw_index: object '{}' not found", name));
        }
    }

    pub fn get_draw_index(&self, name: &str) -> i32 {
        if let Some(object) = self.objects.get(name) {
            return object.draw_index;
        } else {
            error_panic(&format!("GMObjectManager::get_draw_index: object '{}' not found", name));
        }
    }

    pub fn set_update_index(&mut self, name: &str, update_index: i32) {
        if let Some(object) = self.objects.get_mut(name) {
            object.update_index = update_index;
        } else {
            error_panic(&format!("GMObjectManager::set_update_index: object '{}' not found", name));
        }
    }

    pub fn get_update_index(&self, name: &str) -> i32 {
        if let Some(object) = self.objects.get(name) {
            return object.update_index;
        } else {
            error_panic(&format!("GMObjectManager::get_update_index: object '{}' not found", name));
        }
    }

    pub fn set_active(&mut self, name: &str, active: bool) {
        if let Some(object) = self.objects.get_mut(name) {
            object.active = active;
        } else {
            error_panic(&format!("GMObjectManager::set_active: object '{}' not found", name));
        }
    }

    pub fn set_active_in_group(&mut self, group: &str, active: bool) {
        for object in self.objects.values_mut() {
            if object.groups.contains(group) {
                object.active = active;
            }
        }
    }

    pub fn set_active_not_in_group(&mut self, group: &str, active: bool) {
        for object in self.objects.values_mut() {
            if !object.groups.contains(group) {
                object.active = active;
            }
        }
    }

    pub fn get_active(&self, name: &str) -> bool {
        if let Some(object) = self.objects.get(name) {
            return object.active;
        } else {
            error_panic(&format!("GMObjectManager::get_active: object '{}' not found", name));
        }
    }

    pub fn toggle_active(&mut self, name: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.active = !object.active;
        } else {
            error_panic(&format!("GMObjectManager::toggle_active: object '{}' not found", name));
        }
    }

    pub fn set_visible(&mut self, name: &str, visible: bool) {
        if let Some(object) = self.objects.get_mut(name) {
            object.visible = visible;
        } else {
            error_panic(&format!("GMObjectManager::set_visible: object '{}' not found", name));
        }
    }

    pub fn set_visible_in_group(&mut self, group: &str, visible: bool) {
        for object in self.objects.values_mut() {
            if object.groups.contains(group) {
                object.visible = visible;
            }
        }
    }

    pub fn set_visible_not_in_group(&mut self, group: &str, visible: bool) {
        for object in self.objects.values_mut() {
            if !object.groups.contains(group) {
                object.active = visible;
            }
        }
    }

    pub fn get_visible(&self, name: &str) -> bool {
        if let Some(object) = self.objects.get(name) {
            return object.visible;
        } else {
            error_panic(&format!("GMObjectManager::get_visible: object '{}' not found", name));
        }
    }

    pub fn toggle_visible(&mut self, name: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.visible = !object.visible;
        } else {
            error_panic(&format!("GMObjectManager::toggle_visible: object '{}' not found", name));
        }
    }

    pub fn add_group(&mut self, name: &str, group: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.groups.insert(group.to_string());
        } else {
            error_panic(&format!("GMObjectManager::add_group: object '{}' not found", name));
        }
    }

    pub fn is_in_group(&self, name: &str, group: &str) -> bool {
        if let Some(object) = self.objects.get(name) {
            return object.groups.contains(group);
        } else {
            error_panic(&format!("GMObjectManager::is_in_group: object '{}' not found", name));
        }
    }

    pub fn remove_group(&mut self, name: &str, group: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.groups.remove(group);
        } else {
            error_panic(&format!("GMObjectManager::remove_group: object '{}' not found", name));
        }
    }

    pub fn clear_groups(&mut self, name: &str) {
        if let Some(object) = self.objects.get_mut(name) {
            object.groups.clear();
        } else {
            error_panic(&format!("GMObjectManager::clear_groups: object '{}' not found", name));
        }
    }

    pub fn get_state_property(&self, name: &str, property: &str) -> GMValue {
        if let Some(object) = self.objects.get(name) {
            return object.state.borrow().get_property(property).clone();
        } else {
            error_panic(&format!("GMObjectManager::get_state_property: object '{}' not found", name));
        }
    }

    pub fn set_state_property<T: Into<GMValue>>(&self, name: &str, property: &str, value: T) {
        if let Some(object) = self.objects.get(name) {
            object.state.borrow_mut().set_property(property, value);
        } else {
            error_panic(&format!("GMObjectManager::get_state_property: object '{}' not found", name));
        }
    }

    pub fn set_state_property_in_group<T: Into<GMValue>>(&self, group: &str, property: &str, value: T) {
        let value = value.into();

        for object in self.objects.values() {
            if object.groups.contains(group) {
                object.state.borrow_mut().set_property(property, value.clone());
            }
        }
    }

    pub fn set_state_property_not_in_group<T: Into<GMValue>>(&self, group: &str, property: &str, value: T) {
        let value = value.into();

        for object in self.objects.values() {
            if !object.groups.contains(group) {
                object.state.borrow_mut().set_property(property, value.clone());
            }
        }
    }

    pub fn remove_state_property(&self, name: &str, property: &str) {
        if let Some(object) = self.objects.get(name) {
            object.state.borrow_mut().remove_property(property);
        } else {
            error_panic(&format!("GMObjectManager::get_state_property: object '{}' not found", name));
        }
    }

    pub fn remove_state_property_in_group(&self, group: &str, property: &str) {
        for object in self.objects.values() {
            if object.groups.contains(group) {
                object.state.borrow_mut().remove_property(property);
            }
        }
    }

    pub fn remove_state_property_not_in_group(&self, group: &str, property: &str) {
        for object in self.objects.values() {
            if !object.groups.contains(group) {
                object.state.borrow_mut().remove_property(property);
            }
        }
    }

    pub fn clear_state_property(&self, name: &str) {
        if let Some(object) = self.objects.get(name) {
            object.state.borrow_mut().clear();
        } else {
            error_panic(&format!("GMObjectManager::get_state_property: object '{}' not found", name));
        }
    }

    pub fn get_state(&self, name: &str) -> &RefCell<GMState> {
        if let Some(object) = self.objects.get(name) {
            return &object.state;
        } else {
            error_panic(&format!("GMObjectManager::get_state: object '{}' not found", name));
        }
    }

    pub fn set_state(&self, name: &str, state: GMState) -> GMState {
        if let Some(object) = self.objects.get(name) {
            return object.state.replace(state)
        } else {
            error_panic(&format!("GMObjectManager::set_state: object '{}' not found", name));
        }
    }

    pub fn send_message_object(&self, name: &str, message: GMMessage, context: &mut GMContext) -> GMValue {
        if let Some(object) = self.objects.get(name) {
            if object.active {
                let mut borrowed_object = object.inner.borrow_mut();
                return borrowed_object.send_message(message, context, &self);
            }
        } else {
            error_panic(&format!("GMObjectManager::send_message_object: object '{}' not found", name));
        }

        GMValue::None
    }

    pub fn send_message_object_zip(&self, names: &Vec<&str>, messages: Vec<GMMessage>, context: &mut GMContext) -> GMValue {
        let mut result = Vec::new();

        for (name, message) in names.iter().zip(messages) {
            if let Some(object) = self.objects.get(*name) {
                if object.active {
                    let mut borrowed_object = object.inner.borrow_mut();
                    let value = borrowed_object.send_message(message.clone(), context, &self);
                    result.push(value);
                }
            }
        }

        return result.into();
    }

    pub fn send_message_group(&self, group: &str, message: GMMessage, context: &mut GMContext) -> GMValue {
        let mut result = Vec::new();

        let objects = self.objects.iter()
            .map(|(_, o)| o)
            .filter(|o| o.active && o.groups.contains(group));

        for object in objects {
            let mut borrowed_object = object.inner.borrow_mut();
            let value = borrowed_object.send_message(message.clone(), context, &self);
            result.push(value);
        }

        return result.into();
    }

    pub fn send_message_group_zip(&self, group: &str, messages: Vec<GMMessage>, context: &mut GMContext) -> GMValue {
        let mut result = Vec::new();

        let objects = self.objects.iter()
            .map(|(_, o)| o)
            .filter(|o| o.active && o.groups.contains(group));

        for (object, message) in objects.zip(messages) {
            let mut borrowed_object = object.inner.borrow_mut();
            let value = borrowed_object.send_message(message, context, &self);
            result.push(value);
        }

        return result.into();
    }

    pub fn send_message(&self, target: &GMTarget, message: GMMessage, context: &mut GMContext) -> GMValue {
        match target {
            GMTarget::Object(name) => {
                return self.send_message_object(name, message, context)
            }
            GMTarget::Group(group) => {
                return self.send_message_group(group, message, context)
            }
            GMTarget::ObjectManager => {
                let mut messages = self.manager_messages.borrow_mut();
                messages.push_back(message);
            }
            GMTarget::Multiple(targets) => {
                let mut result = Vec::new();

                for target in targets {
                    result.push(self.send_message(target, message.clone(), context));
                }

                return result.into();
            }
        }

        GMValue::None
    }

    pub fn send_message_zip(&self, target: &GMTarget, messages: Vec<GMMessage>, context: &mut GMContext) -> GMValue {
        match target {
            GMTarget::Object(name) => {
                return self.send_message_object(name, messages[0].clone(), context)
            }
            GMTarget::Group(group) => {
                return self.send_message_group_zip(group, messages, context)
            }
            GMTarget::ObjectManager => {
                let mut manager_messages = self.manager_messages.borrow_mut();
                manager_messages.push_back(messages[0].clone());
            }
            GMTarget::Multiple(targets) => {
                let mut result = Vec::new();

                for (target, message) in targets.iter().zip(messages) {
                    result.push(self.send_message(target, message, context));
                }

                return result.into();
            }
        }

        GMValue::None
    }

    pub fn send_custom_message0(&self, target: &GMTarget, message: &str, context: &mut GMContext) -> GMValue {
        self.send_message(target, GMMessage::Custom0(message.to_string()), context)
    }

    pub fn send_custom_message1<U: Into<GMValue>>(&self, target: &GMTarget, message: &str, value: U, context: &mut GMContext) -> GMValue {
        self.send_message(target, GMMessage::Custom1(message.to_string(), value.into()), context)
    }

    pub fn send_custom_message2<U: Into<GMValue>, V: Into<GMValue>>(&self, target: &GMTarget, message: &str, value1: U, value2: V, context: &mut GMContext) -> GMValue {
        self.send_message(target, GMMessage::Custom2(message.to_string(), value1.into(), value2.into()), context)
    }

    fn process_manager_messages(&mut self) {
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
                GMMessage::OMClearGroups(object_name) => {
                    self.clear_groups(&object_name);
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
                    error_panic(&format!("Wrong message for GMObjectManager::process_manager_messages: '{:?}'", message))
                }
            }
        }
    }

    pub fn log_all_objects(&self) {
        info!("All objects in current object manager:");

        for (k, v) in self.objects.iter() {
            info!("name: '{}', object: '{:?}'", k, v);
        }
    }
}
