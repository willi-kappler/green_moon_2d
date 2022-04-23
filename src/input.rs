
use std::fmt::Debug;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::EventPump;


pub struct GMInput {
    event_pump: EventPump,
    key_esc_down_: bool,
    key_esc_up_: bool,
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
        }
    }

    pub(crate) fn update(&mut self) {
        self.key_esc_down_ = false;
        self.key_esc_up_ = false;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.key_esc_down_ = true;
                }
                Event::KeyUp { keycode: Some(Keycode::Escape), .. } => {
                    self.key_esc_up_ = true;
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
}
