use std::rc::Rc;
use std::cell::RefCell;

use crate::error::GMError;


pub trait GMName {
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: &str);
    fn has_name(&self, name: &str) -> bool;
    fn has_prefix(&self, name: &str) -> bool;
}

pub struct GMResourceManager<T> {
    manager_name: String,
    items: Vec<Rc<RefCell<T>>>,
}

impl<T: GMName> GMResourceManager<T> {
    pub fn new(manager_name: &str) -> GMResourceManager<T> {
        GMResourceManager {
            manager_name: manager_name.to_string(),
            items: Vec::new(),
        }
    }

    pub fn has_item(&self, name: &str) -> bool {
        for item in self.items.iter() {
            let item2 = item.borrow();
            if item2.has_name(name) {
                return true
            }
        }

        false
    }

    pub fn add_item(&mut self, new_item: T) -> Result<(), GMError> {
        let new_name = new_item.get_name();

        if self.has_item(new_name) {
            return Err(GMError::ItemNameAlreadyInUse(self.manager_name.clone(), new_name.to_string()))
        }

        self.items.push(Rc::new(RefCell::new(new_item)));

        Ok(())
    }

    pub fn get_item(&self, item_name: &str) -> Result<Rc<RefCell<T>>, GMError> {
        for item in self.items.iter() {
            let item2 = item.borrow();
            if item2.has_name(item_name) {
                return Ok(item.clone())
            }
        }

        Err(GMError::ItemNotFound(self.manager_name.clone(), item_name.to_string()))
    }

    pub fn delete_item(&mut self, item_name: &str) -> Result<(), GMError> {
        let mut maybe_index = None;

        for (index, item) in self.items.iter().enumerate() {
            let item2 = item.borrow();
            if item2.has_name(item_name) {
                maybe_index = Some(index);
                break
            }
        }

        match maybe_index {
            Some(index) => {
                self.items.remove(index);
                Ok(())
            }
            None => {
                Err(GMError::ItemNotFound(self.manager_name.clone(), item_name.to_string()))
            }
        }
    }

    pub fn delete_with_prefix(&mut self, prefix: &str) -> Result<(), GMError> {
        // TODO
        Ok(())
    }
}
