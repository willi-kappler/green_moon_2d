


use std::any::Any;

use log::debug;

use crate::GMContext;
use crate::GMError;
use crate::sprite::GMSprite;
use crate::text::GMText;


#[derive(Debug)]
pub enum GMDrawRefType<'a> {
    Sprite(&'a GMSprite),
    Text(&'a GMText),

    Custom(&'a dyn Any)
}

#[derive(Debug)]
pub enum GMDrawMutRefType<'a> {
    Sprite(&'a mut GMSprite),
    Text(&'a mut GMText),

    Custom(&'a mut dyn Any)
}


pub trait GMDrawT {
    fn update(&mut self, _context: &mut GMContext) -> Result<(), GMError> {
        Ok(())
    }

    fn draw(&self, _context: &mut GMContext) {}

    fn set_active(&mut self, _active: bool) {}

    fn get_z_index(&self) -> i32 {
        0
    }

    fn set_z_index(&mut self, _z_index: i32) {}

    fn box_clone(&self) -> Box<dyn GMDrawT>;

    fn cast_ref(&self) -> GMDrawRefType;

    fn cast_mut_ref(&mut self) -> GMDrawMutRefType;
}

impl Clone for Box<dyn GMDrawT> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub struct GMDrawContainer {
    pub draw_objects: Vec<(String, Box<dyn GMDrawT>)>,
}

impl GMDrawContainer {
    pub fn new() -> Self {
        Self {
            draw_objects: Vec::new(),
        }
    }

    pub fn has_draw_object(&self, name: &str) -> bool {
        debug!("GMDrawContainer::has_draw_object(), name: '{}'", name);

        self.draw_objects.iter().any(|(o_name, _)| o_name == name)
    }

    pub fn add_draw_object1<D: 'static + GMDrawT>(&mut self, name: &str, object: D) -> Result<(), GMError> {
        debug!("GMDrawContainer::add_draw_object1(), name: '{}'", name);

        if self.has_draw_object(name) {
            return Err(GMError::DrawObjectAlreadyExists(name.to_string()))
        }

        self.draw_objects.push((name.to_string(), Box::new(object)));

        Ok(())
    }

    pub fn add_draw_object2(&mut self, name: &str, object: Box<dyn GMDrawT>) -> Result<(), GMError> {
        debug!("GMDrawContainer::add_draw_object2(), name: '{}'", name);

        if self.has_draw_object(name) {
            return Err(GMError::DrawObjectAlreadyExists(name.to_string()))
        }

        self.draw_objects.push((name.to_string(), object));

        Ok(())
    }

    pub fn get_draw_object(&self, name: &str) -> Result<&Box<dyn GMDrawT>, GMError> {
        debug!("GMDrawContainer::get_draw_object(), name: '{}'", name);

        for (o_name, object) in self.draw_objects.iter() {
            if o_name == name {
                return Ok(object)
            }
        }

        Err(GMError::DrawObjectNotFound(name.to_string()))
    }

    pub fn get_draw_object_mut(&mut self, name: &str) -> Result<&mut Box<dyn GMDrawT>, GMError> {
        debug!("GMDrawContainer::get_draw_object_mut(), name: '{}'", name);

        for (o_name, object) in self.draw_objects.iter_mut() {
            if o_name == name {
                return Ok(object)
            }
        }

        Err(GMError::DrawObjectNotFound(name.to_string()))
    }

    pub fn get_draw_object_clone(&self, name: &str) -> Result<Box<dyn GMDrawT>, GMError> {
        debug!("GMDrawContainer::get_draw_object_clone(), name: '{}'", name);

        for (o_name, object) in self.draw_objects.iter() {
            if o_name == name {
                return Ok(object.box_clone())
            }
        }

        Err(GMError::DrawObjectNotFound(name.to_string()))
    }

    pub fn remove_draw_object(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMDrawContainer::remove_draw_object(), name: '{}'", name);

        match self.draw_objects.iter().position(|(o_name, _)| o_name == name) {
            Some(index) => {
                self.draw_objects.remove(index);
                Ok(())
            }
            None => {
                Err(GMError::DrawObjectNotFound(name.to_string()))
            }
        }
    }

    pub fn add_sprite(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMDrawContainer::add_sprite(), name: '{}'", name);

        if self.has_draw_object(name) {
            return Err(GMError::SpriteAlreadyExists(name.to_string()))
        }

        todo!();

        // Ok(())
    }

    pub fn add_text(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMDrawContainer::add_text(), name: '{}'", name);

        if self.has_draw_object(name) {
            return Err(GMError::TextAlreadyExists(name.to_string()))
        }

        todo!();

        // Ok(())
    }

    pub fn update(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        for (_, object) in self.draw_objects.iter_mut() {
            object.update(context)?;
        }

        Ok(())

    }

    pub fn draw(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        // Sort all drawable objects by z order before drawing them
        self.draw_objects.sort_by_key(|(_, object)| object.get_z_index());

        for (_, object) in self.draw_objects.iter() {
            object.draw(context);
        }

        Ok(())
    }
}
