use sdl2::{Sdl, EventPump, event::Event, keyboard::Keycode};

use crate::constants;

pub struct Keyboard {
    event_pump: EventPump,
    state: [bool; constants::KEYBOARD_SIZE],
}

impl Keyboard {
    pub fn new(sdl_context: &Sdl) -> Result<Keyboard, String> {
        return Ok(Keyboard {
            event_pump: sdl_context.event_pump()?,
            state: [false; constants::KEYBOARD_SIZE]
        });
    }

    pub fn handle_input(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { return false; },
                Event::KeyDown { keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape => { return false; },
                        Keycode::Kp1 => self.state[0x1] = true,
                        Keycode::Kp2 => self.state[0x2] = true,
                        Keycode::Kp3 => self.state[0x3] = true,
                        Keycode::Q => self.state[0x4] = true,
                        Keycode::W => self.state[0x5] = true,
                        Keycode::E => self.state[0x6] = true,
                        Keycode::A => self.state[0x7] = true,
                        Keycode::S => self.state[0x8] = true,
                        Keycode::D => self.state[0x9] = true,
                        Keycode::Z => self.state[0xa] = true,
                        Keycode::X => self.state[0x0] = true,
                        Keycode::C => self.state[0xb] = true,
                        Keycode::Kp4 => self.state[0xc] = true,
                        Keycode::R => self.state[0xd] = true,
                        Keycode::F => self.state[0xe] = true,
                        Keycode::V => self.state[0xf] = true,
                        _ => {}
                    }
                }
                _ => { return true; }
            }
        }

        return true;
    }
}