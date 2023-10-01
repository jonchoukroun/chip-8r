use sdl2::{
    Sdl,
    EventPump,
    event::Event,
    keyboard::Scancode
};
use std::collections::HashMap;

use crate::constants::KEYBOARD_SIZE;

type KeyState = HashMap<Scancode, bool>;
pub struct Keyboard {
    pub key_state: KeyState,
    event_pump: EventPump,
}

impl Keyboard {
    pub fn new(sdl_context: &Sdl) -> Result<Keyboard, String> {
        return Ok(
            Keyboard {
                key_state: HashMap::from([
                    (Scancode::X, false),    // 0x0
                    (Scancode::Num1, false),  // 0x1
                    (Scancode::Num2, false),  // 0x2
                    (Scancode::Num3, false),  // 0x3
                    (Scancode::Q, false),    // 0x4
                    (Scancode::W, false),    // 0x5
                    (Scancode::E, false),    // 0x6
                    (Scancode::A, false),    // 0x7
                    (Scancode::S, false),    // 0x8
                    (Scancode::D, false),    // 0x9
                    (Scancode::Z, false),    // 0xa
                    (Scancode::C, false),    // 0xb
                    (Scancode::Num4, false),  // 0xc
                    (Scancode::R, false),    // 0xd
                    (Scancode::F, false),    // 0xe
                    (Scancode::V, false),    // 0xf
                ]),
                event_pump: sdl_context.event_pump()?,
            }
        );
    }

    pub fn handle_input(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { return false; },
                Event::KeyDown { scancode: Some(scancode), ..} => {
                    match scancode {
                        Scancode::Escape => {
                            return false;
                        },
                        s if self.key_state.contains_key(&s) => {
                            self.key_state.entry(s)
                                .and_modify(|v| *v = true);
                        }
                        _ => ()
                    }
                },
                Event::KeyUp {scancode: Some(scancode), ..} => {
                    match scancode {
                        s if self.key_state.contains_key(&s) => {
                            self.key_state.entry(s)
                                .and_modify(|v| *v = false);
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        }
        
        return true
    }
}

pub fn to_hex(scancode: Scancode) -> Option<u8> {
    return match scancode {
        Scancode::X => Some(0x0),
        Scancode::Num1 => Some(0x1),
        Scancode::Num2 => Some(0x2),
        Scancode::Num3 => Some(0x3),
        Scancode::Q => Some(0x4),
        Scancode::W => Some(0x5),
        Scancode::E => Some(0x6),
        Scancode::A => Some(0x7),
        Scancode::S => Some(0x8),
        Scancode::D => Some(0x9),
        Scancode::Z => Some(0xa),
        Scancode::C => Some(0xb),
        Scancode::Num4 => Some(0xc),
        Scancode::R => Some(0xd),
        Scancode::F => Some(0xe),
        Scancode::V => Some(0xf),
        _ => None,
    }
}

pub fn to_scancode(key: usize) -> Option<Scancode> {
    let scancode: [Scancode; KEYBOARD_SIZE] = [
        Scancode::X,
        Scancode::Num1,
        Scancode::Num2,
        Scancode::Num3,
        Scancode::Q,
        Scancode::W,
        Scancode::E,
        Scancode::A,
        Scancode::S,
        Scancode::D,
        Scancode::Z,
        Scancode::C,
        Scancode::Num4,
        Scancode::R,
        Scancode::F,
        Scancode::V];
    if key > scancode.len() {
        return None;
    } else {
        return Some(scancode[key]);
    }
}

pub fn is_pressed(key: u8, keyboard: &Keyboard) -> bool {
    match to_scancode(key as usize) {
        Some(scancode) => {
            return keyboard.key_state.get(&scancode) == Some(&true);
        },
        None => { return false; },
    }
}