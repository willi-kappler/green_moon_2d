use std::ops::{Add, Sub, Mul};



#[derive(Copy, Clone, Debug)]
pub struct GMVec2D {
    pub x: f32,
    pub y: f32,
}

impl GMVec2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn len(&self) -> f32 {
        self.len2().sqrt()
    }

    pub fn len2(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn norm(&mut self) {
        let l = self.len();

        self.x  = self.x / l;
        self.y  = self.y / l;
    }

    pub fn norm_new(&self) -> Self {
        let l = self.len();

        Self {
            x: self.x / l,
            y: self.y / l,
        }
    }

    pub fn set1(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set2(&mut self, other: &GMVec2D) {
        self.x = other.x;
        self.y = other.y;
    }

    pub fn set3(&mut self, other: GMVec2D) {
        self.x = other.x;
        self.y = other.y;
    }

    pub fn add1(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn add2(&mut self, other: &GMVec2D) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn sub1(&mut self, x: f32, y: f32) {
        self.x -= x;
        self.y -= y;
    }

    pub fn sub2(&mut self, other: &GMVec2D) {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn mul2(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }

    pub fn angle() -> f32 {
        todo!("Return angle between two vectors");
    }
}

impl Default for GMVec2D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}

impl Add for GMVec2D {
    type Output = GMVec2D;

    fn add(self, rhs: Self) -> Self::Output {
        GMVec2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for GMVec2D {
    type Output = GMVec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        GMVec2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for GMVec2D {
    type Output = GMVec2D;

    fn mul(self, rhs: f32) -> Self::Output {
        GMVec2D::new(self.x * rhs, self.y * rhs)
    }
}

impl From<(f32, f32)> for GMVec2D {
    fn from((x, y): (f32, f32)) -> Self {
        GMVec2D { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GMSize {
    pub width: f32,
    pub height: f32,
}

impl GMSize {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }
}

impl Default for GMSize {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }
}

impl From<(f32, f32)> for GMSize {
    fn from((width, height): (f32, f32)) -> Self {
        GMSize { width, height }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GMRectangle {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

impl GMRectangle {
    pub fn new(x1: f32, x2: f32, y1: f32, y2: f32) -> Self {
        Self {
            x1, x2, y1, y2,
        }
    }

    pub fn new2(point: &GMVec2D, size: &GMSize) -> Self {
        Self {
            x1: point.x,
            x2: point.x + size.width,
            y1: point.y,
            y2: point.y + size.width,
        }
    }

    // Center point
    pub fn new3(point: &GMVec2D, size: &GMSize) -> Self {
        let w2 = size.width / 2.0;
        let h2 = size.height / 2.0;

        Self {
            x1: point.x - w2,
            x2: point.x + w2,
            y1: point.y - h2,
            y2: point.y + h2,
        }
    }

    pub fn point_inside(&self, x: f32, y: f32) -> bool {
        (self.x1 <= x) && (x <= self.x2) && (self.y1 <= y) && (y <= self.y2)
    }

    pub fn point_inside2(&self, point: &GMVec2D) -> bool {
        self.point_inside(point.x, point.y)
    }

    pub fn rect_intersect(&self, other: &GMRectangle) -> bool {
        self.point_inside(other.x1, other.y1) ||
        self.point_inside(other.x1, other.y2) ||
        self.point_inside(other.x2, other.y1) ||
        self.point_inside(other.x2, other.y2)
    }

    // TODO: intersect return points
}

impl From<(f32, f32, f32, f32)> for GMRectangle {
    fn from((x1, x2, y1, y2): (f32, f32, f32, f32)) -> Self {
        GMRectangle { x1, x2, y1, y2 }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct GMCircle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl GMCircle {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self {
            x, y, radius,
        }
    }

    pub fn new2(point: &GMVec2D, radius: f32) -> Self {
        Self {
            x: point.x,
            y: point.y,
            radius,
        }
    }

    pub fn point_inside(&self, x: f32, y: f32) -> bool {
        let dist = (self.x - x).hypot(self.y - y);

        dist <= self.radius
    }

    pub fn point_inside2(&self, point: &GMVec2D) -> bool {
        self.point_inside(point.x, point.y)
    }

    pub fn circ_intersect(&self, other: &GMCircle) -> bool {
        let dist = (self.x - other.x).hypot(self.y - other.y);

        dist <= (self.radius + other.radius)
    }

    pub fn circ_point(&self, x: f32, y: f32) -> GMVec2D {
        let mut vec = GMVec2D::new(x - self.x, y - self.y);
        vec.norm();
        vec.mul2(self.radius);
        vec.add1(self.x, self.y);
        vec
    }

    pub fn circ_point2(&self, point: &GMVec2D) -> GMVec2D {
        self.circ_point(point.x, point.y)
    }
}
