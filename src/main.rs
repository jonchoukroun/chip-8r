extern crate sdl2;

use sdl2::event::Event;
use std::time::Duration;

use crate::cpu::CPU;
use crate::display::Display;

mod bus;
mod cpu;
mod display;
mod error;
mod registers;

fn main() -> Result<(), String> {
    let mut cpu = CPU::new();
    let sdl_context = sdl2::init()?;
    let mut display = match Display::new(&sdl_context) {
        Ok(display) => display,
        Err(e) => { return Err(e); }
    };
    // let keyboard = Keyboard::new(&sdl_context);
    // let audio = Audio::new(&sdl_context);

    let mut event_pump = sdl_context.event_pump()?;

    let mut running = true;
    while running {
        // TODO: keyboard.handle_input();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { running = false; },
                _ => {},
            }

        }
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