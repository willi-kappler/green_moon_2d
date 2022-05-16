
use std::rc::Rc;
use std::convert::From;

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
    pub fn new<S: Into<GMSender>, R: Into<GMReceiver>>(sender: S, receiver: R, message_data: GMMessageData) -> Self {
        Self {
            sender: sender.into(),
            receiver: receiver.into(),
            message_data,
        }
    }

    pub fn new_reply<S: Into<GMSender>>(sender: S, message: &GMMessage, message_data: GMMessageData) -> Self {
        Self::new(sender, &message.sender, message_data)
    }

    pub fn from_factory(message_factory: GMMessageFactory) -> Self {
        if message_factory.sender.is_none() {
            panic!("GMMessage::from_factory(), sender is missing: {:?}", message_factory);
        } else if message_factory.receiver.is_none() {
            panic!("GMMessage::from_factory(), receiver is missing: {:?}", message_factory);
        } else if message_factory.message_data.is_none() {
            panic!("GMMessage::from_factory(), message_data is missing: {:?}", message_factory);
        }

        Self {
            sender: message_factory.sender.unwrap(),
            receiver: message_factory.receiver.unwrap(),
            message_data: message_factory.message_data.unwrap(),
        }
    }

    pub fn as_reply(&self) -> GMReceiver {
        self.sender.as_receiver()
    }
}

#[derive(Clone, Debug)]
pub struct GMMessageFactory {
    pub sender: Option<GMSender>,
    pub receiver: Option<GMReceiver>,
    pub message_data: Option<GMMessageData>,
}

impl GMMessageFactory {
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None,
            message_data: None,
        }
    }

    pub fn new_with<T: Into<GMMessageFactory>>(params: T) -> Self {
        params.into()
    }

    pub fn set<T: Into<GMMessageFactory>>(&mut self, params: T) {
        let message_factory = params.into();

        if message_factory.sender.is_some() {
            self.sender = message_factory.sender;
        }

        if message_factory.receiver.is_some() {
            self.receiver = message_factory.receiver;
        }

        if message_factory.message_data.is_some() {
            self.message_data = message_factory.message_data;
        }

    }

    pub fn create(&self) -> GMMessage {
        GMMessage::from_factory(self.clone())
    }

    pub fn create_with<T: Into<GMMessageFactory>>(&self, params: T) -> GMMessage {
        let mut message_factory = params.into();

        if message_factory.sender.is_none() && self.sender.is_some() {
            message_factory.sender = self.sender.clone();
        }

        if message_factory.receiver.is_none() && self.receiver.is_some() {
            message_factory.receiver = self.receiver.clone();
        }

        if message_factory.message_data.is_none() && self.message_data.is_some() {
            message_factory.message_data = self.message_data.clone();
        }

        GMMessage::from_factory(message_factory)
    }

    pub fn create_to_engine(&self) -> GMMessage {
        self.create_with(GMReceiver::Engine)
    }

    pub fn create_data_to_engine(&self, message_data: GMMessageData) -> GMMessage {
        self.create_with((GMReceiver::Engine, message_data))
    }

    pub fn create_to_scene_manager(&self) -> GMMessage {
        self.create_with(GMReceiver::SceneManager)
    }

    pub fn create_data_to_scene_manager(&self, message_data: GMMessageData) -> GMMessage {
        self.create_with((GMReceiver::SceneManager, message_data))
    }

    pub fn create_to_scene(&self, scene: &str) -> GMMessage {
        self.create_with(GMReceiver::Scene(scene.to_string()))
    }

    pub fn create_data_to_scene(&self, scene: &str, message_data: GMMessageData) -> GMMessage {
        self.create_with((GMReceiver::Scene(scene.to_string()), message_data))
    }

    pub fn create_to_scene_with_property(&self, property: &str) -> GMMessage {
        self.create_with(GMReceiver::SceneWithProperty(property.to_string()))
    }

    pub fn create_data_to_scene_with_property(&self, property: &str, message_data: GMMessageData) -> GMMessage {
        self.create_with((GMReceiver::SceneWithProperty(property.to_string()), message_data))
    }

    pub fn create_to_current_scene(&self) -> GMMessage {
        self.create_with(GMReceiver::CurrentScene)
    }

    pub fn create_data_to_current_scene(&self, message_data: GMMessageData) -> GMMessage {
        self.create_with((GMReceiver::CurrentScene, message_data))
    }

    pub fn create_to_object_manager(&self) -> GMMessage {
        self.create_with(GMReceiver::ObjectManager)
    }

    pub fn create_data_to_object_manager(&self, message_data: GMMessageData) -> GMMessage {
        self.create_with((GMReceiver::ObjectManager, message_data))
    }

    pub fn create_to_object(&self, object: &str) -> GMMessage {
        self.create_with(GMReceiver::Object(object.to_string()))
    }

    pub fn create_data_to_object(&self, object: &str, message_data: GMMessageData) -> GMMessage {
        self.create_with((GMReceiver::Object(object.to_string()), message_data))
    }

    pub fn create_to_object_with_property(&self, property: &str) -> GMMessage {
        self.create_with(GMReceiver::ObjectWithProperty(property.to_string()))
    }

    pub fn create_data_to_object_with_property(&self, property: &str, message_data: GMMessageData) -> GMMessage {
        self.create_with((GMReceiver::ObjectWithProperty(property.to_string()), message_data))
    }







    // Engine messages:
    pub fn engine_quit(&self) -> GMMessage {
        debug!("GMMessageFactory::engine_quit()");
        self.create_data_to_engine(GMMessageData::Quit)
    }

    pub fn engine_change_fps(&self, new_fps: u32) -> GMMessage {
        debug!("GMMessageFactory::engine_change_fps(), new_fps: '{}'", new_fps);
        self.create_data_to_engine(GMMessageData::ChangeFPS(new_fps))
    }


    // Scene manager messages:
    pub fn scene_add<S: 'static + GMSceneT>(&self, scene: S) -> GMMessage {
        debug!("GMMessageFactory::scene_add(), name: '{}'", scene.get_name());
        self.scene_add_box(Box::new(scene))
    }

    pub fn scene_add_box(&self, scene: Box<dyn GMSceneT>) -> GMMessage {
        debug!("GMMessageFactory::scene_add_box(), name: '{}'", scene.get_name());
        self.create_data_to_scene_manager(GMMessageData::AddScene(scene))
    }

    pub fn scene_remove(&self, name: &str) -> GMMessage {
        debug!("GMMessageFactory::scene_remove(), name: '{}'", name);
        self.create_data_to_scene_manager(GMMessageData::RemoveScene(name.to_string()))
    }

    pub fn scene_change(&self, name: &str) -> GMMessage {
        debug!("GMMessageFactory::scene_change(), name: '{}'", name);
        self.create_data_to_scene_manager(GMMessageData::ChangeToScene(name.to_string()))
    }

    pub fn scene_replace<S: 'static + GMSceneT>(&self, scene: S) -> GMMessage {
        debug!("GMMessageFactory::scene_replace(), name: '{}'", scene.get_name());
        self.scene_replace_box(Box::new(scene))
    }

    pub fn scene_replace_box(&self, scene: Box<dyn GMSceneT>) -> GMMessage {
        debug!("GMMessageFactory::scene_replace_box(), name: '{}'", scene.get_name());
        self.create_data_to_scene_manager(GMMessageData::ReplaceScene(scene))
    }

    pub fn scene_push(&self) -> GMMessage {
        debug!("GMMessageFactory::scene_push()");
        self.create_data_to_scene_manager(GMMessageData::PushCurrentScene)
    }

    pub fn scene_pop(&self) -> GMMessage {
        debug!("GMMessageFactory::scene_pop()");
        self.create_data_to_scene_manager(GMMessageData::PopCurrentScene)
    }


    // Object messages:
    pub fn object_set_z_index(&self, name: &str, z_index: i32) -> GMMessage {
        self.create_data_to_object(name, GMMessageData::SetZIndex(z_index))
    }

    pub fn object_set_z_index_with_property(&self, property: &str, z_index: i32) -> GMMessage {
        self.create_data_to_object_with_property(property, GMMessageData::SetZIndex(z_index))
    }

    pub fn object_set_active(&self, name: &str, active: bool) -> GMMessage {
        self.create_data_to_object(name, GMMessageData::SetActive(active))
    }

    pub fn object_set_active_with_property(&self, property: &str, active: bool) -> GMMessage {
        self.create_data_to_object_with_property(property, GMMessageData::SetActive(active))
    }

    pub fn object_set_position(&self, name: &str, position: GMVec2D) -> GMMessage {
       self.create_data_to_object(name, GMMessageData::SetPosition(position))
    }

    pub fn object_set_position_with_property(&self, property: &str, position: GMVec2D) -> GMMessage {
        self.create_data_to_object_with_property(property, GMMessageData::SetPosition(position))
    }

    pub fn object_add_position(&self, name: &str, position: GMVec2D) -> GMMessage {
        self.create_data_to_object(name, GMMessageData::AddPosition(position))
    }

    pub fn object_add_position_with_property(&self, property: &str, position: GMVec2D) -> GMMessage {
        self.create_data_to_object_with_property(property, GMMessageData::AddPosition(position))
    }

    pub fn object_set_velocity(&self, name: &str, velocity: GMVec2D) -> GMMessage {
        self.create_data_to_object(name, GMMessageData::SetVelocity(velocity))
    }

    pub fn object_set_velocity_with_property(&self, property: &str, velocity: GMVec2D) -> GMMessage {
        self.create_data_to_object_with_property(property, GMMessageData::SetVelocity(velocity))
    }

    pub fn object_add_velocity(&self, name: &str, velocity: GMVec2D) -> GMMessage {
        self.create_data_to_object(name, GMMessageData::AddVelocity(velocity))
    }

    pub fn object_add_velocity_with_property(&self, property: &str, velocity: GMVec2D) -> GMMessage {
        self.create_data_to_object_with_property(property, GMMessageData::AddVelocity(velocity))
    }

    pub fn object_set_acceleration(&self, name: &str, acceleration: GMVec2D) -> GMMessage {
        self.create_data_to_object(name, GMMessageData::SetAcceleration(acceleration))
    }

    pub fn object_set_acceleration_with_property(&self, property: &str, acceleration: GMVec2D) -> GMMessage {
        self.create_data_to_object_with_property(property, GMMessageData::SetAcceleration(acceleration))
    }

    pub fn object_add_acceleration(&self, name: &str, acceleration: GMVec2D) -> GMMessage {
        self.create_data_to_object(name, GMMessageData::AddAcceleration(acceleration))
    }

    pub fn object_add_acceleration_with_property(&self, property: &str, acceleration: GMVec2D) -> GMMessage {
        self.create_data_to_object_with_property(property, GMMessageData::AddAcceleration(acceleration))
    }

    pub fn object_set_animation(&self, name: &str, animation: Box<dyn GMAnimationT>) -> GMMessage {
        self.create_data_to_object(name, GMMessageData::SetAnimation(animation))
    }

    pub fn object_set_animation_property(&self, property: &str, animation: Box<dyn GMAnimationT>) -> GMMessage {
        self.create_data_to_object_with_property(property, GMMessageData::SetAnimation(animation))
    }

}

impl From<GMSender> for GMMessageFactory {
    fn from(sender: GMSender) -> Self {
        Self {
            sender: Some(sender),
            receiver: None,
            message_data: None,
        }
    }
}

impl From<GMReceiver> for GMMessageFactory {
    fn from(receiver: GMReceiver) -> Self {
        Self {
            sender: None,
            receiver: Some(receiver),
            message_data: None,
        }
    }
}

impl From<GMMessageData> for GMMessageFactory {
    fn from(message_data: GMMessageData) -> Self {
        Self {
            sender: None,
            receiver: None,
            message_data: Some(message_data),
        }
    }
}

impl From<(GMSender, GMReceiver)> for GMMessageFactory {
    fn from((sender, receiver): (GMSender, GMReceiver)) -> Self {
        Self {
            sender: Some(sender),
            receiver: Some(receiver),
            message_data: None
        }
    }
}

impl From<(GMSender, GMMessageData)> for GMMessageFactory {
    fn from((sender, message_data): (GMSender, GMMessageData)) -> Self {
        Self {
            sender: Some(sender),
            receiver: None,
            message_data: Some(message_data)
        }
    }
}

impl From<(GMReceiver, GMMessageData)> for GMMessageFactory {
    fn from((receiver, message_data): (GMReceiver, GMMessageData)) -> Self {
        Self {
            sender: None,
            receiver: Some(receiver),
            message_data: Some(message_data)
        }
    }
}

impl From<(GMSender, GMReceiver, GMMessageData)> for GMMessageFactory {
    fn from((sender, receiver, message_data): (GMSender, GMReceiver, GMMessageData)) -> Self {
        Self {
            sender: Some(sender),
            receiver: Some(receiver),
            message_data: Some(message_data)
        }
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

impl From<&Box<dyn GMObjectT>> for GMSender {
    fn from(object: &Box<dyn GMObjectT>) -> Self {
        object.as_sender()
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

impl From<&Box<dyn GMObjectT>> for GMReceiver {
    fn from(object: &Box<dyn GMObjectT>) -> Self {
        object.as_receiver()
    }
}

impl From<&GMSender> for GMReceiver {
    fn from(sender: &GMSender) -> Self {
        sender.as_receiver()
    }
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
    MessageToChild(Box<GMMessageData>),

    SetZIndex(i32),
    GetZIndex,
    ZIndex(i32),

    SetPosition(GMVec2D),
    AddPosition(GMVec2D),
    GetPosition,
    Position(GMVec2D),
    GetNextPosition,
    NextPosition(GMVec2D),

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

    SetDuration(f32),
    GetDuration,
    Duration(f32),

    AddProperty(String, GMValue),
    AddTag(String),
    GetProperty(String),
    Property(String, GMValue),
    HasProperty(String),
    PropertyFound(String),
    PropertyNotFound(String),
    RemoveProperty(String),
}
