

use crate::math::{GMSize, GMVec2D, GMRectangle, GMCircle};

#[derive(Clone, Debug)]
pub enum GMCollisionShape {
    AABB(GMSize),
    Circle(f32),
}

impl Default for GMCollisionShape {
    fn default() -> Self {
        Self::AABB(GMSize::default())
    }
}

pub fn intersects(pos1: &GMVec2D, shape1: &GMCollisionShape, pos2: &GMVec2D, shape2: &GMCollisionShape) -> bool {
    use GMCollisionShape::*;

    match (shape1, shape2) {
        (AABB(size1), AABB(size2)) => {
            let rect1 = GMRectangle::new3(pos1, size1);
            let rect2 = GMRectangle::new3(pos2, size2);

            rect1.rect_intersect(&rect2) ||
            rect2.rect_intersect(&rect1)
        }
        (AABB(size1), Circle(radius2)) => {
            todo!("intersects: {:?}, {:?}", size1, radius2);
        }
        (Circle(radius1), AABB(size2)) => {
            todo!("intersects: {:?}, {:?}", radius1, size2);
        }
        (Circle(radius1), Circle(radius2)) => {
            let circle1 = GMCircle::new2(pos1, *radius1);
            let circle2 = GMCircle::new2(pos2, *radius2);

            todo!("intersects: {:?}, {:?}", circle1, circle2);
        }
    }
}
