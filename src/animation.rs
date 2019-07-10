
// Local modules
use crate::active::{GM_Active_T};
use crate::update::{GM_Update_Elapsed_T};


pub trait GM_Animation_T: GM_Update_Elapsed_T + GM_Active_T {
    fn current_frame(&self) -> usize;

    fn start(&mut self);
}

pub struct GM_Animation {
    current_index: usize,
    frames: Vec<(u16, usize)>,
    active: bool,
    frame_duration: u16,
}

impl GM_Animation {
    fn new(frames: Vec<(u16, usize)>) -> GM_Animation {
        assert!(frames.len() > 0, "GM_Animation::new(), frames must not be empty!");

        GM_Animation {
            current_index: 0,
            frames,
            active: false,
            frame_duration: 0,
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

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }

    fn current_frame(&self) -> usize {
        self.frames[self.current_index].1
    }

    fn start(&mut self) {
        self.current_index = 0;
        self.active = true;
        self.frame_duration = self.frames[0].0;
    }

}

/*
impl GM_Animation_T for GM_Animation {}

impl GM_Update_Elapsed_T for GM_Animation {}

impl GM_Active_T for GM_Animation {
}
*/

pub struct GM_Animation_Once {
    base: GM_Animation,
}

impl GM_Animation_Once {
    pub fn new(frames: Vec<(u16, usize)>) -> GM_Animation_Once {
        GM_Animation_Once {
            base: GM_Animation::new(frames),
        }
    }
}

impl GM_Animation_T for GM_Animation_Once {
    fn current_frame(&self) -> usize {
        self.base.current_frame()
    }

    fn start(&mut self) {
        self.base.start();
    }
}

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
    pub fn new(frames: Vec<(u16, usize)>) -> GM_Animation_Cycle {
        GM_Animation_Cycle {
            base: GM_Animation::new(frames),
        }
    }
}

impl GM_Animation_T for GM_Animation_Cycle {
    fn current_frame(&self) -> usize {
        self.base.current_frame()
    }

    fn start(&mut self) {
        self.base.start();
    }
}

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
    pub fn new(frames: Vec<(u16, usize)>) -> GM_Animation_PingPong {
        GM_Animation_PingPong {
            base: GM_Animation::new(frames),
            forward: true,
        }
    }
}

impl GM_Animation_T for GM_Animation_PingPong {
    fn current_frame(&self) -> usize {
        self.base.current_frame()
    }

    fn start(&mut self) {
        self.base.start();
    }
}

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
