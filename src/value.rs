
#[derive(Debug, Clone, PartialEq)]
pub enum GMValue {
    GMBool(bool),
    GMF32(f32),
    GMUSize(usize),
    GMString(String),
}
