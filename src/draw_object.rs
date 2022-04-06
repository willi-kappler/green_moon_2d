


use std::any::Any;


pub enum GMMessage {
    SetX(f32),
    SetY(f32),
    SetXY(f32, f32),
    GetX,
    GetY,
    GetXY,
    CustomString(String),
    CustomAny(Box<dyn Any>),
}

pub enum GMAnswer {
    X(f32),
    Y(f32),
    XY(f32, f32),

    CustomString(String),
    CustomAny(Box<dyn Any>),
}


pub trait GMDrawT {
    fn update(&mut self) {
    }

    fn draw(&self);

    fn get_z_index(&self) -> i32 {
        0
    }

    fn box_clone(&self) -> Box<dyn GMDrawT>;

    fn send_message1(&mut self, message: GMMessage);

    fn send_message2(&mut self, message: GMMessage) -> GMAnswer;
}
