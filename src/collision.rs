
pub enum GM_CollisionShape {
    GM_Empty,
    GM_Circle{ radius: u32 },
    GM_Box{ width: u32, height: u32 },
    GM_Capsule,
    GM_Polygon,
}
