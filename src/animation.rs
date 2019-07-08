
pub trait GM_Animation_T {
    fn uptate(&mut self) {

    }
}

struct GM_Animation {
    current_index: usize,
    frames: Vec<usize>,
}

impl GM_Animation {
    fn new(frames: Vec<usize>) -> GM_Animation {
        GM_Animation {
            current_index: 0,
            frames,
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

pub struct GM_Animation_Once {
    inner: GM_Animation,
}

impl GM_Animation_Once {
    pub fn new(frames: Vec<usize>) -> GM_Animation_Once {
        GM_Animation_Once {
            inner: GM_Animation::new(frames),
        }
    }
}

impl GM_Animation_T for GM_Animation_Once {
    fn update(&mut self) {
        if !self.inner.at_end() {
            self.inner.inc();
        }
    }
}

pub struct GM_Animation_Cycle {
    inner: GM_Animation,
}

impl GM_Animation_Cycle {
    pub fn new(frames: Vec<usize>) -> GM_Animation_Cycle {
        GM_Animation_Cycle {
            inner: GM_Animation::new(frames),
        }
    }
}

impl GM_Animation_T for GM_Animation_Cycle {
    fn update(&mut self) {
        if !self.inner.at_end() {
            self.inner.inc();
        } else {
            // Restart animation again if finished
            self.inner.current_index = 0;
        }
    }
}

pub struct GM_Animation_PingPong {
    inner: GM_Animation,
    forward: bool,
}

impl GM_Animation_PingPong {
    pub fn new(frames: Vec<usize>) -> GM_Animation_PingPong {
        GM_Animation_PingPong {
            inner: GM_Animation::new(frames),
            forward: true,
        }
    }
}

impl GM_Animation_T for GM_Animation_PingPong {
    fn update(&mut self) {
        if self.forward {
            if !self.inner.at_end() {
                self.inner.inc();
            } else {
                // Play animation backwards
                self.forward = false;
            }
        } else {
            if !self.inner.at_start() {
                self.inner.dec();
            } else {
                // Play animation forwards
                self.forward = true;
            }
        }
    }
}
