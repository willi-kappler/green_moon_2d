
use std::fmt::Debug;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::event::WindowEvent;
use sdl2::EventPump;

#[derive(Copy, Clone, Debug)]
pub enum GMEventCode {
    Quit = 0,
    WindowClose,

    // Special keys
    KeyESCDown,
    KeyESCUp,

    // Numbers
    Key0Down,
    Key0Up,
    Key1Down,
    Key1Up,
    Key2Down,
    Key2Up,
    Key3Down,
    Key3Up,
    Key4Down,
    Key4Up,
    Key5Down,
    Key5Up,
    Key6Down,
    Key6Up,
    Key7Down,
    Key7Up,
    Key8Down,
    Key8Up,
    Key9Down,
    Key9Up,

    // Cursor keys
    KeyUpDown,
    KeyUpUp,
    KeyDownDown,
    KeyDownUp,
    KeyLeftDown,
    KeyLeftUp,
    KeyRightDown,
    KeyRightUp,

    // Letter keys
    KeyAUp,
    KeyADown,
    KeyBUp,
    KeyBDown,
    KeyCUp,
    KeyCDown,
    KeyDUp,
    KeyDDown,
    KeyEUp,
    KeyEDown,
    KeyFUp,
    KeyFDown,
    KeyGUp,
    KeyGDown,
    KeyHUp,
    KeyHDown,
    KeyIUp,
    KeyIDown,
    KeyJUp,
    KeyJDown,
    KeyKUp,
    KeyKDown,
    KeyLUp,
    KeyLDown,
    KeyMUp,
    KeyMDown,
    KeyNUp,
    KeyNDown,
    KeyOUp,
    KeyODown,
    KeyPUp,
    KeyPDown,
    KeyQUp,
    KeyQDown,
    KeyRUp,
    KeyRDown,
    KeySUp,
    KeySDown,
    KeyTUp,
    KeyTDown,
    KeyUUp,
    KeyUDown,
    KeyVUp,
    KeyVDown,
    KeyWUp,
    KeyWDown,
    KeyXUp,
    KeyXDown,
    KeyYUp,
    KeyYDown,
    KeyZUp,
    KeyZDown,
}

pub struct GMInput {
    event_pump: EventPump,
    events: Vec<bool>,
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
            events: vec![false; 100],
        }
    }

    pub(crate) fn event(&self, event_code: GMEventCode) -> bool {
        self.events[event_code as usize]
    }

    pub(crate) fn update(&mut self) {
        for event in self.events.iter_mut() {
            *event = false;
        }

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit{..} => {
                    self.events[GMEventCode::Quit as usize] = true;
                }
                Event::Window{win_event: WindowEvent::Close, ..} => {
                    self.events[GMEventCode::WindowClose as usize] = true;
                }
                Event::KeyDown{keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape => {
                            self.events[GMEventCode::KeyESCDown as usize] = true;
                        }
                        Keycode::Num0 => {
                            self.events[GMEventCode::Key0Down as usize] = true;
                        }
                        Keycode::Num1 => {
                            self.events[GMEventCode::Key1Down as usize] = true;
                        }
                        Keycode::Num2 => {
                            self.events[GMEventCode::Key2Down as usize] = true;
                        }
                        Keycode::Num3 => {
                            self.events[GMEventCode::Key3Down as usize] = true;
                        }
                        Keycode::Num4 => {
                            self.events[GMEventCode::Key4Down as usize] = true;
                        }
                        Keycode::Num5 => {
                            self.events[GMEventCode::Key5Down as usize] = true;
                        }
                        Keycode::Num6 => {
                            self.events[GMEventCode::Key6Down as usize] = true;
                        }
                        Keycode::Num7 => {
                            self.events[GMEventCode::Key7Down as usize] = true;
                        }
                        Keycode::Num8 => {
                            self.events[GMEventCode::Key8Down as usize] = true;
                        }
                        Keycode::Num9 => {
                            self.events[GMEventCode::Key9Down as usize] = true;
                        }
                        Keycode::Up => {
                            self.events[GMEventCode::KeyUpDown as usize] = true;
                        }
                        Keycode::Down => {
                            self.events[GMEventCode::KeyDownDown as usize] = true;
                        }
                        Keycode::Left => {
                            self.events[GMEventCode::KeyLeftDown as usize] = true;
                        }
                        Keycode::Right => {
                            self.events[GMEventCode::KeyRightDown as usize] = true;
                        }
                        Keycode::A => {
                            self.events[GMEventCode::KeyADown as usize] = true;
                        }
                        Keycode::B => {
                            self.events[GMEventCode::KeyBDown as usize] = true;
                        }
                        Keycode::C => {
                            self.events[GMEventCode::KeyCDown as usize] = true;
                        }
                        Keycode::D => {
                            self.events[GMEventCode::KeyDDown as usize] = true;
                        }
                        Keycode::E => {
                            self.events[GMEventCode::KeyEDown as usize] = true;
                        }
                        Keycode::F => {
                            self.events[GMEventCode::KeyFDown as usize] = true;
                        }
                        Keycode::G => {
                            self.events[GMEventCode::KeyGDown as usize] = true;
                        }
                        Keycode::H => {
                            self.events[GMEventCode::KeyHDown as usize] = true;
                        }
                        _ => {
                            // Unsupported keycode for now
                        }
                    }
                }
                Event::KeyUp{keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape => {
                            self.events[GMEventCode::KeyESCUp as usize] = true;
                        }
                        Keycode::Num0 => {
                            self.events[GMEventCode::Key0Up as usize] = true;
                        }
                        Keycode::Num1 => {
                            self.events[GMEventCode::Key1Up as usize] = true;
                        }
                        Keycode::Num2 => {
                            self.events[GMEventCode::Key2Up as usize] = true;
                        }
                        Keycode::Num3 => {
                            self.events[GMEventCode::Key3Up as usize] = true;
                        }
                        Keycode::Num4 => {
                            self.events[GMEventCode::Key4Up as usize] = true;
                        }
                        Keycode::Num5 => {
                            self.events[GMEventCode::Key5Up as usize] = true;
                        }
                        Keycode::Num6 => {
                            self.events[GMEventCode::Key6Up as usize] = true;
                        }
                        Keycode::Num7 => {
                            self.events[GMEventCode::Key7Up as usize] = true;
                        }
                        Keycode::Num8 => {
                            self.events[GMEventCode::Key8Up as usize] = true;
                        }
                        Keycode::Num9 => {
                            self.events[GMEventCode::Key9Up as usize] = true;
                        }
                        Keycode::Up => {
                            self.events[GMEventCode::KeyUpUp as usize] = true;
                        }
                        Keycode::Down => {
                            self.events[GMEventCode::KeyDownUp as usize] = true;
                        }
                        Keycode::Left => {
                            self.events[GMEventCode::KeyLeftUp as usize] = true;
                        }
                        Keycode::Right => {
                            self.events[GMEventCode::KeyRightUp as usize] = true;
                        }
                        Keycode::A => {
                            self.events[GMEventCode::KeyAUp as usize] = true;
                        }
                        Keycode::B => {
                            self.events[GMEventCode::KeyBUp as usize] = true;
                        }
                        Keycode::C => {
                            self.events[GMEventCode::KeyCUp as usize] = true;
                        }
                        Keycode::D => {
                            self.events[GMEventCode::KeyDUp as usize] = true;
                        }
                        Keycode::E => {
                            self.events[GMEventCode::KeyEUp as usize] = true;
                        }
                        Keycode::F => {
                            self.events[GMEventCode::KeyFUp as usize] = true;
                        }
                        Keycode::G => {
                            self.events[GMEventCode::KeyGUp as usize] = true;
                        }
                        Keycode::H => {
                            self.events[GMEventCode::KeyHUp as usize] = true;
                        }
                        _ => {
                            // Unsupported keycode for now                            
                        }
                    }
                }
                _ => {
                    // Unsupported event for now
                }
            }
        }
    }
}
