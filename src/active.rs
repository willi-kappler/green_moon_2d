
pub trait GM_Active_T {
    fn is_active(&self) -> bool;

    fn set_active(&mut self, active: bool) {
    }
}
