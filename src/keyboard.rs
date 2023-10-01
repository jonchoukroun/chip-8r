use sdl2::{
    Sdl,
    EventPump,
    event::Event,
    keyboard::Scancode
};

#[derive(PartialEq, Debug)]
enum KeyState {
    None,
    KeyDown(u8),
    KeyUp(u8),
}
pub struct Keyboard {
    key_state: KeyState,
    event_pump: EventPump,
}

impl Keyboard {
    pub fn new(sdl_context: &Sdl) -> Result<Keyboard, String> {
        return Ok(
            Keyboard {
                key_state: KeyState::None,
                event_pump: sdl_context.event_pump()?,
            }
        );
    }

    pub fn handle_input(&mut self) -> bool {
        match self.key_state {
            KeyState::KeyUp(_) => { self.key_state = KeyState::None; }
            _ => ()
        };

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { return false; },
                Event::KeyDown { scancode: Some(scancode), ..} => {
                    match scancode {
                        Scancode::Escape => {
                            return false;
                        },
                        s => {
                            match to_hex(s) {
                                Some(key) => {
                                    self.key_state = KeyState::KeyDown(key)
                                },
                                _ => (),
                            };
                        },
                    }
                },
                Event::KeyUp {scancode: Some(scancode), ..} => {
                    match to_hex(scancode) {
                        Some(key) if self.key_state == KeyState::KeyDown(key) => {
                            self.key_state = KeyState::KeyUp(key);
                        },
                        _ => ()
                    }
                }
                _ => ()
            }
        }
        
        return true
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        return self.key_state == KeyState::KeyDown(key);
    }

    pub fn get_keyup(&self) -> Option<u8> {
        match self.key_state {
            KeyState::KeyUp(key) => Some(key),
            _ => None
        }
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