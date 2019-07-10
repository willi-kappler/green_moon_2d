
// Local modules
use crate::update::{GM_Update_Elapsed_T};
use crate::active::{GM_Active_T};



pub trait GM_Animation_T: GM_Update_Elapsed_T + GM_Active_T {
}

pub struct GM_Animation {
    current_index: usize,
    frames: Vec<usize>,
    active: bool,
}

impl GM_Animation {
    fn new(frames: Vec<usize>) -> GM_Animation {
        GM_Animation {
            current_index: 0,
            frames,
            active: true,
        }
    }

    fn at_end(&self) -> bool {
        self.current_index + 1 == self.frames.len()
    }

    fn at_start(&self) -> bool {
        self.current_index == 0
    }

    fn inc(&mut self) {
        self.current_index += 1;
    }

    fn dec(&mut self) {
        self.current_index -= 1;
    }
}

impl GM_Animation_T for GM_Animation {}

impl GM_Update_Elapsed_T for GM_Animation {}

impl GM_Active_T for GM_Animation {
    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }
}

pub struct GM_Animation_Once {
    base: GM_Animation,
}

impl GM_Animation_Once {
    pub fn new(frames: Vec<usize>) -> GM_Animation_Once {
        GM_Animation_Once {
            base: GM_Animation::new(frames),
        }
    }
}

impl GM_Animation_T for GM_Animation_Once {}

impl GM_Update_Elapsed_T for GM_Animation_Once {
    fn update(&mut self, time_elapsed: u16) {
        if !self.base.at_end() {
            self.base.inc();
        }
    }
}

impl GM_Active_T for GM_Animation_Once {
    fn is_active(&self) -> bool {
        self.base.is_active()
    }

    fn set_active(&mut self, active: bool) {
        self.base.set_active(active)
    }
}

pub struct GM_Animation_Cycle {
    base: GM_Animation,
}

impl GM_Animation_Cycle {
    pub fn new(frames: Vec<usize>) -> GM_Animation_Cycle {
        GM_Animation_Cycle {
            base: GM_Animation::new(frames),
        }
    }
}

impl GM_Animation_T for GM_Animation_Cycle {}

impl GM_Update_Elapsed_T for GM_Animation_Cycle {
    fn update(&mut self, time_elapsed: u16) {
        if !self.base.at_end() {
            self.base.inc();
        } else {
            // Restart animation again if finished
            self.base.current_index = 0;
        }
    }
}

impl GM_Active_T for GM_Animation_Cycle {
    fn is_active(&self) -> bool {
        self.base.is_active()
    }

    fn set_active(&mut self, active: bool) {
        self.base.set_active(active)
    }
}

pub struct GM_Animation_PingPong {
    base: GM_Animation,
    forward: bool,
}

impl GM_Animation_PingPong {
    pub fn new(frames: Vec<usize>) -> GM_Animation_PingPong {
        GM_Animation_PingPong {
            base: GM_Animation::new(frames),
            forward: true,
        }
    }
}

impl GM_Animation_T for GM_Animation_PingPong {}

impl GM_Update_Elapsed_T for GM_Animation_PingPong {
    fn update(&mut self, time_elapsed: u16) {
        if self.forward {
            if !self.base.at_end() {
                self.base.inc();
            } else {
                // Play animation backwards
                self.forward = false;
            }
        } else {
            if !self.base.at_start() {
                self.base.dec();
            } else {
                // Play animation forwards
                self.forward = true;
            }
        }
    }
}

impl GM_Active_T for GM_Animation_PingPong {
    fn is_active(&self) -> bool {
        self.base.is_active()
    }

    fn set_active(&mut self, active: bool) {
        self.base.set_active(active)
    }
}
