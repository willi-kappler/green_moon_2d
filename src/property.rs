






/*

pub trait GMPropertyT {
    fn set_property1(&mut self, name: &str, value: &dyn Any) -> Result<(), GMError>;
    fn set_property2(&mut self, name: &str, value: Box<dyn Any>) -> Result<(), GMError>;
    fn get_property_ref(&self, name: &str) -> Result<&dyn Any, GMError>;
    fn get_property_mut_ref(&self, name: &str) -> Result<&mut dyn Any, GMError>;
}

pub struct GMPropertyContainer {
    properties: Vec<(String, Box<dyn Any>)>,
}

impl GMPropertyContainer {
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
        }
    }

    pub fn set_property<A: Any>(&mut self, name: &str, value: A) {
        for (name_old, value_old) in self.properties.iter_mut() {
            if name == name_old {
                *value_old = Box::new(value);
                return
            }
        }

        self.properties.push((name.to_string(), Box::new(value)));
    }

    pub fn get_property<T: 'static>(&mut self, name: &str) -> Option<&T> {
        for (name_, value) in self.properties.iter() {
            if name_ == name {
                return value.downcast_ref::<T>()
            }
        }

        None
    }

    pub fn get_property_mut<T: 'static>(&mut self, name: &str) -> Option<&mut T> {
        for (name_, value) in self.properties.iter_mut() {
            if name_ == name {
                return value.downcast_mut::<T>()
            }
        }

        None
    }

    pub fn del_property(&mut self, name: &str) {
        let pos = self.properties.iter().position(|(name_, _)| name_ == name);

        if let Some(index) = pos {
            self.properties.swap_remove(index);
        }
    }

    pub fn has_property(&self, name: &str) -> bool {
        for (name_, _) in self.properties.iter() {
            if name_ == name {
                return true
            }
        }

        false
    }
}

*/
