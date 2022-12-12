
use std::collections::HashSet;

use log::debug;

/*

use crate::effect::{GMEffectManager, GMEffectT};
use crate::object_manager::{GMObjectBaseT, GMObjectManager};
use crate::context::GMContext;
use crate::data::GMData;
use crate::util::error_panic;

use crate::return_name_and_groups;

pub struct GMBorderBase {
    pub name: String,
    pub groups: HashSet<String>,
}

impl GMBorderBase {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            groups: HashSet::new(),
        }
    }
}

impl GMObjectBaseT for GMBorderBase {
    fn send_message(&mut self, message: &str, data: GMData, context: &mut crate::GMContext) {
        match message {
            "" => {
                todo!()
            }
            _ => {
                error_panic(&format!("GMBorderBase::send_message(), unknown message: '{}'", message))
            }
        }
    }

    fn update(&mut self, _context: &mut GMContext) {
    }

    fn draw(&self, _context: &mut GMContext) {
    }

    return_name_and_groups!();
}

pub type GMBorder = GMObjectManager<GMBorderBase>;

impl GMBorder {
    pub fn new() -> Self {
        Self {
            base: GMBorderBase::new(),
            effects: GMEffectManager::new(),
        }
    }
}

pub struct GMBorderBuilder {
    border: GMBorder,
}
*/
