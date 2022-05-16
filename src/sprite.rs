
// use std::rc::Rc;

// use crate::animation::{GMAnimationT};
// use crate::context::{GMUpdateContext, GMDrawContext};
// use crate::GMError;
// use crate::texture::GMTexture;


/*


#[derive(Debug, Clone)]
pub struct GMSpriteCommon {
    pub texture: Rc<GMTexture>,
    pub animations: Vec<Box<dyn GMAnimationT>>,
    pub current_animation: usize,
    pub flip_x: bool,
    pub flip_y: bool,
    pub draw_object_common: GMDrawObjectCommon,
}

impl GMSpriteCommon {
    pub fn new(texture: Rc<GMTexture>, name: &str, x: f32, y: f32, animation: Box<dyn GMAnimationT>) -> Self {
        let (width, height) = texture.get_unit_dimension();

        Self {
            texture,
            animations: vec![animation],
            current_animation: 0,
            flip_x: false,
            flip_y: false,
            draw_object_common: GMDrawObjectCommon::new(name, x, y, width, height),
        }
    }

    pub fn update(&mut self) {
        if self.draw_object_common.active {
            self.draw_object_common.update();
            self.animations[self.current_animation].update();
        }
    }

    pub fn draw(&self, _context: &mut GMDrawContext) {
        if self.draw_object_common.active {
            let _index = self.animations[self.current_animation].frame_index();
        }

        // self.texture.draw_ex(x, y, index, angle, self.flip_x, self.flip_y, context);
    }
}

#[derive(Debug, Clone)]
pub struct GMSprite {
    pub sprite_common: GMSpriteCommon,
}

impl GMSprite {
    pub fn new(texture: Rc<GMTexture>, name: &str, x: f32, y: f32, animation: Box<dyn GMAnimationT>) -> Self {
        let sprite_common = GMSpriteCommon::new(texture, name, x, y, animation);

        Self {
            sprite_common,
        }
    }
}

impl GMDrawObjectT for GMSprite {
    fn update(&mut self, _context: &mut GMUpdateContext) -> Result<(), GMError> {
        self.sprite_common.update();

        while let Some(message) = self.sprite_common.draw_object_common.get_next_message() {
            let value = message.value;

            if let Some(message) = value.downcast_ref::<GMSpriteMessage>() {
                use GMSpriteMessage::*;

                match message {
                    SetTexture(texture) => {

                    }
                    SetCurrentAnimation(index) => {

                    }
                    FlipX(value) => {

                    }
                    FlipY(value) => {

                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&self, context: &mut GMDrawContext) -> Result<(), GMError> {
        self.sprite_common.draw(context);

        Ok(())
    }

    fn get_common_ref(&self) -> &GMDrawObjectCommon {
        &self.sprite_common.draw_object_common
    }

    fn get_common_mut_ref(&mut self) -> &mut GMDrawObjectCommon {
        &mut self.sprite_common.draw_object_common
    }

    fn box_clone(&self) -> Box<dyn GMDrawObjectT> {
        let result = self.clone();

        Box::new(result)
    }
}

pub enum GMSpriteMessage {
    SetTexture(Rc<GMTexture>),
    SetCurrentAnimation(usize),
    FlipX(bool),
    FlipY(bool),
}

*/
