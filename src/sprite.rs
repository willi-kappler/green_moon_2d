
use crate::animation::GMAnimationT;
use crate::spritesheet::GMSpriteSheet;

use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GMCollisionShape {
    GMRectangle,
    GMCircle,
}

pub fn between(a: f32, b: f32, c: f32) -> bool {
    a <= b && b <= c
}

pub fn in_rect(x1: f32, x2: f32, y1: f32, y2: f32, xp: f32, yp: f32) -> bool {
    between(x1, xp, x2) && between(y1, yp, y2)
}

pub struct GMSprite {
    pub(crate) sheet: Rc<GMSpriteSheet>,
    pub(crate) animation: Box<dyn GMAnimationT>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) vx: f32,
    pub(crate) vy: f32,
    pub(crate) visible: bool,
    pub(crate) active: bool,
    pub(crate) collision_shape: GMCollisionShape,
}

impl GMSprite {
    pub fn new(sheet: &Rc<GMSpriteSheet>, animation: Box<dyn GMAnimationT>, x: f32, y: f32) -> Self {
        Self {
            sheet: sheet.clone(),
            animation,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            visible: true,
            active: true,
            collision_shape: GMCollisionShape::GMRectangle,
        }
    }
    pub fn clone_sprite(&self) -> Self {
        Self {
            sheet: self.sheet.clone(),
            animation: self.animation.clone_animation(),
            x: self.x,
            y: self.y,
            vx: self.vx,
            vy: self.vy,
            visible: self.visible,
            active: self.active,
            collision_shape: self.collision_shape,
        }
    }
    pub fn draw(&self) {
        if self.visible {
            let rect = self.animation.get_rect();
            self.sheet.draw(&rect, self.x, self.y);
        }
    }
    pub fn update(&mut self) {
        if self.active {
            self.animation.next_frame();
            self.x += self.vx;
            self.y += self.vy;
            }
    }
    pub fn get_extend(&self) -> (f32, f32) {
        let rect = self.animation.get_rect();
        (rect.w, rect.h)
    }
    pub fn collides_with(&self, other: &GMSprite) -> bool {
        let (self_width, self_height) = self.get_extend();
        let (other_width, other_height) = other.get_extend();

        let sx1 = self.x;
        let sx2 = self.x + self_width;
        let sy1 = self.y;
        let sy2 = self.y + self_height;

        let ox1 = other.x;
        let ox2 = other.x + other_width;
        let oy1 = other.y;
        let oy2 = other.y + other_height;

        use GMCollisionShape::*;

        match (self.collision_shape, other.collision_shape) {
            (GMRectangle, GMRectangle) => {
                if in_rect(sx1, sx2, sy1, sy2, ox1, oy1) {
                    return true
                } else if in_rect(sx1, sx2, sy1, sy2, ox1, oy2) {
                    return true
                } else if in_rect(sx1, sx2, sy1, sy2, ox2, oy1) {
                    return true
                } else if in_rect(sx1, sx2, sy1, sy2, ox2, oy2) {
                    return true
                }
            }
            (GMRectangle, GMCircle) => {
                todo!();
            }
            (GMCircle, GMRectangle) => {
                todo!();
            }
            (GMCircle, GMCircle) => {
                let sr = self_width / 2.0;
                let or = other_width / 2.0;

                let smx = self.x + sr;
                let smy = self.y + sr;
                let omx = other.x + or;
                let omy = other.y + or;

                let dx = smx - omx;
                let dy = smy - omy;

                let diff = dx.hypot(dy);
                return diff <= sr + or
            }
        };
        false
    }
    pub fn set_sheet(&mut self, sheet: &Rc<GMSpriteSheet>) {
        self.sheet = sheet.clone();
    }
    pub fn set_animation(&mut self, animation: Box<dyn GMAnimationT>) {
        self.animation = animation;
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn set_vx(&mut self, vx: f32) {
        self.vx = vx;
    }
    pub fn set_vy(&mut self, vy: f32) {
        self.vy = vy;
    }
}
