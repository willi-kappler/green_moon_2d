
use std::collections::{HashMap, HashSet, VecDeque};
use std::cell::RefCell;

use crate::value::GMValue;
use crate::object::GMObjectT;
use crate::message::GMMessage;
use crate::util::error_panic;
use crate::context::GMContext;
use crate::target::GMTarget;

#[derive(Clone, Debug)]
pub struct GMObjectInfo {
    pub active: bool,
    pub custom_properties: HashMap<String, GMValue>,
    pub draw_index: i32,
    pub groups: HashSet<String>,
    pub inner: RefCell<Box<dyn GMObjectT>>,
    pub update_index: i32,
    pub visible: bool,
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

#[derive(Debug)]
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

    pub fn add_normal_object_group<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, object: T, update_index: i32, group: &str) {
        let mut groups = HashSet::new();
        groups.insert(group.to_string());

        let new_object = GMObjectInfo {
            active: true,
            custom_properties: HashMap::new(),
            draw_index: 0,
            groups,
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

    pub fn add_draw_object_group<T: Into<Box<dyn GMObjectT>>>(&mut self, name: &str, object: T, update_index: i32, draw_index: i32, group: &str) {
        let mut groups = HashSet::new();
        groups.insert(group.to_string());

        let new_object = GMObjectInfo {
            active: true,
            custom_properties: HashMap::new(),
            draw_index: draw_index,
            groups,
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

    pub fn remove_objects_in_group(&mut self, group: &str) {
        self.objects.retain(|_, v| !v.groups.contains(group));
    }

    pub fn remove_objects_not_in_group(&mut self, group: &str) {
        self.objects.retain(|_, v| v.groups.contains(group));
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

    pub fn send_message(&self, target: &GMTarget, message: GMMessage, context: &mut GMContext) -> GMValue {
        match target {
            GMTarget::Single(name) => {
                if let Some(object) = self.objects.get(name) {
                    if object.active {
                        let mut borrowed_object = object.inner.borrow_mut();
                        return borrowed_object.send_message(message, context, &self);
                    }
                } else {
                    error_panic(&format!("GMObjectManager::send_message: object {} not found", name));
                }
            }
            GMTarget::Multiple(names) => {
                let mut result = Vec::new();

                for name in names {
                    if let Some(object) = self.objects.get(name) {
                        if object.active {
                            let mut borrowed_object = object.inner.borrow_mut();
                            let value = borrowed_object.send_message(message.clone(), context, &self);
                            result.push(value);
                        }
                    } else {
                        error_panic(&format!("GMObjectManager::send_message: object {} not found", name));
                    }
                }

                return GMValue::Multiple(result);
            }
            GMTarget::Group(group) => {
                let mut result = Vec::new();

                for (_, object) in self.objects.iter() {
                    if object.active && object.groups.contains(group) {
                        let mut borrowed_object = object.inner.borrow_mut();
                        let value = borrowed_object.send_message(message.clone(), context, &self);
                        result.push(value);
                    }
                }

                return GMValue::Multiple(result);
            }
            GMTarget::MultipleGroups(groups) => {
                let mut result = Vec::new();

                for (_, object) in self.objects.iter() {
                    for group in groups.iter() {
                        if object.active && object.groups.contains(group) {
                            let mut borrowed_object = object.inner.borrow_mut();
                            let value = borrowed_object.send_message(message.clone(), context, &self);
                            result.push(value);
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

    pub fn send_custom_message0(&self, target: &GMTarget, message: &str, context: &mut GMContext) -> GMValue {
        self.send_message(target, GMMessage::Custom0(message.to_string()), context)
    }

    pub fn send_custom_message1(&self, target: &GMTarget, message: &str, value: GMValue, context: &mut GMContext) -> GMValue {
        self.send_message(target, GMMessage::Custom1(message.to_string(), value), context)
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
