

pub mod collision;
pub mod event;
pub mod green_moon_2d;
pub mod position;
pub mod runtime;
pub mod settings;



use runtime::{GM_Runtime};

pub trait GM_Process_T {
    fn process(&mut self, runtime: &mut GM_Runtime) {

    }
}

pub trait GM_Update_T {
    fn update(&mut self, runtime: &mut GM_Runtime) {

    }
}

pub trait GM_Draw_T {
    fn draw(&mut self, runtime: &mut GM_Runtime) {

    }
}

pub trait GM_Active_T {
    fn is_active(&self) -> bool;

    fn set_active(&mut self, active: bool);
}
