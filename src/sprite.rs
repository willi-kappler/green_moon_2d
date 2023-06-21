

use std::rc::Rc;
use std::fmt::Debug;

use log::debug;

use crate::texture::{GMTexture};
use crate::animation::GMAnimation;
use crate::math::{GMVec2D, GMSize, GMFlipXY};
use crate::context::GMContext;
use crate::object::GMObjectT;
use crate::value::GMValue;
use crate::object_manager::GMObjectManager;
use crate::util::{error_panic, send_message_f32};
use crate::message::GMMessage;


#[derive(Clone, Debug)]
pub struct GMSprite {
    pub position: GMVec2D,
    pub animation: GMAnimation,
    pub angle: f32,
    pub scale: f32,
    pub flipxy: GMFlipXY,
    texture: Rc<GMTexture>,
    size: GMSize,
}

impl GMSprite {
    pub fn new<T: Into<GMVec2D>>(position: T, texture: &Rc<GMTexture>, animation: GMAnimation) -> GMSprite {
        let position = position.into();
        let (width, height) = texture.get_unit_dimension();
        debug!("GMSprite::new(), position: '{:?}', width: '{}', height: '{}", position, width, height);

        GMSprite {
            position,
            texture: texture.clone(),
            animation,
            angle: 0.0,
            scale: 1.0,
            flipxy: GMFlipXY::new(),
            size: GMSize::new(width, height),
        }
    }

    pub fn set_texture(&mut self, texture: &Rc<GMTexture>) {
        self.texture = texture.clone();

        self.reset_size();
    }

    pub fn get_texture(&self) -> &Rc<GMTexture> {
        &self.texture
    }

    pub fn get_size(&self) -> GMSize {
        self.size
    }

    pub fn reset_size(&mut self) {
        let (width, height) = self.texture.get_unit_dimension();
        self.size.width = width;
        self.size.height = height;
    }
}

impl GMObjectT for GMSprite {
    fn send_message(&mut self, mut message: GMMessage, _object_manager: &GMObjectManager) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();

        match tag.as_str() {
            "" => {
                match method {
                    "get" => {
                        return self.clone().into();
                    }
                    "set" => {
                        *self = message.value.into_sprite();
                    }
                    "get_texture" => {
                        return self.texture.clone().into();
                    }
                    "set_texture" => {
                        self.texture = message.value.into_texture();
                        self.reset_size();
                    }
                    "get_width" => {
                        return self.size.width.into();
                    }
                    "get_height" => {
                        return self.size.height.into();
                    }
                    _ => {
                        error_panic(&format!("GMSprite::send_message: unknown method '{}', no tag", method));
                    }
                }
            }
            "angle" => {
                return send_message_f32(&mut self.angle, method, message.value)
            }
            "scale" => {
                return send_message_f32(&mut self.scale, method, message.value)
            }
            "position" => {
                return self.position.send_message(method, message.value)
            }
            "animation" => {
                return self.animation.send_message(message)
            }
            "flipxy" => {
                return self.flipxy.send_message(message);
            }
            "size" => {
                return self.size.send_message(message);
            }
            _ => {
                error_panic(&format!("GMSprite::send_message: unknown tag '{}'", tag));
            }
        }

        GMValue::None
    }

    fn update(&mut self, _object_manager: &GMObjectManager, _context: &mut GMContext) {
        self.animation.update();
    }

    fn draw(&self, context: &mut GMContext) {
        let index = self.animation.texture_index();
        let dx = self.position.x;
        let dy = self.position.y;

        self.texture.draw_opt(dx, dy, index, self.angle,
            self.scale, self.flipxy.x, self.flipxy.y, context);
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
