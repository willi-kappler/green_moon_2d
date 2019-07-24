


#[derive(Clone)]
pub struct GM_Position {
    x: u32,
    y: u32,
}

impl GM_Position {
    pub fn new(x: u32, y: u32) -> GM_Position {
        GM_Position {
            x,
            y,
        }
    }

    pub fn set_position(&mut self, other: &GM_Position) {
        self.x = other.x;
        self.y = other.y;
    }

    pub fn get_x(&self) -> u32 {
        self.x
    }

    pub fn get_y(&self) -> u32 {
        self.y
    }

    pub fn set_x(&mut self, x: u32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u32) {
        self.y = y;
    }

    pub fn inc_x(&mut self, dx: u32) {
        self.x += dx;
    }

    pub fn inc_y(&mut self, dy: u32) {
        self.y += dy;
    }

    pub fn dec_x(&mut self, dx: u32) {
        self.x -= dx;
    }

    pub fn dec_y(&mut self, dy: u32) {
        self.y -= dy;
    }
}
