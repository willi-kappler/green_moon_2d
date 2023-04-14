

use crate::context::GMContext;

pub enum GMMessage {

}

pub enum GMProperty {

}

pub enum GMGetProperty {

}




pub trait GMObjectT {
    fn update(&mut self, context: &mut GMContext);

    fn get_name(&self) -> &str;

    fn send_message(&mut self, message: GMMessage);

    fn set_property(&mut self, property: GMProperty);

    fn get_property(&self, property: GMGetProperty) -> Option<GMProperty>;
}

pub trait GMObjectDrawT {
    fn update(&mut self, context: &mut GMContext);

    fn get_name(&self) -> &str;

    fn send_message(&mut self, message: GMMessage);

    fn set_property(&mut self, property: GMProperty);

    fn get_property(&self, property: GMGetProperty) -> Option<GMProperty>;

    fn draw(&self, context: &mut GMContext);

    fn get_z_index(&self) -> i32;
}
