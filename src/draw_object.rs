


use crate::GMContext;
use crate::GMError;


pub trait GMDrawObjectT {
    fn update(&mut self, _context: &mut GMContext) -> Result<(), GMError> {
        Ok(())
    }

    fn draw(&self, _context: &mut GMContext) {}

    fn get_z_index(&self) -> i32 {
        0
    }

    fn get_name(&self) -> &str;

    fn get_groups(&self) -> &[String] {
        &[]
    }

    fn box_clone(&self) -> Box<dyn GMDrawObjectT>;
}

impl Clone for Box<dyn GMDrawObjectT> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub struct GMDrawObjectManager {
    draw_objects: Vec<Box<dyn GMDrawObjectT>>,
}

impl GMDrawObjectManager {
    pub fn new() -> Self {
        Self {
            draw_objects: Vec::new(),
        }
    }

    fn get_draw_object_index(&self, name: &str) -> Option<usize> {
        self.draw_objects.iter().position(|object| object.get_name() == name)
    }

    pub fn add_draw_object<O: 'static + GMDrawObjectT>(&mut self, object: O) -> Result<(), GMError> {
        let name = object.get_name();

        match self.get_draw_object_index(name) {
            Some(_) => {
                Err(GMError::DrawObjectAlreadyExists(name.to_string()))
            }
            None => {
                self.draw_objects.push(Box::new(object));

                Ok(())
            }
        }
    }

    pub fn remove_draw_object(&mut self, name: &str) -> Result<(), GMError> {
        match self.get_draw_object_index(name) {
            Some(index) => {
                self.draw_objects.swap_remove(index);
                Ok(())
            }
            None => {
                Err(GMError::DrawObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn replace_draw_object<O: 'static + GMDrawObjectT>(&mut self, object: O) -> Result<(), GMError> {
        let name = object.get_name();

        match self.get_draw_object_index(name) {
            Some(index) => {
                self.draw_objects[index] = Box::new(object);

                Ok(())
            }
            None => {
                Err(GMError::DrawObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn update(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        for object in self.draw_objects.iter_mut() {
            object.update(context)?;
        }

        Ok(())

    }

    pub fn draw(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        // Sort all drawable objects by z order before drawing them
        self.draw_objects.sort_by_key(|object| object.get_z_index());

        for object in self.draw_objects.iter() {
            object.draw(context);
        }

        Ok(())
    }
}
