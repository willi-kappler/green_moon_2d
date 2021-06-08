
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GMValue {
    GMNone,
    GMBool(bool),
    GMF32(f32),
    GMUSize(usize),
}
