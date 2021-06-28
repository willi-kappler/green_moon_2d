
#[derive(Debug, Clone, PartialEq)]
pub enum GMValue {
    None,
    Bool(bool),
    F32(f32),
    USize(usize),
    String(String),
}
