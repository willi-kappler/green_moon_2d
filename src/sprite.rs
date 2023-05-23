

use std::rc::Rc;
use std::fmt::Debug;

use log::debug;

use crate::texture::{GMTexture};
use crate::animation::GMAnimation;
use crate::math::{GMVec2D, GMSize};
use crate::context::GMContext;
use crate::object::GMObjectT;
use crate::value::GMValue;
use crate::object_manager::GMObjectManager;
use crate::util::error_panic;
use crate::message::GMMessage;


#[derive(Clone, Debug)]
pub struct GMSprite {
    pub position: GMVec2D,
    pub animation: GMAnimation,
    pub angle: f32,
    pub scale: f32,
    pub flipx: bool,
    pub flipy: bool,
    texture: Rc<GMTexture>,
    size: GMSize,
}

impl GMSprite {
    pub fn new<T: Into<GMVec2D>>(position: T, texture: &Rc<GMTexture>, animation: GMAnimation) -> GMSprite {
        let position = position.into();
        let (width, height) = texture.get_unit_dimension();
        debug!("GMSprite::new(), position: '{:?}', width: '{:?}', height: '{:?}", position, width, height);

        GMSprite {
            position,
            texture: texture.clone(),
            animation,
            angle: 0.0,
            scale: 1.0,
            flipx: false,
            flipy: false,
            size: GMSize::new(width, height),
        }
    }

    pub fn set_texture(&mut self, texture: &Rc<GMTexture>) {
        self.texture = texture.clone();

        let (width, height) = self.texture.get_unit_dimension();
        self.size = GMSize::new(width, height);
    }

    pub fn get_texture(&self) -> &Rc<GMTexture> {
        &self.texture
    }

    pub fn get_size(&self) -> GMSize {
        self.size
    }
}

impl GMObjectT for GMSprite {
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {
        let tag = &message.tag;
        let method = &message.method;
        let value = message.value;

        match tag.as_str() {
            "" => {

            }
            "position" => {
                return self.position.send_message(method, value)
            }
            "animation" => {
                return self.animation.send_message(method, value)
            }
            "flipxy" => {
                // return self.flipxy.send_message(method, value);
            }
            _ => {
                error_panic(&format!("GMSprite::send_message: unknown tag '{:?}'", tag));
            }
        }

        GMValue::None
    }

    /*
    fn send_message(&mut self, message: GMMessage, context: &mut GMContext, object_manager: &GMObjectManager) -> GMValue {

        match message {
            // Custom sprite messages:
            GMMessage::Custom0(name) if name == "get_angle" => {
                self.angle.into()
            }
            GMMessage::Custom0(name) if name == "get_scale" => {
                self.scale.into()
            }
            GMMessage::Custom0(name) if name == "get_texture" => {
                GMValue::Any(self.texture.clone())
            }
            GMMessage::Custom0(name) if name == "get_size" => {
                GMValue::Any(Rc::new(self.size))
            }
            GMMessage::Custom0(name) if name == "get_size2" => {
                (self.size.width, self.size.height).into()
            }
            GMMessage::Custom1(name, GMValue::F32(angle)) if name == "set_angle" => {
                self.angle = angle;
                GMValue::None
            }
            GMMessage::Custom1(name, GMValue::F32(scale)) if name == "set_scale" => {
                self.scale = scale;
                GMValue::None
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_texture" => {
                let texture = value.downcast::<GMTexture>().unwrap();
                self.set_texture(&texture);
                GMValue::None
            }
        }
    }
    */

    fn update(&mut self, _context: &mut GMContext, _object_manager: &GMObjectManager) {
        self.animation.update();
    }

    fn draw(&self, context: &mut GMContext) {
        let index = self.animation.texture_index();
        let dx = self.position.x;
        let dy = self.position.y;

        self.texture.draw_opt(dx, dy, index, self.angle,
            self.scale, self.flipx, self.flipy, context);
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
