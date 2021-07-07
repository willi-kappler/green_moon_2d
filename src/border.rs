
pub trait GMBorderT {
    fn draw(&self);
    fn update(&mut self);
}

pub struct GMBorder {
    border: Box<dyn GMBorderT>,
}

impl GMBorder {
    pub fn new<T: 'static + GMBorderT>(border: T) -> Self {
        Self {
            border: Box::new(border),
        }
    }
    pub fn draw(&self) {
        self.border.draw();
    }
    pub fn update(&mut self) {
        self.border.update();
    }
}
