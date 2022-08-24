
use std::fmt::Debug;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::EventPump;


pub struct GMInput {
    event_pump: EventPump,
    key_esc_down_: bool,
    key_esc_up_: bool,
    key_0_down_: bool,
    key_0_up_: bool,
    key_1_down_: bool,
    key_1_up_: bool,
    key_2_down_: bool,
    key_2_up_: bool,
    key_3_down_: bool,
    key_3_up_: bool,
}


impl Debug for GMInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GMInput")
    }
}

impl GMInput {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,
            key_esc_down_: false,
            key_esc_up_: false,
            key_0_down_: false,
            key_0_up_: false,
            key_1_down_: false,
            key_1_up_: false,
            key_2_down_: false,
            key_2_up_: false,
            key_3_down_: false,
            key_3_up_: false,
        }
    }

    pub(crate) fn update(&mut self) {
        self.key_esc_down_ = false;
        self.key_esc_up_ = false;
        self.key_0_down_ = false;
        self.key_0_up_ = false;
        self.key_1_down_ = false;
        self.key_1_up_ = false;
        self.key_2_down_ = false;
        self.key_2_up_ = false;
        self.key_3_down_ = false;
        self.key_3_up_ = false;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.key_esc_down_ = true;
                }
                Event::KeyUp { keycode: Some(Keycode::Escape), .. } => {
                    self.key_esc_up_ = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Num0), .. } => {
                    self.key_0_down_ = true;
                }
                Event::KeyUp { keycode: Some(Keycode::Num0), .. } => {
                    self.key_0_up_ = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    self.key_1_down_ = true;
                }
                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {
                    self.key_1_up_ = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                    self.key_2_down_ = true;
                }
                Event::KeyUp { keycode: Some(Keycode::Num2), .. } => {
                    self.key_2_up_ = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                    self.key_3_down_ = true;
                }
                Event::KeyUp { keycode: Some(Keycode::Num3), .. } => {
                    self.key_3_up_ = true;
                }
                _ => {

                }
            }
        }
    }

    pub fn key_esc_down(&self) -> bool {
        self.key_esc_down_
    }

    pub fn key_esc_up(&self) -> bool {
        self.key_esc_up_
    }

    pub fn key_0_down(&self) -> bool {
        self.key_0_down_
    }

    pub fn key_0_up(&self) -> bool {
        self.key_0_up_
    }

    pub fn key_1_down(&self) -> bool {
        self.key_1_down_
    }

    pub fn key_1_up(&self) -> bool {
        self.key_1_up_
    }

    pub fn key_2_down(&self) -> bool {
        self.key_2_down_
    }

    pub fn key_2_up(&self) -> bool {
        self.key_2_up_
    }

    pub fn key_3_down(&self) -> bool {
        self.key_3_down_
    }

    pub fn key_3_up(&self) -> bool {
        self.key_3_up_
    }
}
