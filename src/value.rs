
#[derive(Debug, Clone, PartialEq)]
pub enum GMValue {
    GMNone,
    GMBool(bool),
    GMF32(f32),
    GMUSize(usize),
    GMString(String),
}
