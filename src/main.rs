extern crate sdl2;

use std::time::Duration;

use crate::cpu::CPU;
use crate::display::Display;
use crate::keyboard::Keyboard;

mod bus;
mod constants;
mod cpu;
mod display;
mod error;
mod keyboard;
mod registers;

fn main() -> Result<(), String> {
    let mut cpu = CPU::new();
    let sdl_context = sdl2::init()?;
    let mut display = match Display::new(&sdl_context) {
        Ok(display) => display,
        Err(e) => { return Err(e); }
    };
    display.render(cpu.frame_buffer());
    let mut keyboard = match Keyboard::new(&sdl_context) {
        Ok(keyboard) => keyboard,
        Err(e) => { return Err(e); }
    };
    // let audio = Audio::new(&sdl_context);

    let mut running = true;
    while running {
        running = keyboard.handle_input();

        match cpu.cycle() {
            Ok(()) => (),
            Err(error) => {
                dbg!(error);
                running = false;
            },

        }
        display.render(cpu.frame_buffer());

        // TODO: audio.emit();

        // TODO: cpu.adjust_cycles();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}