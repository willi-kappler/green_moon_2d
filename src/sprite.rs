
use crate::animation::GMAnimation;
use crate::spritesheet::GMSpriteSheet;

use macroquad::window::{screen_width, screen_height};

use std::rc::Rc;
use std::f32::consts;

// TODO:
// - Sprite trait: GMSpriteT
// - Remove GMSprite Part use GMMultiSprite
// - Rename GMSprite to GMSpriteSingle
// - Use GMSprite as wrapper for GMSpriteT


pub fn between(a: f32, b: f32, c: f32) -> bool {
    a <= b && b <= c
}

pub fn in_rect(x1: f32, x2: f32, y1: f32, y2: f32, xp: f32, yp: f32) -> bool {
    between(x1, xp, x2) && between(y1, yp, y2)
}

pub fn dist_angle(x1: f32, y1: f32, x2: f32, y2: f32) -> (f32, f32) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dist = dx.hypot(dy);
    let angle = (dy / dx).atan();

    (dist, angle)
}

pub fn angle(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dy / dx).atan()
}

pub trait GMSpriteT {
    // TODO: Add methods from GMSprite here
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GMCollisionShape {
    Rectangle,
    Circle,
}

#[derive(Clone)]
pub struct GMSprite {
    sheet: Rc<GMSpriteSheet>,
    animation: GMAnimation,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    active: bool,
    collision_shape: GMCollisionShape,
    state_id: u32,
    flipx: bool,
    flipy: bool,
    rotation: f32,
    rot_speed: f32,
}

impl GMSprite {
    pub fn new(sheet: &Rc<GMSpriteSheet>, animation: GMAnimation, x: f32, y: f32) -> Self {
        Self {
            sheet: sheet.clone(),
            animation,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            active: true,
            collision_shape: GMCollisionShape::Rectangle,
            state_id: 0,
            flipx: false,
            flipy: false,
            rotation: 0.0,
            rot_speed: 0.0,
        }
    }
    pub fn draw(&self) {
        if !self.active {
            return
        }
        let rect = self.animation.get_rect();
        self.sheet.draw_ex(&rect, self.x, self.y, self.flipx, self.flipy, self.rotation);
    }
    pub fn update(&mut self) {
        if !self.active {
            return
        }

        self.animation.next_frame();
        self.x += self.vx;
        self.y += self.vy;
        self.rotation += self.rot_speed;
        if self.rotation > consts::TAU {
            self.rotation -= consts::TAU;
        } else if self.rotation < consts::TAU {
            self.rotation += consts::TAU;
        }

    }
    pub fn get_extend(&self) -> (f32, f32) {
        let rect = self.animation.get_rect();
        (rect.w, rect.h)
    }
    pub fn get_state_id(&self) -> u32 {
        self.state_id
    }
    pub fn get_active(&self) -> bool {
        self.active
    }
    pub fn collides_with(&self, other: &GMSprite) -> bool {
        if !self.active {
            return false
        }
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
            (Rectangle, Rectangle) => {
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
            (Rectangle, Circle) => {
                todo!();
            }
            (Circle, Rectangle) => {
                todo!();
            }
            (Circle, Circle) => {
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
    pub fn set_animation(&mut self, animation: GMAnimation) {
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
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    pub fn set_collision_shape(&mut self, collision_shape: GMCollisionShape) {
        self.collision_shape = collision_shape;
    }
    pub fn set_state_id(&mut self, state_id: u32) {
        self.state_id = state_id;
    }
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
    pub fn set_rot_speed(&mut self, rot_speed: f32) {
        self.rot_speed = rot_speed;
    }
    pub fn rotate_to_point(&mut self, px: f32, py: f32) {
        let a = angle(self.x, self.y, px, py);
        self.set_rotation(a);
    }
    pub fn is_offscreen(&self) -> bool {
        let (width, height) = self.get_extend();

        if self.x + width < 0.0 {
            return true;
        } else if self.x > screen_width() {
            return true;
        } else if self.y + height < 0.0 {
            return true;
        } else if self.y > screen_height() {
            return true;
        }

        false
    }
    pub fn wrap_around(&mut self) {
        let (width, height) = self.get_extend();
        let screen_w = screen_width();
        let screen_h = screen_height();
        let x2 = self.x + width;
        let y2 = self.y + height;

        if x2 < 0.0 {
            self.x = screen_w + x2;
        } else if self.x > screen_w {
            self.x = self.x - x2;
        } else if y2 < 0.0 {
            self.x = screen_h + y2;
        } else if self.y > screen_h {
            self.y = self.y - y2;
        }
    }
    pub fn animation_finished(&self) -> bool {
        self.animation.finished()
    }
    pub fn flipx(&mut self, flipx: bool) {
        self.flipx = flipx;
    }
    pub fn flipy(&mut self, flipy: bool) {
        self.flipy = flipy;
    }
    pub fn start_animation(&mut self) {
        self.animation.start();
    }
    pub fn pause_animation(&mut self) {
        self.animation.pause();
    }
    pub fn resume_animation(&mut self) {
        self.animation.resume();
    }
}

#[derive(Clone)]
pub struct GMSpriteMultiple {

}
