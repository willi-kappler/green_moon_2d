

pub struct GMMovementInner {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl GMMovementInner {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

pub trait GMMovementT {
    fn update(&mut self, movement_inner: &mut GMMovementInner);
    fn set_active(&mut self, active: bool);
    fn box_clone(&self) -> Box<dyn GMMovementT>;
}

pub struct GMConstVelocity {
    vx: f32,
    vy: f32,
    active: bool,
}

impl GMConstVelocity {
    pub fn new(vx: f32, vy: f32) -> Self {
        Self {
            vx,
            vy,
            active: true,
        }
    }
}

impl GMMovementT for GMConstVelocity {
    fn update(&mut self, movement_inner: &mut GMMovementInner) {
        if self.active {
            movement_inner.x += self.vx;
            movement_inner.y += self.vy;
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = GMConstVelocity {
            vx: self.vx,
            vy: self.vy,
            active: self.active,
        };

        Box::new(result)
    }
}

pub struct GMConstAcceleration {
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,
    active: bool,
}

impl GMConstAcceleration {
    pub fn new(vx: f32, vy: f32, ax: f32, ay: f32) -> Self {
        Self {
            vx,
            vy,
            ax,
            ay,
            active: true,
        }
    }
}

impl GMMovementT for GMConstAcceleration {
    fn update(&mut self, movement_inner: &mut GMMovementInner) {
        if self.active {
            self.vx += self.ax;
            self.vy += self.ay;
            movement_inner.x += self.vx;
            movement_inner.y += self.vy;
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = GMConstAcceleration {
            vx: self.vx,
            vy: self.vy,
            ax: self.ax,
            ay: self.ay,
            active: self.active,
        };

        Box::new(result)
    }
}

pub struct GMWrapAround {
    screen_width: f32,
    screen_height: f32,
    active: bool,
}

impl GMWrapAround {
    pub fn new(screen_width: f32, screen_height: f32, item_width: f32, item_height: f32) -> Self {
        Self {
            screen_width,
            screen_height,
            active: true,
        }
    }
}

impl GMMovementT for GMWrapAround {
    fn update(&mut self, movement_inner: &mut GMMovementInner) {
        let x = movement_inner.x;
        let y = movement_inner.y;

        if x > self.screen_width {
            movement_inner.x = -movement_inner.width;
        } else if x < -movement_inner.width {
            movement_inner.x = self.screen_width;
        }

        if y > self.screen_height {
            movement_inner.y = -movement_inner.height;
        } else if y < -movement_inner.height {
            movement_inner.y = self.screen_height;
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMMovementT> {
        let result = GMWrapAround {
            screen_width: self.screen_width,
            screen_height: self.screen_height,
            active: self.active,
        };

        Box::new(result)
    }
}

pub struct GMMovementConstVeloBounce {
    vx: f32,
    vy: f32,
    screen_width: f32,
    screen_height: f32,
    item_width: f32,
    item_height: f32,
    active: bool,
}

pub struct GMMovementConstAccelBounce {
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,
    screen_width: f32,
    screen_height: f32,
    item_width: f32,
    item_height: f32,
    active: bool,
}

pub struct GMMovementCircular {
    mx: f32,
    my: f32,
    radius: f32,
    angle: f32,
}
