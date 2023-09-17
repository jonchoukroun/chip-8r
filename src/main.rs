use std::{
    thread,
    time::{Duration, Instant}
};

use constants::{MS_PER_FRAME, MS_PER_CYCLE};

use crate::cpu::CPU;

mod bus;
mod constants;
mod cpu;
mod display;
mod error;
mod keyboard;
mod registers;

fn main() -> Result<(), String> {
    let mut cpu = CPU::new()?;

    let mut running = true;
    let mut fps_timer = Instant::now();
    while running {
        let cycle_timer = Instant::now();

        running = cpu.handle_input();

        match cpu.cycle() {
            Ok(()) => (),
            Err(error) => {
                dbg!(error);
                running = false;
            },

        }

        if fps_timer.elapsed().as_millis() as f32 >= MS_PER_FRAME {
            cpu.render();

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