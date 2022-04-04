


pub trait GMDrawT {
    fn update(&mut self) {

    }

    fn draw(&self);

    fn get_z_index(&self) -> i32 {
        0
    }

    fn box_clone(&self) -> Box<dyn GMDrawT>;
}
