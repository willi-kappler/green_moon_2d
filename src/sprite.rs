

use std::rc::Rc;
use std::fmt::Debug;

// use log::debug;

use crate::texture::{GMTexture};
use crate::animation::GMAnimation;
use crate::util::{error_panic, GMRepetition};
use crate::math::{GMVec2D, GMSize};
use crate::context::GMContext;
use crate::object::GMObjectT;
use crate::message::GMMessage;
use crate::value::GMValue;
use crate::object_manager::GMObjectManager;


#[derive(Clone, Debug)]
pub struct GMSprite {
    pub position: GMVec2D,
    pub animation: GMAnimation,
    pub angle: f32,
    pub scale: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    texture: Rc<GMTexture>,
    size: GMSize,
}

impl GMSprite {
    pub fn new<T: Into<GMVec2D>>(position: T, texture: Rc<GMTexture>, animation: GMAnimation) -> GMSprite {
        let position = position.into();

        let (width, height) = texture.get_unit_dimension();

        GMSprite {
            position,
            texture,
            animation,
            angle: 0.0,
            scale: 1.0,
            flip_x: false,
            flip_y: false,
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

        match message {
            GMMessage::AddPosition(vec) => {
                self.position += vec;
            }
            GMMessage::AddX(x) => {
                self.position.x += x;
            }
            GMMessage::AddY(y) => {
                self.position.y += y;
            }
            GMMessage::SetPosition(vec) => {
                self.position = vec;
            }
            GMMessage::SetX(x) => {
                self.position.x = x;
            }
            GMMessage::SetY(y) => {
                self.position.y = y;
            }
            GMMessage::GetPosition => {
                return GMValue::Position(self.position)
            }
            GMMessage::GetX => {
                return GMValue::F32(self.position.x)
            }
            GMMessage::GetY => {
                return GMValue::F32(self.position.y)
            }
            GMMessage::Tuple2(m1, m2) => {
                return self.send_tuple2_message(*m1, *m2, context, object_manager)
            }
            GMMessage::Tuple3(m1, m2, m3) => {
                return self.send_tuple3_message(*m1, *m2, *m3, context, object_manager)
            }
            GMMessage::Tuple4(m1, m2, m3, m4) => {
                return self.send_tuple4_message(*m1, *m2, *m3, *m4, context, object_manager)
            }
            GMMessage::Multiple(messages) => {
                return self.send_multi_message(messages, context, object_manager)
            }
            // Custom animation messages:
            GMMessage::Custom0(name) if name == "animation_finished" => {
                let result = self.animation.finished();
                return result.into()
            }
            GMMessage::Custom0(name) if name == "animation_reverse" => {
                self.animation.reverse();
            }
            GMMessage::Custom1(name, GMValue::USize(frame)) if name == "animation_set_frame" => {
                self.animation.current_frame = frame;
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "animation_set_repetition" => {
                let repetition = *value.downcast::<GMRepetition>().unwrap();
                self.animation.repetition = repetition;
            }
            // Custom sprite messages:
            GMMessage::Custom0(name) if name == "get_animation" => {
                return GMValue::Any(Rc::new(self.animation.clone()))
            }
            GMMessage::Custom0(name) if name == "get_angle" => {
                return self.angle.into()
            }
            GMMessage::Custom0(name) if name == "get_scale" => {
                return self.scale.into()
            }
            GMMessage::Custom0(name) if name == "get_flip_x" => {
                return self.flip_x.into()
            }
            GMMessage::Custom0(name) if name == "get_flip_y" => {
                return self.flip_y.into()
            }
            GMMessage::Custom0(name) if name == "get_flip_xy" => {
                return (self.flip_x, self.flip_y).into()
            }
            GMMessage::Custom0(name) if name == "get_texture" => {
                return GMValue::Any(self.texture.clone())
            }
            GMMessage::Custom0(name) if name == "get_size" => {
                return GMValue::Any(Rc::new(self.size))
            }
            GMMessage::Custom0(name) if name == "get_size2" => {
                return (self.size.width, self.size.height).into()
            }
            GMMessage::Custom0(name) if name == "toggle_flip_x" => {
                self.flip_x = !self.flip_x;
            }
            GMMessage::Custom0(name) if name == "toggle_flip_y" => {
                self.flip_y = !self.flip_y;
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_animation" => {
                let animation = (*value.downcast::<GMAnimation>().unwrap()).clone();
                self.animation = animation;
            }
            GMMessage::Custom1(name, GMValue::F32(angle)) if name == "set_angle" => {
                self.angle = angle;
            }
            GMMessage::Custom1(name, GMValue::F32(scale)) if name == "set_scale" => {
                self.scale = scale;
            }
            GMMessage::Custom1(name, GMValue::Bool(flip_x)) if name == "set_flip_x" => {
                self.flip_x = flip_x;
            }
            GMMessage::Custom1(name, GMValue::Bool(flip_y)) if name == "set_flip_y" => {
                self.flip_y = flip_y;
            }
            GMMessage::Custom1(name, GMValue::Tuple2(flip_x, flip_y)) if name == "set_flip_xy" => {
                if let GMValue::Bool(fx) = *flip_x {
                    if let GMValue::Bool(fy) = *flip_y {
                        self.flip_x = fx;
                        self.flip_y = fy;
                    }
                }
            }
            GMMessage::Custom1(name, GMValue::Any(value)) if name == "set_texture" => {
                let texture = value.downcast::<GMTexture>().unwrap();
                self.set_texture(&texture);
            }
            _ => {
                error_panic(&format!("Wrong message for GMSprite::send_message: {:?}", message))
            }
        }

        GMValue::None
    }

    fn update(&mut self, _context: &mut GMContext, _object_manager: &GMObjectManager) {
        self.animation.update();
    }

    fn draw(&self, context: &mut GMContext) {
        let index = self.animation.texture_index();
        let dx = self.position.x;
        let dy = self.position.y;

        self.texture.draw_opt(dx, dy, index, self.angle, self.scale, self.flip_x, self.flip_y, context);
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
