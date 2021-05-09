

pub enum GMObjectName {
    Single(String),
    Group(String, u32),
}

pub struct GMObject {
    name: GMObjectName,
}
