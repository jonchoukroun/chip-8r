use sdl2::{
    Sdl,
    EventPump,
    event::Event,
    keyboard::{Keycode, Scancode}
};

use crate::GameState;

pub struct Keyboard {
    event_pump: EventPump,
}

impl Keyboard {
    pub fn new(sdl_context: &Sdl) -> Result<Keyboard, String> {
        return Ok(Keyboard { event_pump: sdl_context.event_pump()? });
    }

    pub fn handle_input(&mut self) -> GameState {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { return GameState::Ended; },
                Event::KeyDown { keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape => {
                            return GameState::Ended;
                        },
                        _ => {}
                    }
                },
                _ => { return GameState::Playing; }
            }
        }

        return GameState::Playing;
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        let code = match Scancode::from_i32(key as i32) {
            Some(code) => code,
            None => { return false; },
        };
        return self.event_pump.keyboard_state()
            .is_scancode_pressed(code);
    }
}