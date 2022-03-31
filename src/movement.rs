

pub trait GMMovementT {
    fn update(&mut self, x: i32, y: i32) -> (i32, i32);
    fn set_active(&mut self, active: bool);
}

pub struct GMConstPos {
}

impl GMConstPos {
    pub fn new() -> Self {
        Self {}
    }
}

impl GMMovementT for GMConstPos {
    fn update(&mut self, x: i32, y: i32) -> (i32, i32) {
        (x, y)
    }
    fn set_active(&mut self, active: bool) {
    }
}
