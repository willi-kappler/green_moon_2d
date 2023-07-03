

use std::collections::HashMap;

use crate::object::{GMObjectT, GMObjectBox};
use crate::value::GMValue;
use crate::message::GMMessage;
use crate::object_manager::GMObjectManager;
use crate::context::GMContext;
use crate::util::error_panic;

#[derive(Clone, Debug)]
pub struct GMGroupInfo {
    pub active: bool,
    pub draw_index: i32,
    pub object: GMObjectBox,
    pub update_index: i32,
    pub visible: bool,
}

#[derive(Clone, Debug)]
pub struct GMObjectGroup {
    pub objects: HashMap<String, GMGroupInfo>,
}

impl GMObjectGroup {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn add_object(&mut self, name: &str, object: GMObjectBox) {
        let group_info = GMGroupInfo {
            active: true,
            draw_index: 0,
            object,
            update_index: 0,
            visible: true,
        };

        self.objects.insert(name.to_string(), group_info);
    }

    pub fn add_object2(&mut self, name: &str, object: &dyn GMObjectT) {
        let group_info = GMGroupInfo {
            active: true,
            draw_index: 0,
            object: object.into(),
            update_index: 0,
            visible: true,
        };

        self.objects.insert(name.to_string(), group_info);
    }

    pub fn remove_object(&mut self, name: &str) {
        self.objects.remove(name);
    }

    pub fn set_active(&mut self, name: &str, active: bool) {
        self.objects.get_mut(name).unwrap().active = active;
    }

    pub fn toggle_active(&mut self, name: &str) {
        let entry = self.objects.entry(name.to_string());
        entry.and_modify(|group_info| group_info.active = !group_info.active);
    }

    pub fn set_visible(&mut self, name: &str, visible: bool) {
        self.objects.get_mut(name).unwrap().visible = visible;
    }

    pub fn toggle_visible(&mut self, name: &str) {
        let entry = self.objects.entry(name.to_string());
        entry.and_modify(|group_info| group_info.visible = !group_info.visible);
    }

    pub fn set_draw_index(&mut self, name: &str, draw_index: i32) {
        self.objects.get_mut(name).unwrap().draw_index = draw_index;
    }

    pub fn set_update_index(&mut self, name: &str, update_index: i32) {
        self.objects.get_mut(name).unwrap().update_index = update_index;
    }

    pub fn object_message(&mut self, name: &str, message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        self.objects.get_mut(name).unwrap().object.send_message(message, object_manager)
    }
}

impl GMObjectT for GMObjectGroup {
    fn send_message(&mut self, mut message: GMMessage, object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "" => {
                let value = message.value;

                match method {
                    "add_object" => {
                        let (name, object) = value.into_generic::<(String, GMObjectBox)>();
                        self.add_object(&name, object);
                    }
                    "remove_object" => {
                        let name = value.into_string();
                        self.remove_object(&name);
                    }
                    "set_active" => {
                        let (name, active) = value.into_generic::<(String, bool)>();
                        self.set_active(&name, active);
                    }
                    "toggle_active" => {
                        let name = value.into_string();
                        self.toggle_active(&name);
                    }
                    "set_visible" => {
                        let (name, visible) = value.into_generic::<(String, bool)>();
                        self.set_visible(&name, visible);
                    }
                    "toggle_visible" => {
                        let name = value.into_string();
                        self.toggle_visible(&name);
                    }
                    "set_draw_index" => {
                        let (name, draw_index) = value.into_generic::<(String, i32)>();
                        self.set_draw_index(&name, draw_index);
                    }
                    "set_update_index" => {
                        let (name, update_index) = value.into_generic::<(String, i32)>();
                        self.set_update_index(&name, update_index);
                    }
                    _ => {
                        error_panic(&format!("GMObjectGroup::send_message: Unknown method '{}', no tag", method));
                    }
                }
            }
            object_name => {
                return self.object_message(object_name, message, object_manager);
            }
        }


        GMValue::None
    }

    fn update(&mut self, object_manager: &GMObjectManager, context: &mut GMContext) {
        let mut objects: Vec<&mut GMGroupInfo> = self.objects.values_mut()
            .filter(|gi| gi.active).collect();
        objects.sort_by(|a, b| a.update_index.cmp(&b.update_index));

        for object in objects.iter_mut() {
            object.object.update(object_manager, context);
        }
    }

    fn draw(&self, context: &mut GMContext) {
        let mut objects: Vec<&GMGroupInfo> = self.objects.values()
            .filter(|gi| gi.visible).collect();
        objects.sort_by(|a, b| a.draw_index.cmp(&b.draw_index));

        for object in objects.iter() {
            object.object.draw(context);
        }
    }

    fn clone_box(&self) -> GMObjectBox {
        Box::new(self.clone())
    }
}
