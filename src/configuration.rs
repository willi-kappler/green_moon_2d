

pub struct GMConfiguration {
    pub frame_time: f32,
}

impl GMConfiguration {
    pub fn new() -> Self {
        Self {
            frame_time: 1.0 / 60.0, // 60 FPS
        }
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.frame_time = 1.0 / fps;
    }
}
