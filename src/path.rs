

// Local modules
use crate::active::{GM_Active_T};
use crate::position::{GM_Position};
use crate::update::{GM_UpdateElapsed_T};


pub trait GM_Path_T: GM_UpdateElapsed_T + GM_Active_T {
}

pub struct GM_Path {
    positions: Vec<GM_Position>,
}

