
use crate::animation::GMAnimation;
use crate::spritesheet::GMSpriteSheet;
use crate::utils::{angle_point, in_rect};

use macroquad::window::{screen_width, screen_height};

use std::rc::Rc;
use std::f32::consts;

// TODO:
// - GMMultiSprite



#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GMCollisionShape {
    Rectangle,
    Circle,
}

pub trait GMSpriteT {
    fn clone_sprite(&self) -> GMSprite;
    fn draw(&self);
    fn draw_at(&self, x: f32, y: f32);
    fn update(&mut self);
    fn get_extend(&self) -> (f32, f32);
    fn get_state_id(&self) -> u32;
    fn get_active(&self) -> bool;
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_mid_x(&self) -> f32;
    fn get_mid_y(&self) -> f32;
    fn get_rotation(&self) -> f32;
    fn get_collision_shape(&self) -> GMCollisionShape;
    fn set_sheet(&mut self, sheet: &Rc<GMSpriteSheet>);
    fn set_animation(&mut self, animation: &GMAnimation);
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn set_mid_x(&mut self, x: f32);
    fn set_mid_y(&mut self, y: f32);
    fn set_vx(&mut self, vx: f32);
    fn set_vy(&mut self, vy: f32);
    fn set_active(&mut self, active: bool);
    fn set_collision_shape(&mut self, collision_shape: GMCollisionShape);
    fn set_state_id(&mut self, state_id: u32);
    fn set_rotation(&mut self, rotation: f32);
    fn set_rot_speed(&mut self, rot_speed: f32);
    fn rotate_to_point(&mut self, px: f32, py: f32);
    fn collides_with(&self, other: &GMSprite) -> bool;
    fn is_offscreen(&self) -> bool;
    fn wrap_around(&mut self);
    fn animation_finished(&self) -> bool;
    fn flip_x(&mut self, flip_x: bool);
    fn flip_y(&mut self, flip_y: bool);
    fn start_animation(&mut self);
    fn pause_animation(&mut self);
    fn resume_animation(&mut self);
    fn to_simple(&self) -> GMSpriteSimple;
}

pub struct GMSprite {
    sprite: Box<dyn GMSpriteT>,
}

impl GMSprite {
    pub fn new<T: 'static + GMSpriteT>(sprite: T) -> Self {
        Self {
            sprite: Box::new(sprite),
        }
    }
    pub fn draw(&self) {
        self.sprite.draw();
    }
    pub fn draw_at(&self, x: f32, y: f32) {
        self.sprite.draw_at(x, y);
    }
    pub fn update(&mut self) {
        self.sprite.update();
    }
    pub fn get_extend(&self) -> (f32, f32) {
        self.sprite.get_extend()
    }
    pub fn get_state_id(&self) -> u32 {
        self.sprite.get_state_id()
    }
    pub fn get_active(&self) -> bool {
        self.sprite.get_active()
    }
    pub fn get_x(&self) -> f32 {
        self.sprite.get_x()
    }
    pub fn get_y(&self) -> f32 {
        self.sprite.get_y()
    }
    pub fn get_mid_x(&mut self) -> f32 {
        self.sprite.get_mid_x()
    }
    pub fn get_mid_y(&mut self) -> f32 {
        self.sprite.get_mid_y()
    }
    pub fn get_rotation(&self) -> f32 {
        self.sprite.get_rotation()
    }
    pub fn get_collision_shape(&self) -> GMCollisionShape {
        self.sprite.get_collision_shape()
    }
    pub fn set_sheet(&mut self, sheet: &Rc<GMSpriteSheet>) {
        self.sprite.set_sheet(sheet);
    }
    pub fn set_animation(&mut self, animation: &GMAnimation) {
        self.sprite.set_animation(animation);
    }
    pub fn set_x(&mut self, x: f32) {
        self.sprite.set_x(x);
    }
    pub fn set_y(&mut self, y: f32) {
        self.sprite.set_y(y);
    }
    pub fn set_mid_x(&mut self, x: f32) {
        self.sprite.set_mid_x(x);
    }
    pub fn set_mid_y(&mut self, y: f32) {
        self.sprite.set_mid_y(y);
    }
    pub fn set_vx(&mut self, vx: f32) {
        self.sprite.set_vx(vx);
    }
    pub fn set_vy(&mut self, vy: f32) {
        self.sprite.set_vy(vy);
    }
    pub fn set_active(&mut self, active: bool) {
        self.sprite.set_active(active);
    }
    pub fn set_collision_shape(&mut self, collision_shape: GMCollisionShape) {
        self.sprite.set_collision_shape(collision_shape);
    }
    pub fn set_state_id(&mut self, state_id: u32) {
        self.sprite.set_state_id(state_id);
    }
    pub fn set_rotation(&mut self, rotation: f32) {
        self.sprite.set_rotation(rotation);
    }
    pub fn set_rot_speed(&mut self, rot_speed: f32) {
        self.sprite.set_rot_speed(rot_speed);
    }
    pub fn rotate_to_point(&mut self, px: f32, py: f32) {
        self.sprite.rotate_to_point(px, py);
    }
    pub fn collides_with(&self, other: &GMSprite) -> bool {
        self.sprite.collides_with(other)
    }
    pub fn is_offscreen(&self) -> bool {
        self.sprite.is_offscreen()
    }
    pub fn wrap_around(&mut self) {
        self.sprite.wrap_around()
    }
    pub fn animation_finished(&self) -> bool {
        self.sprite.animation_finished()
    }
    pub fn flip_x(&mut self, flip_x: bool) {
        self.sprite.flip_x(flip_x);
    }
    pub fn flip_y(&mut self, flip_y: bool) {
        self.sprite.flip_y(flip_y);
    }
    pub fn start_animation(&mut self) {
        self.sprite.start_animation();
    }
    pub fn pause_animation(&mut self) {
        self.sprite.pause_animation();
    }
    pub fn resume_animation(&mut self) {
        self.sprite.resume_animation();
    }
    pub fn to_simple(&self) -> GMSpriteSimple {
        self.sprite.to_simple()
    }
}

impl Clone for GMSprite {
    fn clone(&self) -> Self {
        self.sprite.clone_sprite()
    }
}

#[derive(Clone)]
pub struct GMSpriteSingle {
    sheet: Rc<GMSpriteSheet>,
    animation: GMAnimation,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    active: bool,
    collision_shape: GMCollisionShape,
    state_id: u32,
    flip_x: bool,
    flip_y: bool,
    rotation: f32,
    rot_speed: f32,
}

impl GMSpriteSingle {
    pub fn new(sheet: &Rc<GMSpriteSheet>, animation: &GMAnimation, x: f32, y: f32) -> Self {
        Self {
            sheet: sheet.clone(),
            animation: animation.clone(),
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            active: true,
            collision_shape: GMCollisionShape::Rectangle,
            state_id: 0,
            flip_x: false,
            flip_y: false,
            rotation: 0.0,
            rot_speed: 0.0,
        }
    }
    pub fn new_wrapped(sheet: &Rc<GMSpriteSheet>, animation: &GMAnimation, x: f32, y: f32) -> GMSprite {
        let sprite = Self::new(sheet, animation, x, y);
        GMSprite::new(sprite)
    }
}
impl GMSpriteT for GMSpriteSingle {
    fn clone_sprite(&self) -> GMSprite {
        let sprite = self.clone();
        GMSprite::new(sprite)
    }
    fn draw(&self) {
        if !self.active {
            return
        }
        let rect = self.animation.get_rect();
        self.sheet.draw_ex(&rect, self.x, self.y, self.flip_x, self.flip_y, self.rotation);
    }
    fn draw_at(&self, x: f32, y: f32) {
        if !self.active {
            return
        }
        let rect = self.animation.get_rect();
        self.sheet.draw_ex(&rect, x, y, self.flip_x, self.flip_y, self.rotation);
    }
    fn update(&mut self) {
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
    fn get_extend(&self) -> (f32, f32) {
        let rect = self.animation.get_rect();
        (rect.w, rect.h)
    }
    fn get_state_id(&self) -> u32 {
        self.state_id
    }
    fn get_active(&self) -> bool {
        self.active
    }
    fn get_x(&self) -> f32 {
        self.x
    }
    fn get_y(&self) -> f32 {
        self.y
    }
    fn get_mid_x(&self) -> f32 {
        self.x + (self.animation.get_rect().w / 2.0)
    }
    fn get_mid_y(&self) -> f32 {
        self.y + (self.animation.get_rect().h / 2.0)
    }
    fn get_rotation(&self) -> f32 {
        self.rotation
    }
    fn get_collision_shape(&self) -> GMCollisionShape {
        self.collision_shape
    }
    fn set_sheet(&mut self, sheet: &Rc<GMSpriteSheet>) {
        self.sheet = sheet.clone();
    }
    fn set_animation(&mut self, animation: &GMAnimation) {
        self.animation = animation.clone();
    }
    fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    fn set_mid_x(&mut self, x: f32) {
        self.x = x - (self.animation.get_rect().w / 2.0);
    }
    fn set_mid_y(&mut self, y: f32) {
        self.y = y - (self.animation.get_rect().h / 2.0);
    }
    fn set_vx(&mut self, vx: f32) {
        self.vx = vx;
    }
    fn set_vy(&mut self, vy: f32) {
        self.vy = vy;
    }
    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    fn set_collision_shape(&mut self, collision_shape: GMCollisionShape) {
        self.collision_shape = collision_shape;
    }
    fn set_state_id(&mut self, state_id: u32) {
        self.state_id = state_id;
    }
    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
    fn set_rot_speed(&mut self, rot_speed: f32) {
        self.rot_speed = rot_speed;
    }
    fn rotate_to_point(&mut self, px: f32, py: f32) {
        let a = angle_point(self.x, self.y, px, py);
        self.set_rotation(a);
    }
    fn collides_with(&self, other: &GMSprite) -> bool {
        if !self.active {
            return false
        }
        if !other.get_active() {
            return false
        }

        let (self_width, self_height) = self.get_extend();
        let (other_width, other_height) = other.get_extend();

        let sx1 = self.x;
        let sx2 = self.x + self_width;
        let sy1 = self.y;
        let sy2 = self.y + self_height;

        let ox1 = other.get_x();
        let ox2 = other.get_x() + other_width;
        let oy1 = other.get_y();
        let oy2 = other.get_y() + other_height;

        use GMCollisionShape::*;

        match (self.collision_shape, other.get_collision_shape()) {
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
                let omx = other.get_x() + or;
                let omy = other.get_y() + or;

                let dx = smx - omx;
                let dy = smy - omy;

                let diff = dx.hypot(dy);
                return diff <= sr + or
            }
        };
        false
    }
    fn is_offscreen(&self) -> bool {
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
    fn wrap_around(&mut self) {
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
    fn animation_finished(&self) -> bool {
        self.animation.finished()
    }
    fn flip_x(&mut self, flip_x: bool) {
        self.flip_x = flip_x;
    }
    fn flip_y(&mut self, flip_y: bool) {
        self.flip_y = flip_y;
    }
    fn start_animation(&mut self) {
        self.animation.start();
    }
    fn pause_animation(&mut self) {
        self.animation.pause();
    }
    fn resume_animation(&mut self) {
        self.animation.resume();
    }
    fn to_simple(&self) -> GMSpriteSimple {
        GMSpriteSimple::new(
            self.x,
            self.y,
            &self.sheet,
            &self.animation
        )
    }
}

#[derive(Clone)]
pub struct GMSpriteMultiple {

}

#[derive(Clone)]
pub struct GMSpriteSimple {
    x: f32,
    y: f32,
    sprite_sheet: Rc<GMSpriteSheet>,
    animation: GMAnimation,
    flip_x: bool,
    flip_y: bool,
}

impl GMSpriteSimple {
    pub fn new(x: f32, y: f32, sprite_sheet: &Rc<GMSpriteSheet>, animation: &GMAnimation) -> Self {
        Self {
            x,
            y,
            sprite_sheet: sprite_sheet.clone(),
            animation: animation.clone(),
            flip_x: false,
            flip_y: false,
        }
    }
    pub fn draw(&self) {
        let rect = self.animation.get_rect();
        self.sprite_sheet.draw(&rect, self.x, self.y);
    }
    pub fn update(&mut self) {
        self.animation.next_frame();
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_extend(&self) -> (f32, f32) {
        let rect = self.animation.get_rect();
        (rect.w, rect.h)
    }
    pub fn set_sprite_sheet(&mut self, sprite_sheet: &Rc<GMSpriteSheet>) {
        self.sprite_sheet = sprite_sheet.clone();
    }
    pub fn set_animation(&mut self, animation: &GMAnimation) {
        self.animation = animation.clone()
    }
    pub fn flip_x(&mut self, flip_x: bool) {
        self.flip_x = flip_x;
    }
    pub fn flip_y(&mut self, flip_y: bool) {
        self.flip_y = flip_y;
    }
    pub fn start_animation(&mut self) {
        self.animation.start();
    }
}
