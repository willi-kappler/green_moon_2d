
trait GM_Animation_T {
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

struct GM_Animation_Once {
    inner: GM_Animation,
}

impl GM_Animation_T for GM_Animation_Once {
    fn update(&mut self) {
        if !self.inner.at_end() {
            self.inner.inc();
        }
    }
}

struct GM_Animation_Cycle {
    inner: GM_Animation,
}

impl GM_Animation_Cycle {
    fn new(frames: Vec<usize>) -> GM_Animation_Cycle {
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
            // Restart animantion again if finished
            self.inner.current_index = 0;
        }
    }
}

struct GM_Animation_PingPong {
    inner: GM_Animation,
    foreward: bool,
}

impl GM_Animation_PingPong {
    fn new(frames: Vec<usize>) -> GM_Animation_PingPong {
        GM_Animation_PingPong {
            inner: GM_Animation::new(frames),
            foreward: true,
        }
    }
}

impl GM_Animation_T for GM_Animation_PingPong {
    fn update(&mut self) {
        if self.foreward {
            if !self.inner.at_end() {
                self.inner.inc();
            } else {
                // Play animation backwards
                self.foreward = false;
            }
        } else {
            if !self.inner.at_start() {
                self.inner.dec();
            } else {
                // Play animation forewards
                self.foreward = true;
            }
        }
    }
}
