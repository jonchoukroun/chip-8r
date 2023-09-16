extern crate sdl2;

use std::{
    thread,
    time::{Duration, Instant}
};

use constants::{MS_PER_FRAME, MS_PER_CYCLE};

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
    let mut fps_timer = Instant::now();
    while running {
        let cycle_timer = Instant::now();

        running = keyboard.handle_input();

        match cpu.cycle() {
            Ok(()) => (),
            Err(error) => {
                dbg!(error);
                running = false;
            },

        }

        if fps_timer.elapsed().as_millis() as f32 >= MS_PER_FRAME {
            display.render(cpu.frame_buffer());

            // TODO: sound timer

            // TODO: delay timer

            fps_timer = Instant::now();
        }

        // TODO: audio.emit();

        let remaining = MS_PER_CYCLE - cycle_timer.elapsed().as_millis() as f32;
        thread::sleep(Duration::from_millis(remaining as u64));
    }

    Ok(())
}