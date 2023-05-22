
use crate::target::GMTarget;
use crate::message::GMMessage;
use crate::value::GMValue;
use crate::math::GMVec2D;
use crate::object::GMObjectT;
use crate::context::GMContext;
use crate::object_manager::GMObjectManager;


#[derive(Clone, Debug)]
pub struct GMTargetBase {
    pub value: GMTarget,
}

impl GMTargetBase {
    pub fn new<T: Into<GMTarget>>(target: T) -> Self {
        Self {
            value: target.into(),
        }
    }

    pub fn send_message(&mut self, message: GMMessage) -> GMValue {
        match message {
            GMMessage::GetTarget => {
                self.value.clone().into()
            }
            GMMessage::SetTarget(value) => {
                self.value = value;
                GMValue::None
            }
            _ => {
                GMValue::unknown(message)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct GMSourceBase {
    pub value: GMTarget,
}

impl GMSourceBase {
    pub fn new<T: Into<GMTarget>>(target: T) -> Self {
        Self {
            value: target.into(),
        }
    }

    pub fn send_message(&mut self, message: GMMessage) -> GMValue {
        match message {
            GMMessage::GetSource => {
                self.value.clone().into()
            }
            GMMessage::SetSource(value) => {
                self.value = value;
                GMValue::None
            }
            _ => {
                GMValue::unknown(message)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct GMValueBoolBase {
    pub value: bool,
    pub get: String,
    pub set: String,
    pub toggle: String,
}

impl GMValueBoolBase {
    pub fn new(value: bool, name: &str) -> Self {
        Self {
            value,
            get: format!("get_{}", name),
            set: format!("set_{}", name),
            toggle: format!("toggle_{}", name),
        }
    }

    pub fn send_message(&mut self, message: GMMessage) -> GMValue {
        match message {
            GMMessage::Custom0(name)  if name == self.toggle => {
                self.value = !self.value;
                GMValue::None
            }
            GMMessage::Custom0(name)  if name == self.get => {
                self.value.into()
            }
            GMMessage::Custom1(name, GMValue::Bool(value)) if name == self.set => {
                self.value = value;
                GMValue::None
            }
            _ => {
                GMValue::unknown(message)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct GMValueF32Base {
    pub value: f32,
    pub get: String,
    pub set: String,
    pub add: String,
    pub mul: String,
}

impl GMValueF32Base {
    pub fn new(value: f32, name: &str) -> Self {
        Self {
            value,
            get: format!("get_{}", name),
            set: format!("set_{}", name),
            add: format!("add_{}", name),
            mul: format!("mul_{}", name),
        }
    }

    pub fn send_message(&mut self, message: GMMessage) -> GMValue {
        match message {
            GMMessage::Custom0(name)  if name == self.get => {
                self.value.into()
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == self.set => {
                self.value = value;
                GMValue::None
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == self.add => {
                self.value += value;
                GMValue::None
            }
            GMMessage::Custom1(name, GMValue::F32(value)) if name == self.mul => {
                self.value *= value;
                GMValue::None
            }
            _ => {
                GMValue::unknown(message)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct GMValueUSizeBase {
    pub value: usize,
    pub get: String,
    pub set: String,
    pub add: String,
}

impl GMValueUSizeBase {
    pub fn new(value: usize, name: &str) -> Self {
        Self {
            value,
            get: format!("get_{}", name),
            set: format!("set_{}", name),
            add: format!("add_{}", name),
        }
    }

    pub fn send_message(&mut self, message: GMMessage) -> GMValue {
        match message {
            GMMessage::Custom0(name)  if name == self.get => {
                self.value.into()
            }
            GMMessage::Custom1(name, GMValue::USize(value)) if name == self.set => {
                self.value = value;
                GMValue::None
            }
            GMMessage::Custom1(name, GMValue::USize(value)) if name == self.add => {
                self.value += value;
                GMValue::None
            }
            _ => {
                GMValue::unknown(message)
            }
        }
    }
}



// This is just an example:
#[derive(Clone, Debug)]
pub struct Foo {
    position: GMVec2D,
    auto_update: GMValueBoolBase,
    speed: GMValueF32Base,
}

impl Foo {
    pub fn new() -> Self {
        Self {
            position: GMVec2D::new(0.0, 0.0),
            auto_update: GMValueBoolBase::new(false, "auto_update"),
            speed: GMValueF32Base::new(1.0, "speed"),
        }
    }
}

impl GMObjectT for Foo {
    fn send_message(&mut self, message: GMMessage, _context: &mut GMContext, _object_manager: &GMObjectManager) -> GMValue {
        match message {
            GMMessage::Update => {
                self.speed.value += 0.1;
                GMValue::None
            }
            _ => {
                self.position.send_message(message)
                    .handle(|m| self.auto_update.send_message(m))
                    .handle(|m| self.speed.send_message(m))
            }
        }
    }

    fn clone_box(&self) -> Box<dyn GMObjectT> {
        Box::new(self.clone())
    }
}
