

pub trait GMMovementT {
    fn update(&mut self, x: f32, y: f32) -> (f32, f32);
    fn set_active(&mut self, active: bool);
}

pub struct GMConstPosition {
}

impl GMConstPosition {
    pub fn new() -> Self {
        Self {}
    }
}

impl GMMovementT for GMConstPosition {
    fn update(&mut self, x: f32, y: f32) -> (f32, f32) {
        (x, y)
    }
    fn set_active(&mut self, _active: bool) {
    }
}

pub struct GMConstVelocity {
    pub vx: f32,
    pub vy: f32,
    active: bool,
}

impl GMConstVelocity {
    pub fn new(vx: f32, vy: f32) -> Self {
        Self {
            vx,
            vy,
            active: false,
        }
    }
}

impl GMMovementT for GMConstVelocity {
    fn update(&mut self, x: f32, y: f32) -> (f32, f32) {
        if self.active {
            (x + self.vx, y + self.vy)
        } else {
            (x, y)
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

pub struct GMConstAcceleration {
    pub vx: f32,
    pub vy: f32,
    pub ax: f32,
    pub ay: f32,
    active: bool,
}

impl GMConstAcceleration {
    pub fn new(vx: f32, vy: f32, ax: f32, ay: f32) -> Self {
        Self {
            vx,
            vy,
            ax,
            ay,
            active: false,
        }
    }
}

impl GMMovementT for GMConstAcceleration {
    fn update(&mut self, x: f32, y: f32) -> (f32, f32) {
        if self.active {
            self.vx += self.ax;
            self.vy += self.ay;
            (x + self.vx, y + self.vy)
        } else {
            (x, y)
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

pub struct GMWrapAround {
    screen_width: f32,
    screen_height: f32,
    item_width: f32,
    item_height: f32,
    active: bool,
}

impl GMWrapAround {
    pub fn new(screen_width: f32, screen_height: f32, item_width: f32, item_height: f32) -> Self {
        Self {
            screen_width,
            screen_height,
            item_width,
            item_height,
            active: false,
        }
    }
}

impl GMMovementT for GMWrapAround {
    fn update(&mut self, x: f32, y: f32) -> (f32, f32) {
        let mut new_x = x;
        let mut new_y = y;

        if x > self.screen_width {
            new_x = -self.item_width;
        } else if x < -self.item_width {
            new_x = self.screen_width;
        }

        if y > self.screen_height {
            new_y = -self.item_height;
        } else if y < -self.item_height {
            new_y = self.screen_height;
        }

        (new_x, new_y)
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}
