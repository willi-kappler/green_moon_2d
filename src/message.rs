
use std::rc::Rc;

use log::{debug};

use crate::animation::GMAnimationT;
// use crate::context::GMUpdateContext;
use crate::math::GMVec2D;
use crate::object::GMObjectT;
use crate::scene::GMSceneT;
use crate::texture::GMTexture;
use crate::property::GMValue;
use crate::font::GMFontT;
// use crate::error::GMError;


#[derive(Clone, Debug)]
pub struct GMMessage {
    pub sender: GMSender,
    pub receiver: GMReceiver,
    pub message_data: GMMessageData, // TODO: Vec<GMMessageData>
}

impl GMMessage {
    pub fn new(sender: GMSender, receiver: GMReceiver, message_data: GMMessageData) -> Self {
        Self {
            sender,
            receiver,
            message_data,
        }
    }

    pub fn new_object_reply(object: &Box<dyn GMObjectT>, message: &GMMessage, message_data: GMMessageData) -> Self {
        let sender = object.as_sender();
        let receiver = message.sender.as_receiver();

        Self {
            sender,
            receiver,
            message_data,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMMessageFactory {
    sender: GMSender,
    receiver: Option<GMReceiver>,
}

impl GMMessageFactory {
    pub fn new(sender: GMSender) -> Self {
        Self {
            sender,
            receiver: None,
        }
    }

    pub fn set_receiver(&mut self, receiver: GMReceiver) {
        self.receiver = Some(receiver);
    }

    pub fn sender_from_object(&mut self, sender: &Box<dyn GMObjectT>) {
        let name = sender.get_name();
        self.sender = GMSender::Object(name.to_string());
    }

    pub fn create(&self, message_data: GMMessageData) -> GMMessage {
        GMMessage::new(
            self.sender.clone(),
            self.receiver.clone().unwrap(),
            message_data,
        )
    }

    pub fn create_to(&self, receiver: GMReceiver, message_data: GMMessageData) -> GMMessage {
        GMMessage::new(
            self.sender.clone(),
            receiver,
            message_data,
        )
    }

    pub fn create_reply(&self, message: &GMMessage, message_data: GMMessageData) -> GMMessage {
        let receiver = message.sender.as_receiver();
        self.create_to(receiver, message_data)
    }

    pub fn create_to_engine(&self, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::Engine, message_data)
    }

    pub fn create_to_scene_manager(&self, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::SceneManager, message_data)
    }

    pub fn create_to_scene(&self, scene: &str, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::Scene(scene.to_string()), message_data)
    }

    pub fn create_to_scene_with_property(&self, property: &str, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::SceneWithProperty(property.to_string()), message_data)
    }

    pub fn create_to_current_scene(&self, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::CurrentScene, message_data)
    }

    pub fn create_to_object_manager(&self, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::ObjectManager, message_data)
    }

    pub fn create_to_object(&self, object: &str, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::Object(object.to_string()), message_data)
    }

    pub fn create_to_object_with_property(&self, property: &str, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::ObjectWithProperty(property.to_string()), message_data)
    }

    // Engine messages:

    pub fn engine_quit(&self) -> GMMessage {
        debug!("GMMessageFactory::engine_quit()");
        self.create_to_engine(GMMessageData::Quit)
    }

    pub fn engine_change_fps(&self, new_fps: u32) -> GMMessage {
        debug!("GMMessageFactory::engine_change_fps(), new_fps: '{}'", new_fps);
        self.create_to_engine(GMMessageData::ChangeFPS(new_fps))
    }


    // Scene manager messages:

    pub fn scene_add<S: 'static + GMSceneT>(&self, scene: S) -> GMMessage {
        debug!("GMMessageFactory::scene_add(), name: '{}'", scene.get_name());
        self.scene_add_box(Box::new(scene))
    }

    pub fn scene_add_box(&self, scene: Box<dyn GMSceneT>) -> GMMessage {
        debug!("GMMessageFactory::scene_add_box(), name: '{}'", scene.get_name());
        self.create_to_scene_manager(GMMessageData::AddScene(scene))
    }

    pub fn scene_remove(&self, name: &str) -> GMMessage {
        debug!("GMMessageFactory::scene_remove(), name: '{}'", name);
        self.create_to_scene_manager(GMMessageData::RemoveScene(name.to_string()))
    }

    pub fn scene_change(&self, name: &str) -> GMMessage {
        debug!("GMMessageFactory::scene_change(), name: '{}'", name);
        self.create_to_scene_manager(GMMessageData::ChangeToScene(name.to_string()))
    }

    pub fn scene_replace<S: 'static + GMSceneT>(&self, scene: S) -> GMMessage {
        debug!("GMMessageFactory::scene_replace(), name: '{}'", scene.get_name());
        self.scene_replace_box(Box::new(scene))
    }

    pub fn scene_replace_box(&self, scene: Box<dyn GMSceneT>) -> GMMessage {
        debug!("GMMessageFactory::scene_replace_box(), name: '{}'", scene.get_name());
        self.create_to_scene_manager(GMMessageData::ReplaceScene(scene))
    }

    pub fn scene_push(&self) -> GMMessage {
        debug!("GMMessageFactory::scene_push()");
        self.create_to_scene_manager(GMMessageData::PushCurrentScene)
    }

    pub fn scene_pop(&self) -> GMMessage {
        debug!("GMMessageFactory::scene_pop()");
        self.create_to_scene_manager(GMMessageData::PopCurrentScene)
    }


    // Object messages:

    pub fn object_set_z_index(&self, name: &str, z_index: i32) -> GMMessage {
        self.create_to_object(name, GMMessageData::SetZIndex(z_index))
    }

    pub fn object_set_z_index_with_property(&self, property: &str, z_index: i32) -> GMMessage {
        self.create_to_object_with_property(property, GMMessageData::SetZIndex(z_index))
    }

    pub fn object_set_active(&self, name: &str, active: bool) -> GMMessage {
        self.create_to_object(name, GMMessageData::SetActive(active))
    }

    pub fn object_set_active_with_property(&self, property: &str, active: bool) -> GMMessage {
        self.create_to_object_with_property(property, GMMessageData::SetActive(active))
    }

    pub fn object_set_position(&self, name: &str, position: GMVec2D) -> GMMessage {
       self.create_to_object(name, GMMessageData::SetPosition(position))
    }

    pub fn object_set_position_with_property(&self, property: &str, position: GMVec2D) -> GMMessage {
        self.create_to_object_with_property(property, GMMessageData::SetPosition(position))
    }

    pub fn object_add_position(&self, name: &str, position: GMVec2D) -> GMMessage {
        self.create_to_object(name, GMMessageData::AddPosition(position))
    }

    pub fn object_add_position_with_property(&self, property: &str, position: GMVec2D) -> GMMessage {
        self.create_to_object_with_property(property, GMMessageData::AddPosition(position))
    }

    pub fn object_set_velocity(&self, name: &str, velocity: GMVec2D) -> GMMessage {
        self.create_to_object(name, GMMessageData::SetVelocity(velocity))
    }

    pub fn object_set_velocity_with_property(&self, property: &str, velocity: GMVec2D) -> GMMessage {
        self.create_to_object_with_property(property, GMMessageData::SetVelocity(velocity))
    }

    pub fn object_add_velocity(&self, name: &str, velocity: GMVec2D) -> GMMessage {
        self.create_to_object(name, GMMessageData::AddVelocity(velocity))
    }

    pub fn object_add_velocity_with_property(&self, property: &str, velocity: GMVec2D) -> GMMessage {
        self.create_to_object_with_property(property, GMMessageData::AddVelocity(velocity))
    }

    pub fn object_set_acceleration(&self, name: &str, acceleration: GMVec2D) -> GMMessage {
        self.create_to_object(name, GMMessageData::SetAcceleration(acceleration))
    }

    pub fn object_set_acceleration_with_property(&self, property: &str, acceleration: GMVec2D) -> GMMessage {
        self.create_to_object_with_property(property, GMMessageData::SetAcceleration(acceleration))
    }

    pub fn object_add_acceleration(&self, name: &str, acceleration: GMVec2D) -> GMMessage {
        self.create_to_object(name, GMMessageData::AddAcceleration(acceleration))
    }

    pub fn object_add_acceleration_with_property(&self, property: &str, acceleration: GMVec2D) -> GMMessage {
        self.create_to_object_with_property(property, GMMessageData::AddAcceleration(acceleration))
    }

    pub fn object_set_animation(&self, name: &str, animation: Box<dyn GMAnimationT>) -> GMMessage {
        self.create_to_object(name, GMMessageData::SetAnimation(animation))
    }

    pub fn object_set_animation_property(&self, property: &str, animation: Box<dyn GMAnimationT>) -> GMMessage {
        self.create_to_object_with_property(property, GMMessageData::SetAnimation(animation))
    }

}

#[derive(Clone, Debug)]
pub enum GMSender {
    Engine,

    CurrentScene,
    Scene(String),
    SceneManager,

    Object(String),
    ObjectManager,
}

impl GMSender {
    pub fn as_receiver(&self) -> GMReceiver {
        match self {
            GMSender::Engine => {
                GMReceiver::Engine
            }
            GMSender::CurrentScene => {
                GMReceiver::CurrentScene
            }
            GMSender::Scene(name) => {
                GMReceiver::Scene(name.to_string())
            }
            GMSender::SceneManager => {
                GMReceiver::SceneManager
            }
            GMSender::Object(name) => {
                GMReceiver::Object(name.to_string())
            }
            GMSender::ObjectManager => {
                GMReceiver::ObjectManager
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum GMReceiver {
    Engine,

    CurrentScene,
    Scene(String),
    SceneWithProperty(String),
    SceneManager,

    Object(String),
    ObjectWithProperty(String),
    ObjectManager,
}

#[derive(Clone, Debug)]
pub enum GMMessageData {
    // Engine messages
    Quit,
    ChangeFPS(u32),
    ChangeResolution(u32, u32),
    ChangeTitle(String),

    // Scene messages
    InitScene,
    ExitScene,
    AddScene(Box<dyn GMSceneT>),
    RemoveScene(String),
    // TODO: Maybe add TakeScene message
    ChangeToScene(String),
    ReplaceScene(Box<dyn GMSceneT>),
    PushCurrentScene,
    PopCurrentScene,
    SetSceneParent(String, Box<dyn GMSceneT>),
    RemoveSceneParent(String),
    SetSceneChild(String, Box<dyn GMSceneT>),
    RemoveSceneChild(String),
    TakeSceneChild(String),

    // Object manager
    AddObject(Box<dyn GMObjectT>),
    ReplaceObject(Box<dyn GMObjectT>),
    RemoveObject(String),
    TakeObject(String),
    Object(Box<dyn GMObjectT>),
    SetObjectParent(String, Box<dyn GMObjectT>),
    RemoveObjectParent(String),
    SetObjectChild(String, Box<dyn GMObjectT>),
    RemoveObjectChild(String),
    TakeObjectChild(String),

    // Other messages
    SetActive(bool),
    GetActive,
    Active(bool),

    SetChild(Box<dyn GMObjectT>),
    GetChildClone,
    Child(Option<Box<dyn GMObjectT>>),

    SetZIndex(i32),
    GetZIndex,
    ZIndex(i32),

    SetPosition(GMVec2D),
    AddPosition(GMVec2D),
    GetPosition,
    Position(GMVec2D),

    SetVelocity(GMVec2D),
    AddVelocity(GMVec2D),
    GetVelocity,
    Velocity(GMVec2D),

    SetAcceleration(GMVec2D),
    AddAcceleration(GMVec2D),
    GetAcceleration,
    Acceleration(GMVec2D),

    SetRadius(f32),
    GetRadius,
    Radius(f32),

    SetAngle(f32),
    GetAngle,
    Angle(f32),

    SetAnimation(Box<dyn GMAnimationT>),
    SetAnimationName(String),
    SetTexture(Rc<GMTexture>),
    SetTextureName(String),
    AnimationDone,

    SetText(String),
    SetFont(Rc<dyn GMFontT>),
    SetFontName(String),

    Property(String, GMValue),
}
