

// Local modules
use crate::update::{GM_Update_Elapsed_T};
use crate::active::{GM_Active_T};
use crate::position::{GM_Position};


pub trait GM_Path_T: GM_Update_Elapsed_T + GM_Active_T {
}

pub struct GM_Path {
    positions: Vec<GM_Position>,
}

